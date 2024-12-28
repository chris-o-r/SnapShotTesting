use crate::models::{raw_image::RawImage, snapshot::SnapShotType};

use futures_util::{future::join_all, stream::FuturesUnordered};
use image::{DynamicImage, ImageFormat, Rgba};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::thread::available_parallelism;
use tokio::task::{self};
use utoipa::ToSchema;

const DIFF_RATIO_THRESHOLD: f64 = 0.0001;
static RATE: f32 = 100.0 / 256.0;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CategorizedImages {
    created_images_paths: Vec<RawImage>,
    deleted_images_paths: Vec<RawImage>,
    diff_images_paths: Vec<(RawImage, RawImage)>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CompareImagesReturn {
    pub created_images_paths: Vec<RawImage>,
    pub deleted_images_paths: Vec<RawImage>,
    pub diff_images_paths: Vec<(RawImage, RawImage)>,
}

pub async fn compare_images(
    image_paths_1: Vec<RawImage>,
    image_paths_2: Vec<RawImage>,
) -> Result<CompareImagesReturn, anyhow::Error> {
    let num_threads = available_parallelism().unwrap().get();

    let handles = FuturesUnordered::new();

    let categorized_images = categorize_images(&image_paths_1, &image_paths_2);

    if categorized_images.diff_images_paths.is_empty() {
        return Ok(CompareImagesReturn {
            created_images_paths: categorized_images.created_images_paths.clone(),
            deleted_images_paths: categorized_images.deleted_images_paths.clone(),
            diff_images_paths: vec![],
        });
    }

    let mut chunk_size = 1;
    if categorized_images.diff_images_paths.len() > num_threads {
        chunk_size = categorized_images.diff_images_paths.len() / num_threads;
    }

    for chunk in categorized_images.diff_images_paths.chunks(chunk_size) {
        let chunk: Vec<(RawImage, RawImage)> = chunk.to_vec();
        handles.push(task::spawn(compare_image_chunk(chunk)));
    }

    let diff_images = join_all(handles.into_iter())
        .await
        .into_iter()
        .map(|handle| handle.unwrap())
        .flat_map(|arr| arr.unwrap())
        .collect::<Vec<(RawImage, RawImage)>>();

    Ok(CompareImagesReturn {
        created_images_paths: categorized_images.created_images_paths.clone(),
        deleted_images_paths: categorized_images.deleted_images_paths.clone(),
        diff_images_paths: diff_images,
    })
}

async fn compare_image_chunk(
    chunk: Vec<(RawImage, RawImage)>,
) -> Result<Vec<(RawImage, RawImage)>, anyhow::Error> {
    let result = chunk.into_iter().map(|(raw_image_1, raw_image_2)| {
        let image_result: Result<Option<(RawImage, RawImage)>, anyhow::Error> = (|| {
            let mut image_1 = image::load_from_memory(&raw_image_1.raw_image).map_err(|_| {
                anyhow::Error::msg(format!("Failed to open image: {}", &raw_image_1.image_name))
            })?;

            let mut image_2 = image::load_from_memory(&raw_image_2.raw_image).map_err(|_| {
                anyhow::Error::msg(format!("Failed to open image: {}", &raw_image_2.image_name))
            })?;

            let ratio = diff_img::calculate_diff_ratio(image_1.clone(), image_2.clone());

            if ratio < DIFF_RATIO_THRESHOLD {
                return Ok(None);
            }

            if image_1.width() != image_2.width() || image_1.height() != image_2.height() {
                return Ok(None);
            }

            let color_diff = diff_img::highlight_changes_with_color(
                image_1.clone(),
                image_2.clone(),
                Rgba([0, 255, 0, 0]),
            )
            .map_err(|e| {
                tracing::error!(
                    "Error comparing images \nimage one: {} \nimage two: {}",
                    raw_image_1.image_name,
                    raw_image_2.image_name
                );
                anyhow::Error::msg(e.to_string())
            })?;

            let lcs_diff = diff_img::lcs_diff(&mut image_1, &mut image_2, RATE).map_err(|e| {
                tracing::error!(
                    "Error comparing images \nimage one: {} \nimage two: {}",
                    raw_image_1.image_name,
                    raw_image_2.image_name
                );
                anyhow::Error::msg(e.to_string())
            })?;

            Ok(Some((
                RawImage {
                    raw_image: image_to_vec_u8(color_diff.clone(), ImageFormat::Png),
                    image_name: raw_image_1.image_name.clone(),
                    image_type: SnapShotType::ColorDiff,
                    height: color_diff.height() as f64,
                    width: color_diff.width() as f64,
                },
                RawImage {
                    raw_image: image_to_vec_u8(lcs_diff.clone(), ImageFormat::Png),
                    image_name: raw_image_1.image_name,
                    image_type: SnapShotType::LcsDiff,
                    height: lcs_diff.height() as f64,
                    width: lcs_diff.width() as f64,
                },
            )))
        })();

        image_result
    });

    let filtered_result = result
        .filter_map(|img_result| match img_result {
            Ok(Some(res)) => Some(res),
            Ok(None) => None,
            Err(e) => {
                tracing::error!("Error processing image: {}", e);
                None
            }
        })
        .collect::<Vec<(RawImage, RawImage)>>();

    Ok(filtered_result)
}

fn categorize_images(
    image_paths_1: &Vec<RawImage>,
    image_paths_2: &Vec<RawImage>,
) -> CategorizedImages {
    let mut created_images: Vec<RawImage> = Vec::new();
    let mut deleted_images: Vec<RawImage> = Vec::new();
    let mut diff_images: Vec<(RawImage, RawImage)> = Vec::new();

    image_paths_1
        .clone()
        .into_iter()
        .for_each(|image_1: RawImage| {
            let image_2 = image_paths_2
                .iter()
                .find(|&r| r.image_name == image_1.image_name);

            match image_2 {
                Some(image_2) => {
                    let mut image_1 = image_1.clone();
                    image_1.image_type = SnapShotType::Old;

                    let mut image_2 = image_2.clone();
                    image_2.image_type = SnapShotType::New;
                    diff_images.push((image_1.clone(), image_2.clone()));
                }
                None => {
                    let mut image_1 = image_1.clone();
                    image_1.image_type = SnapShotType::Create;
                    created_images.push(image_1.clone());
                }
            };
        });

    image_paths_2.into_iter().for_each(|image_2| {
        let image_2_in_result = image_paths_1
            .iter()
            .find(|r| r.image_name == image_2.image_name);

        if image_2_in_result.is_none() {
            let mut deleted_image = image_2.clone();
            deleted_image.image_type = SnapShotType::Deleted;
            deleted_images.push(deleted_image);
        }
    });

    CategorizedImages {
        created_images_paths: created_images,
        deleted_images_paths: deleted_images,
        diff_images_paths: diff_images,
    }
}

fn image_to_vec_u8(image: DynamicImage, format: ImageFormat) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, format).unwrap();
    buffer.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_compare_images_diff() {
        let image_1 = image::open("tests/images/image1.png").unwrap();
        let image_2 = image::open("tests/images/image2.png").unwrap();

        let images_1 = vec![RawImage {
            raw_image: image_to_vec_u8(image_1, ImageFormat::Png),
            image_name: "image1.png".to_string(),
            image_type: SnapShotType::Old,
            height: 0.0,
            width: 0.0,
        }];

        let images_2 = vec![RawImage {
            raw_image: image_to_vec_u8(image_2, ImageFormat::Png),
            image_name: "image1.png".to_string(),
            image_type: SnapShotType::New,
            height: 0.0,
            width: 0.0,
        }];

        let res = compare_images(images_1, images_2).await.unwrap();

        assert_eq!(res.created_images_paths.len(), 0);
        assert_eq!(res.deleted_images_paths.len(), 0);
        assert_eq!(res.diff_images_paths.len(), 1);
    }

    #[tokio::test]
    async fn test_compare_images_no_diff() {
        let image_1 = image::open("tests/images/image1.png").unwrap();
        let image_2 = image::open("tests/images/image2.png").unwrap();

        let images_1 = vec![RawImage {
            raw_image: image_to_vec_u8(image_1, ImageFormat::Png),
            image_name: "image1.png".to_string(),
            image_type: SnapShotType::Old,
            height: 0.0,
            width: 0.0,
        }];

        let images_2 = vec![RawImage {
            raw_image: image_to_vec_u8(image_2, ImageFormat::Png),
            image_name: "image2.png".to_string(),
            image_type: SnapShotType::New,
            height: 0.0,
            width: 0.0,
        }];

        let res = compare_images(images_1, images_2).await.unwrap();

        assert_eq!(res.created_images_paths.len(), 1);
        assert_eq!(res.deleted_images_paths.len(), 1);
        assert_eq!(res.diff_images_paths.len(), 0);
    }

    #[test]
    fn test_categorize_images() {
        let image_1 = vec![
            RawImage {
                raw_image: vec![],
                image_name: "image1.jpg".to_string(),
                image_type: SnapShotType::Old,
                height: 0.0,
                width: 0.0,
            },
            RawImage {
                raw_image: vec![],
                image_name: "image2.png".to_string(),
                image_type: SnapShotType::Old,
                height: 0.0,
                width: 0.0,
            },
            RawImage {
                raw_image: vec![],
                image_name: "image3.gif".to_string(),
                image_type: SnapShotType::Old,
                height: 0.0,
                width: 0.0,
            },
        ];
        let images_2: Vec<RawImage> = vec![
            RawImage {
                raw_image: vec![],
                image_name: "image1.jpg".to_string(),
                image_type: SnapShotType::New,
                height: 0.0,
                width: 0.0,
            },
            RawImage {
                raw_image: vec![],
                image_name: "image2.png".to_string(),
                image_type: SnapShotType::New,
                height: 0.0,
                width: 0.0,
            },
            RawImage {
                raw_image: vec![],
                image_name: "otherpath/image4.bmp".to_string(),
                image_type: SnapShotType::New,
                height: 0.0,
                width: 0.0,
            },
        ];

        let expected_result = CategorizedImages {
            created_images_paths: vec![RawImage {
                raw_image: vec![],
                image_name: "image3.gif".to_string(),
                image_type: SnapShotType::Create,
                height: 0.0,
                width: 0.0,
            }],
            deleted_images_paths: vec![RawImage {
                raw_image: vec![],
                image_name: "otherpath/image4.bmp".to_string(),
                image_type: SnapShotType::Deleted,
                height: 0.0,
                width: 0.0,
            }],
            diff_images_paths: vec![
                (
                    RawImage {
                        raw_image: vec![],
                        image_name: "image1.jpg".to_string(),
                        image_type: SnapShotType::Old,
                        height: 0.0,
                        width: 0.0,
                    },
                    RawImage {
                        raw_image: vec![],
                        image_name: "image1.jpg".to_string(),
                        image_type: SnapShotType::New,
                        height: 0.0,
                        width: 0.0,
                    },
                ),
                (
                    RawImage {
                        raw_image: vec![],
                        image_name: "image2.png".to_string(),
                        image_type: SnapShotType::Old,
                        height: 0.0,
                        width: 0.0,
                    },
                    RawImage {
                        raw_image: vec![],
                        image_name: "image2.png".to_string(),
                        image_type: SnapShotType::New,
                        height: 0.0,
                        width: 0.0,
                    },
                ),
            ],
        };

        let result = categorize_images(&image_1, &images_2);

        assert_eq!(result, expected_result);
    }
}
