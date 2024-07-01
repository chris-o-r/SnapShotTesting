use std::{fs, path::Path};

use anyhow::Error;

pub fn safe_save_image(
    raw_image: Vec<u8>,
    folder: &str,
    image_name: &str,
) -> Result<String, Error> {
    let file_name = format!("assets/{}/{}.png", folder, image_name);
    let path_str = format!("assets/{}", folder);
    let path = Path::new(path_str.as_str());

    if !Path::exists(path) {
        fs::create_dir_all(path)?;
    }

    fs::write(&file_name, raw_image).map_err(|e| e)?;

    Ok(file_name)
}

pub fn safe_copy_image(from: &str, to: &str) -> Result<String, Error> {
    let folder = Path::new(to).parent().unwrap();

    fs::create_dir_all(&folder)?;

    fs::copy(&from, &to)?;

    Ok(to.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_save_image() {
        let image = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let folder = "test";
        let image_name = "test_image";

        let result = safe_save_image(image, folder, image_name);

        assert_eq!(result.is_ok(), true);

        let path = Path::new("assets/test/test_image.png");
        assert_eq!(Path::exists(path), true);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_safe_copy_image() {
        let from = "tests/test-data/image1.png";
        let to = "tests/rester-tester/image1.png";
        let result = safe_copy_image(from, to);

        assert_eq!(result.is_ok(), true);

        let path = Path::new(to);
        assert_eq!(Path::exists(path), true);

        fs::remove_file(path).unwrap();
        let path_to_delete = path.parent().unwrap();
        fs::remove_dir(path_to_delete).unwrap();
    }
}
