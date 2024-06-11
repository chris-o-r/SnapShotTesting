use diffimg;
use image;
use short_uuid::short;
use std::{fs, io, path::Path};
use tokio::task;

const DIFF_RATIO_THRESHOLD: f64 = 0.0001;

pub async fn compare_images(
    image_paths_1: Vec<String>,
    image_paths_2: Vec<String>,
) -> Result<Vec<String>, io::Error> {
    let mut handles: Vec<task::JoinHandle<Result<Vec<String>, std::io::Error>>> = Vec::new();

    let path_pairs = get_matching_path_pairs(image_paths_1, image_paths_2);
    let date = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let random_folder_name: String = format!("assets/{}-{}", date, short!());

    fs::create_dir_all(&random_folder_name)?;
    fs::create_dir_all(format!("{}/deleted", random_folder_name))?;
    fs::create_dir_all(format!("{}/created", random_folder_name))?;
    fs::create_dir_all(format!("{}/diff", random_folder_name))?;

    for chunk in path_pairs.chunks(10) {
        let chunk = chunk.to_vec();
        let random_folder_name = random_folder_name.clone(); // clone folder name for async block
        handles.push(task::spawn(async move {
            let mut result: Vec<String> = Vec::new();

            for (image_1_path, image_2_path) in chunk {
                let image_created = image_2_path == "not found" && image_1_path != "not found";
                let image_deleted = image_2_path != "not found" && image_1_path == "not found";

                if !image_created && !image_deleted {
                    result.push(handle_compare_image(
                        &image_1_path,
                        &image_2_path,
                        &random_folder_name,
                    ));
                } else if image_created {
                    result.push(handle_new_image(&image_2_path, &random_folder_name));
                } else if image_deleted {
                    result.push(handle_deleted_image(&image_1_path, &random_folder_name));
                }
            }

            Ok(result)
        }));
    }

    let mut results: Vec<String> = Vec::new();
    for handle in handles {
        results.extend(handle.await??);
    }

    Ok(results)
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
                None => (image_1, "not found".to_string()),
            }
        })
        .collect()
}

fn handle_compare_image(
    image_1_path: &str,
    image_2_path: &str,
    random_folder_name: &str,
) -> String {
    let image_1 = image::open(&image_1_path).unwrap();
    let image_2 = image::open(&image_2_path).unwrap();

    let ratio = diffimg::calculate_diff_ratio(image_1.clone(), image_2.clone());

    if ratio < DIFF_RATIO_THRESHOLD {
        tracing::info!("Images are identical: {}", image_1_path);
        return "".to_string();
    }

    let file_name = format!(
        "{}/diff/{}",
        random_folder_name,
        image_1_path.split('/').last().unwrap()
    );

    diffimg::get_diff_from_images(image_1, image_2, &file_name).unwrap()
}

fn handle_new_image(image_path: &str, random_folder_name: &str) -> String {
    safe_copy_image(
        &image_path,
        format!("{}/created", random_folder_name).as_str(),
    )
    .unwrap()
}

fn handle_deleted_image(image_path: &str, random_folder_name: &str) -> String {
    safe_copy_image(
        &image_path,
        format!("{}/deleted", random_folder_name).as_str(),
    )
    .unwrap()
}

fn safe_copy_image(image_path: &str, folder: &str) -> Result<String, io::Error> {
    let file_name = image_path.split('/').last().unwrap();
    let path = Path::new(folder);

    if !Path::exists(path) {
        match fs::create_dir(folder) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    let new_file_path = format!("{}/{}", folder, file_name);
    match fs::copy(image_path, format!("{}/{}", folder, file_name)) {
        Ok(_) => (Ok(new_file_path)),
        Err(e) => return Err(e),
    }
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
            ("path/to/image3.gif".to_string(), "not found".to_string()),
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
            ("path/to/image1.jpg".to_string(), "not found".to_string()),
            ("path/to/image2.png".to_string(), "not found".to_string()),
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
