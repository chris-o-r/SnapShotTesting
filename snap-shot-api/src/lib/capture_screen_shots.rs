use anyhow::{Error, Ok};
use futures_util::future::join_all;
use headless_chrome::Tab;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser};
use std::sync::Arc;
use tokio::task::{self};

use crate::save_images::safe_save_image;

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
    let mut results: Vec<String> = vec![];

    let browser = Arc::new(Browser::default().unwrap());

    let mut handles = vec![];

    let mut max_threads = urls.len() / num_cpus::get();

    if max_threads == 0 {
        max_threads = 1;
    }

    for chunk in urls.chunks(max_threads) {
        let browser = browser.clone();
        let chunk = chunk.to_vec();
        let random_folder_name = random_folder_name.to_string();
        handles.push(task::spawn(async move {
            let mut results: Vec<String> = vec![];

            let tab = browser
                .new_tab()
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;

            for url in chunk {
                let screen_shot = get_screen_shot(&tab, &url.url);
                let folder_name = format!("{}/{}", random_folder_name, &url.folder);
                let result = safe_save_image(screen_shot, &folder_name, &url.id)?;

                results.push(result);
            }

            Ok(results)
        }));
    }

    tracing::debug!("Threads started, length: {}", handles.len());

    for handle in join_all(handles).await {
        let result = handle?;

        results.extend(result?);
    }

    Ok(results)
}

fn get_screen_shot(tab: &Arc<Tab>, url: &str) -> Vec<u8> {
    tab.set_default_timeout(std::time::Duration::from_secs(60))
        .navigate_to(url)
        .unwrap()
        .wait_until_navigated()
        .unwrap()
        .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(75), None, true)
        .unwrap()
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