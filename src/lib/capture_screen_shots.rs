use futures_util::future::join_all;
use headless_chrome::Tab;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::task::{self};

#[derive(Clone)]
pub struct ScreenShotParams {
    pub url: String,
    pub id: String,
    pub folder: String,
}

pub async fn capture_screen_shots(
    urls: Vec<ScreenShotParams>,
) -> Vec<Result<String, std::io::Error>> {
    let browser = Arc::new(Browser::default().unwrap());

    let mut handles: Vec<task::JoinHandle<Vec<Result<String, std::io::Error>>>> = vec![];

    let mut max_threads = urls.len() / 10;
    if max_threads == 0 {
        max_threads = 1;
    }

    for chunk in urls.chunks(max_threads) {
        let browser = browser.clone();
        let chunk = chunk.to_vec();

        handles.push(task::spawn(async move {
            let mut results = vec![];
            let tab = browser.new_tab().unwrap();
            for url in chunk {
                let screen_shot = get_screen_shot(&tab, &url.url);
                let result = safe_save_image(screen_shot, &url.folder, &url.id);

                results.push(result);
            }
            results
        }));
    }

    tracing::info!("Threads started, length: {}", handles.len());

    let results = join_all(handles).await;
    results.into_iter().flat_map(|res| res.unwrap()).collect()
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

fn safe_save_image(
    raw_image: Vec<u8>,
    folder: &str,
    image_name: &str,
) -> Result<String, std::io::Error> {
    let file_name = format!("assets/{}/{}.png", folder, image_name);
    let path_str = format!("assets/{}", folder);
    let path = Path::new(path_str.as_str());

    if !Path::exists(path) {
        match fs::create_dir(path) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    match fs::write(&file_name, raw_image) {
        Ok(_) => Ok(file_name),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
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

        let result = capture_screen_shots(urls);
        result.await.into_iter().for_each(|res| {
            assert_eq!(res.is_ok(), true);
        });

        let path1 = Path::new("assets/test/google.png");
        let path2 = Path::new("assets/test/bing.png");
        assert_eq!(Path::exists(path1), true);
        assert_eq!(Path::exists(path2), true);

        fs::remove_file(path1).unwrap();
        fs::remove_file(path2).unwrap();
        fs::remove_dir("assets/test").unwrap();
    }

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
        fs::remove_dir("assets/test").unwrap();
    }
}
