use anyhow::{Error, Ok};
use futures_util::future::join_all;
use futures_util::stream::FuturesUnordered;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser};
use headless_chrome::{LaunchOptions, Tab};
use std::sync::Arc;
use tokio::task::{self};

use crate::save_images::safe_save_image;

struct RawImage {
    raw_image: Vec<u8>,
    folder: String,
    image_name: String,
}

#[derive(Clone)]
pub struct ScreenShotParams {
    pub url: String,
    pub id: String,
    pub folder: String,
}

pub async fn capture_screen_shots(
    urls: Vec<ScreenShotParams>,
    random_folder_name: &str,
) -> Result<Vec<String>, Error> {
    let mut raw_images: Vec<RawImage> = vec![];
    let mut results: Vec<String> = vec![];

    let browser = Arc::new(Browser::new(LaunchOptions {
        headless: true,
        enable_gpu: true,
        enable_logging: false,
        ..Default::default()
    })?);

    let handles = FuturesUnordered::new();

    let max_threads = 4;

    for chunk in urls.chunks(max_threads) {
        let tab = browser
            .new_tab()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let chunk = chunk.to_vec();
        let random_folder_name = random_folder_name.to_string();

        let future = task::spawn(take_screen_shots(tab, chunk, random_folder_name));
        handles.push(future);
    }

    for handle in join_all(handles).await {
        let result = handle?;

        raw_images.extend(result?);
    }

    for raw_image in raw_images {
        results.push(safe_save_image(
            raw_image.raw_image,
            &raw_image.folder,
            &raw_image.image_name,
        )?);
    }

    Ok(results)
}

async fn take_screen_shots(
    tab: Arc<Tab>,
    screen_shot_params: Vec<ScreenShotParams>,
    random_folder_name: String,
) -> Result<Vec<RawImage>, Error> {
    let mut results: Vec<RawImage> = vec![];

    for url in screen_shot_params {
        let screen_shot = get_screen_shot(&tab, &url.url)?;
        let folder_name = format!("{}/{}", random_folder_name, &url.folder);

        results.push(RawImage {
            raw_image: screen_shot,
            folder: folder_name.to_string(),
            image_name: url.id.to_string(),
        });
    }

    Ok(results)
}

fn get_screen_shot(tab: &Arc<Tab>, url: &str) -> Result<Vec<u8>, Error> {
    tab.set_default_timeout(std::time::Duration::from_secs(60))
        .navigate_to(url)?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(75), None, true)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[tokio::test]
    async fn test_capture_screen_shots() {
        let urls = vec![
            ScreenShotParams {
                url: "https://www.google.com".to_string(),
                id: "google".to_string(),
                folder: "test".to_string(),
            },
            ScreenShotParams {
                url: "https://www.bing.com".to_string(),
                id: "bing".to_string(),
                folder: "test".to_string(),
            },
        ];

        let sd = capture_screen_shots(urls, "test").await.unwrap();

        println!("Test completed {:?}", sd);

        let path1 = Path::new("assets/test/test/google.png");
        let path2 = Path::new("assets/test/test/bing.png");
        assert_eq!(Path::exists(path1), true);
        assert_eq!(Path::exists(path2), true);

        fs::remove_file(path1).unwrap();
        fs::remove_file(path2).unwrap();
        fs::remove_dir("assets/test/test").unwrap();
    }
}
