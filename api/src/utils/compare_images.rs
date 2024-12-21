use crate::models::{raw_image::RawImage, snapshot::SnapShotType};

use futures_util::{future::join_all, stream::FuturesUnordered};
use image::{DynamicImage, ImageFormat, Rgba};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::thread::available_parallelism;
use tokio::task::{self};
use utoipa::ToSchema;

const DIFF_RATIO_THRESHOLD: f64 = 0.0001;

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
    pub diff_images_paths: Vec<RawImage>,
}

pub async fn compare_images(
    image_paths_1: Vec<RawImage>,
    image_paths_2: Vec<RawImage>,
) -> Result<CompareImagesReturn, anyhow::Error> {
    let num_threads = available_parallelism().unwrap().get();

    let handles = FuturesUnordered::new();

    let categorized_images = categorize_images(&image_paths_1, &image_paths_2);

    for chunk in categorized_images
        .diff_images_paths
        .chunks(categorized_images.diff_images_paths.len() / num_threads)
    {
        let chunk: Vec<(RawImage, RawImage)> = chunk.to_vec();
        handles.push(task::spawn(compare_image_chunk(chunk)));
    }

    let diff_images = join_all(handles.into_iter())
        .await
        .into_iter()
        .map(|handle| handle.unwrap())
        .flat_map(|arr| arr.unwrap())
        .collect::<Vec<RawImage>>();

    Ok(CompareImagesReturn {
        created_images_paths: categorized_images.created_images_paths.clone(),
        deleted_images_paths: categorized_images.deleted_images_paths.clone(),
        diff_images_paths: diff_images,
    })
}

async fn compare_image_chunk(
    chunk: Vec<(RawImage, RawImage)>,
) -> Result<Vec<RawImage>, anyhow::Error> {
    let result = chunk.into_iter().map(|(raw_image_1, raw_image_2)| {
        let image_result: Result<Option<RawImage>, anyhow::Error> = (|| {
            let image_1 = image::load_from_memory(&raw_image_1.raw_image).map_err(|_| {
                anyhow::Error::msg(format!("Failed to open image: {}", &raw_image_1.image_name))
            })?;

            let image_2 = image::load_from_memory(&raw_image_2.raw_image).map_err(|_| {
                anyhow::Error::msg(format!("Failed to open image: {}", &raw_image_2.image_name))
            })?;

            let ratio = diff_img::calculate_diff_ratio(image_1.clone(), image_2.clone());

            if ratio < DIFF_RATIO_THRESHOLD {
                return Ok(None);
            }

            let image = diff_img::mark_diff_with_color(image_1, image_2, Rgba([0, 255, 0, 0]))
                .map_err(|e| {
                    tracing::error!(
                        "Error comparing images \nimage one: {} \nimage two: {}",
                        raw_image_1.image_name,
                        raw_image_2.image_name
                    );
                    anyhow::Error::msg(e.to_string())
                })?;

            Ok(Some(RawImage {
                raw_image: image_to_vec_u8(image.clone(), ImageFormat::Png),
                image_name: raw_image_1.image_name,
                image_type: SnapShotType::New,
                height: image.height() as f64,
                width: image.width() as f64,
            }))
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
        .collect::<Vec<RawImage>>();

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
                    diff_images.push((image_1.clone(), image_2.clone()));
                }
                None => {
                    created_images.push(image_1.clone());
                }
            };
        });

    image_paths_2.into_iter().for_each(|image_2| {
        let image_2_in_result = image_paths_1
            .iter()
            .find(|r| r.image_name == image_2.image_name);

        if image_2_in_result.is_none() {
            deleted_images.push(image_2.clone());
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

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_images() {
        let image_paths_1 = vec![
            "path/to/image1.jpg".to_string(),
            "path/to/image2.png".to_string(),
            "path/to/image3.gif".to_string(),
        ];
        let image_paths_2: Vec<String> = vec![
            "otherpath/image1.jpg".to_string(),
            "otherpath/image2.png".to_string(),
            "otherpath/image4.bmp".to_string(),
        ];

        let expected_result = CategorizedImages {
            created_images_paths: vec!["path/to/image3.gif".to_string()],
            deleted_images_paths: vec!["otherpath/image4.bmp".to_string()],
            diff_images_paths: vec![
                (
                    "path/to/image1.jpg".to_string(),
                    "otherpath/image1.jpg".to_string(),
                ),
                (
                    "path/to/image2.png".to_string(),
                    "otherpath/image2.png".to_string(),
                ),
            ],
        };

        let result = categorize_images(image_paths_1, image_paths_2);

        assert_eq!(result, expected_result);
    }
}
 */
