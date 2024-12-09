
use crate::models::snapshot::SnapShotType;

use super::{capture_screenshots::RawImage, env_variables, save_images::safe_save_image};
use futures_util::{future::join_all, stream::FuturesUnordered};
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use std::{fs, io::Cursor};
use tokio::task::{self};
use utoipa::ToSchema;

const DIFF_RATIO_THRESHOLD: f64 = 0.0001;

static RATE: f32 = 100.0 / 256.0;

static NUM_THREADS: usize = 6;


#[derive(Debug, PartialEq)]
struct CategorizedImages {
    created_images_paths: Vec<RawImage>,
    deleted_images_paths: Vec<RawImage>,
    diff_images_paths: Vec<(RawImage, RawImage)>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CompareImagesReturn {
    pub created_images_paths: Vec<String>,
    pub deleted_images_paths: Vec<String>,
    pub diff_images_paths: Vec<String>,
}

pub async fn compare_images(
    image_paths_1: Vec<RawImage>,
    image_paths_2: Vec<RawImage>,
    random_folder_name: &str,
) -> Result<CompareImagesReturn, anyhow::Error> {
    let env_variables = env_variables::EnvVariables::new();

    let handles = FuturesUnordered::new();

    let categorized_images = categorize_images(image_paths_1, image_paths_2);

    create_folders(format!("{}/{}", env_variables.assets_folder, random_folder_name).as_str())?;


    for chunk in categorized_images.diff_images_paths.chunks(categorized_images.diff_images_paths.len() / NUM_THREADS) {
        let chunk: Vec<(RawImage, RawImage)> = chunk.to_vec();
        handles.push(task::spawn(compare_image_chunk(chunk)));
    }



    let result: CompareImagesReturn = CompareImagesReturn {
        created_images_paths: categorized_images.created_images_paths
            .iter()
            .map(|raw| 
                safe_save_image(
                    raw.raw_image.clone(), 
                    format!("{}/created", random_folder_name).as_str(), 
                    &raw.image_name)
                    .unwrap())
                    .collect(),
        deleted_images_paths: categorized_images.deleted_images_paths
        .iter()
        .map(|raw| 
            safe_save_image(
                raw.raw_image.clone(), 
                format!("{}/deleted", random_folder_name).as_str(), 
                &raw.image_name)
                .unwrap())
                .collect(),
        diff_images_paths: join_all(handles.into_iter())
            .await
            .into_iter()
            .map(|handle| handle.unwrap())
            .flat_map(|arr| arr.unwrap())
            .map(|raw| 
                safe_save_image(raw.raw_image, 
                    format!("{}/diff", random_folder_name).as_str(), 
                    &raw.image_name)
                    .unwrap())
                    .collect(),
    };
    
    Ok(result)
}

async fn compare_image_chunk(
    chunk: Vec<(RawImage, RawImage)>) -> Result<Vec<RawImage>, anyhow::Error> {
    let result = chunk.into_iter().map(|(raw_image_1, raw_image_2)| {
        let image_result: Result<Option<RawImage>, anyhow::Error> = (|| {
         
            let mut image_1 = image::load_from_memory(&raw_image_1.raw_image)
                .map_err(|_| anyhow::Error::msg(format!("Failed to open image: {}", &raw_image_1.image_name)))?;
         
            let mut image_2 = image::load_from_memory(&raw_image_2.raw_image)
                .map_err(|_| anyhow::Error::msg(format!("Failed to open image: {}", &raw_image_2.image_name)))?;

            let ratio = lcs_image_diff::calculate_diff_ratio(image_1.clone(), image_2.clone());

            if ratio < DIFF_RATIO_THRESHOLD {
                return Ok(None);
            }

            let image = lcs_image_diff::compare(&mut image_1, &mut image_2, RATE).map_err(|e| {
                tracing::error!(
                    "Error comparing images \nimage one: {} \nimage two: {}",
                    raw_image_1.image_name,
                    raw_image_2.image_name
                );
                anyhow::Error::msg(e.to_string())
            })?;
            

            Ok(Some(RawImage {
                raw_image: image_to_vec_u8(image, ImageFormat::Png),
                image_name: raw_image_1.image_name,
                image_type: SnapShotType::New
            }))
        })();

        image_result
    });

    let filtered_result = result.filter_map(|img_result| match img_result {
        Ok(Some(res)) => Some(res),
        Ok(None) => None,
        Err(e) => {
            tracing::error!("Error processing image: {}", e);
            None
        }
    }).collect::<Vec<RawImage>>();

    Ok(filtered_result)

}

fn categorize_images(image_paths_1: Vec<RawImage>, image_paths_2: Vec<RawImage>) -> CategorizedImages {
    let mut created_images: Vec<RawImage> = Vec::new();
    let mut deleted_images: Vec<RawImage> = Vec::new();
    let mut diff_images: Vec<(RawImage, RawImage)> = Vec::new();

    image_paths_1.clone().into_iter().for_each(|image_1: RawImage| {
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

fn create_folders(folder_name: &str) -> Result<(), anyhow::Error> {
    fs::create_dir_all(folder_name)?;
    fs::create_dir_all(format!("{}/deleted", folder_name))?;
    fs::create_dir_all(format!("{}/created", folder_name))?;
    fs::create_dir_all(format!("{}/diff", folder_name))?;

    Ok(())
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