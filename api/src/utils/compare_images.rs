use super::env_variables;
use futures_util::{future::join_all, stream::FuturesUnordered};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::task::{self};
use utoipa::ToSchema;

const DIFF_RATIO_THRESHOLD: f64 = 0.0001;

static RATE: f32 = 100.0 / 256.0;


#[derive(Debug, PartialEq)]
struct CategorizedImages {
    created_images_paths: Vec<String>,
    deleted_images_paths: Vec<String>,
    diff_images_paths: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CompareImagesReturn {
    pub created_images_paths: Vec<String>,
    pub deleted_images_paths: Vec<String>,
    pub diff_images_paths: Vec<String>,
}

pub async fn compare_images(
    image_paths_1: Vec<String>,
    image_paths_2: Vec<String>,
    random_folder_name: &str,
) -> Result<CompareImagesReturn, anyhow::Error> {
    let env_variables = env_variables::EnvVariables::new();

    let handles = FuturesUnordered::new();

    let categorized_images = categorize_images(image_paths_1, image_paths_2);

    let random_folder_name = format!("{}/{}", env_variables.assets_folder, random_folder_name);

    create_folders(random_folder_name.as_str())?;


    for chunk in categorized_images.diff_images_paths.chunks(categorized_images.diff_images_paths.len() / 1) {
        let chunk: Vec<(String, String)> = chunk.to_vec();
        let random_folder_name = random_folder_name.clone(); // clone folder name for async block
        handles.push(task::spawn(compare_image_chunk(chunk, random_folder_name)));
    }

    let result: CompareImagesReturn = CompareImagesReturn {
        created_images_paths: categorized_images.created_images_paths,
        deleted_images_paths: categorized_images.deleted_images_paths,
        diff_images_paths: join_all(handles.into_iter())
            .await
            .into_iter()
            .map(|handle| handle.unwrap())
            .flat_map(|arr| arr.unwrap())
            .collect(),
    };
    
    Ok(result)
}

async fn compare_image_chunk(
    chunk: Vec<(String, String)>,
    random_folder_name: String,
) -> Result<Vec<String>, anyhow::Error> {
    let result = chunk.into_iter().map(|(image_1_path, image_2_path)| {
        let image_result: Result<Option<String>, anyhow::Error> = (|| {
            let mut image_1 = image::open(&image_1_path)
                .map_err(|_| anyhow::Error::msg(format!("Failed to open image: {}", image_1_path)))?;
            let mut image_2 = image::open(&image_2_path)
                .map_err(|_| anyhow::Error::msg(format!("Failed to open image: {}", image_2_path)))?;

            let ratio = lcs_image_diff::calculate_diff_ratio(image_1.clone(), image_2.clone());

            if ratio < DIFF_RATIO_THRESHOLD {
                return Ok(None);
            }

            let file_name = format!(
                "{}/diff/{}",
                &random_folder_name,
                image_1_path.split('/').last().unwrap()
            );

            let image = lcs_image_diff::compare(&mut image_1, &mut image_2, RATE).map_err(|e| {
                tracing::error!(
                    "Error comparing images \nimage one: {} \nimage two: {}",
                    image_1_path,
                    image_2_path
                );
                anyhow::Error::msg(e.to_string())
            })?;

            image.save(&file_name).map_err(|_| {
                tracing::error!("Unable to save image");
                anyhow::Error::msg("Unable to save image")
            })?;

            Ok(Some(file_name))
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
    }).collect::<Vec<String>>();

    Ok(filtered_result)

}

fn categorize_images(image_paths_1: Vec<String>, image_paths_2: Vec<String>) -> CategorizedImages {
    let mut created_images: Vec<String> = Vec::new();
    let mut deleted_images: Vec<String> = Vec::new();
    let mut diff_images: Vec<(String, String)> = Vec::new();

    image_paths_1.clone().into_iter().for_each(|image_1| {
        let image_2 = image_paths_2
            .iter()
            .find(|&r| r.split('/').last() == image_1.split('/').last());

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
            .find(|r| r.as_str().split('/').last() == image_2.as_str().split('/').last());

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

#[cfg(test)]
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
