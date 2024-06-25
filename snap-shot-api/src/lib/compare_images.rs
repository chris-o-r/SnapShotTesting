use crate::diff_img;
use anyhow::Error;
use futures_util::future::join_all;
use image;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::task::{self};

use crate::save_images;

const DIFF_RATIO_THRESHOLD: f64 = 0.0001;
const IMAGE_NOT_FOUND: &str = "not found";

#[derive(Debug, Deserialize, Serialize)]
pub struct CompareImagesReturn {
    pub created_images_paths: Vec<String>,
    pub deleted_images_paths: Vec<String>,
    pub diff_images_paths: Vec<String>,
}

pub async fn compare_images(
    image_paths_1: Vec<String>,
    image_paths_2: Vec<String>,
    random_folder_name: &str,
) -> Result<CompareImagesReturn, Error> {
    let mut handles: Vec<task::JoinHandle<Result<CompareImagesReturn, Error>>> = Vec::new();

    let path_pairs = get_matching_path_pairs(image_paths_1, image_paths_2);

    let random_folder_name: String = format!("assets/{}", random_folder_name);

    fs::create_dir_all(&random_folder_name)?;
    fs::create_dir_all(format!("{}/deleted", random_folder_name))?;
    fs::create_dir_all(format!("{}/created", random_folder_name))?;
    fs::create_dir_all(format!("{}/diff", random_folder_name))?;

    tracing::info!("Comparing images with {} threads", num_cpus::get());
    for chunk in path_pairs.chunks(path_pairs.len() / num_cpus::get() + 1) {
        let chunk: Vec<(String, String)> = chunk.to_vec();
        let random_folder_name = random_folder_name.clone(); // clone folder name for async block
        handles.push(task::spawn(compare_image_chunk(chunk, random_folder_name)));
    }

    let mut result: CompareImagesReturn = CompareImagesReturn {
        created_images_paths: Vec::new(),
        deleted_images_paths: Vec::new(),
        diff_images_paths: Vec::new(),
    };

    join_all(handles.into_iter())
        .await
        .into_iter()
        .for_each(|handle| {
            let paths = handle.unwrap().unwrap();
            result
                .created_images_paths
                .extend(paths.created_images_paths);
            result
                .deleted_images_paths
                .extend(paths.deleted_images_paths);
            result.diff_images_paths.extend(paths.diff_images_paths);
        });

    Ok(result)
}

async fn compare_image_chunk(
    chunk: Vec<(String, String)>,
    random_folder_name: String,
) -> Result<CompareImagesReturn, Error> {
    let mut created_images: Vec<String> = Vec::new();
    let mut deleted_images: Vec<String> = Vec::new();
    let mut diff_images: Vec<String> = Vec::new();

    for (image_1_path, image_2_path) in chunk {
        let is_image_created = image_2_path == IMAGE_NOT_FOUND && image_1_path != IMAGE_NOT_FOUND;
        let is_image_deleted = image_2_path != IMAGE_NOT_FOUND && image_1_path == IMAGE_NOT_FOUND;

        if !is_image_created && !is_image_deleted {
            match handle_compare_image(&image_1_path, &image_2_path, &random_folder_name)? {
                Some(image_path) => diff_images.push(image_path),
                None => (),
            }
        } else if is_image_created {
            let created_image_path = handle_new_image(&image_1_path, &random_folder_name)?;
            created_images.push(created_image_path);
        } else if is_image_deleted {
            let deleted_image_path = handle_deleted_image(&image_2_path, &random_folder_name)?;
            deleted_images.push(deleted_image_path);
        }
    }

    Ok(CompareImagesReturn {
        created_images_paths: created_images,
        deleted_images_paths: deleted_images,
        diff_images_paths: diff_images,
    })
}

fn get_matching_path_pairs(
    image_paths_1: Vec<String>,
    image_paths_2: Vec<String>,
) -> Vec<(String, String)> {
    image_paths_1
        .into_iter()
        .map(|image_1| {
            let image_2 = image_paths_2
                .iter()
                .find(|&r| r.split('/').last() == image_1.split('/').last());
            match image_2 {
                Some(image_2) => (image_1, image_2.clone()),
                None => (image_1, IMAGE_NOT_FOUND.to_string()),
            }
        })
        .collect()
}

fn handle_compare_image(
    image_1_path: &str,
    image_2_path: &str,
    random_folder_name: &str,
) -> Result<Option<String>, Error> {
    let image_1 = image::open(&image_1_path).unwrap();
    let image_2 = image::open(&image_2_path).unwrap();

    let ratio = diff_img::calculate_diff_ratio(image_1.clone(), image_2.clone());

    if ratio < DIFF_RATIO_THRESHOLD {
        return Ok(None);
    }

    let file_name = format!(
        "{}/diff/{}",
        random_folder_name,
        image_1_path.split('/').last().unwrap()
    );

    let image_path =
        diff_img::get_diff_from_images(image_1, image_2, &file_name, diff_img::BlendMode::HUE)
            .map_err(|e| Error::msg(e.to_string()))?;

    Ok(Some(image_path))
}

fn handle_new_image(image_path: &str, random_folder_name: &str) -> Result<String, Error> {
    let new_file_path = format!(
        "{}/created/{}",
        random_folder_name,
        image_path.split('/').last().unwrap()
    );

    save_images::safe_copy_image(&image_path, &new_file_path)
}

fn handle_deleted_image(image_path: &str, random_folder_name: &str) -> Result<String, Error> {
    let new_file_path = format!(
        "{}/deleted/{}",
        random_folder_name,
        image_path.split('/').last().unwrap()
    );

    save_images::safe_copy_image(image_path, &new_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_paths() {
        let image_paths_1 = vec![
            "path/to/image1.jpg".to_string(),
            "path/to/image2.png".to_string(),
            "path/to/image3.gif".to_string(),
        ];
        let image_paths_2 = vec![
            "otherpath/image1.jpg".to_string(),
            "otherpath/image2.png".to_string(),
            "otherpath/image4.bmp".to_string(),
        ];
        let expected_result = vec![
            (
                "path/to/image1.jpg".to_string(),
                "otherpath/image1.jpg".to_string(),
            ),
            (
                "path/to/image2.png".to_string(),
                "otherpath/image2.png".to_string(),
            ),
            (
                "path/to/image3.gif".to_string(),
                IMAGE_NOT_FOUND.to_string(),
            ),
        ];

        let result = get_matching_path_pairs(image_paths_1, image_paths_2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_no_matching_paths() {
        let image_paths_1 = vec![
            "path/to/image1.jpg".to_string(),
            "path/to/image2.png".to_string(),
        ];
        let image_paths_2 = vec![
            "otherpath/image3.jpg".to_string(),
            "otherpath/image4.png".to_string(),
        ];
        let expected_result = vec![
            (
                "path/to/image1.jpg".to_string(),
                IMAGE_NOT_FOUND.to_string(),
            ),
            (
                "path/to/image2.png".to_string(),
                IMAGE_NOT_FOUND.to_string(),
            ),
        ];

        let result = get_matching_path_pairs(image_paths_1, image_paths_2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_empty_paths() {
        let image_paths_1: Vec<String> = Vec::new();
        let image_paths_2: Vec<String> = Vec::new();
        let expected_result: Vec<(String, String)> = Vec::new();

        let result = get_matching_path_pairs(image_paths_1, image_paths_2);

        assert_eq!(result, expected_result);
    }
}
