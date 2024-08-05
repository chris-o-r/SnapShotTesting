use std::{env, time::Duration};

use super::save_images::safe_save_image;
use anyhow::{Error, Ok};
use fantoccini::{Client, ClientBuilder};
use futures_util::{future::join_all, stream::FuturesUnordered};
use lazy_static::lazy_static;
struct RawImage {
    raw_image: Vec<u8>,
    folder: String,
    image_name: String,
}

lazy_static! {
    static ref SELENIUM_PORT: String = env::var("SELENIUM_PORT").unwrap();
    static ref SELENIUM_HOST: String = env::var("SELENIUM_HOST").unwrap();
    static ref SELENIUM_URL: String = format!(
        "http://{}:{}",
        SELENIUM_HOST.as_str(),
        SELENIUM_PORT.as_str()
    );
}

#[derive(Clone)]
pub struct ScreenShotParams {
    pub url: String,
    pub id: String,
    pub folder: String,
}

pub async fn capture_screenshots(
    urls: &Vec<ScreenShotParams>,
    random_folder_name: &str,
) -> Result<Vec<String>, Error> {
    let mut raw_images: Vec<RawImage> = vec![];
    let handles = FuturesUnordered::new();
    let mut results: Vec<String> = vec![];

    let max_threads = (urls.len() / 4) + 1;

    for chunk in urls.chunks(max_threads) {
        let chunk = chunk.to_vec();
        let random_folder_name = random_folder_name.to_string();

        let future = tokio::spawn(take_screenshots(chunk, random_folder_name));
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

async fn take_screenshots(
    params: Vec<ScreenShotParams>,
    random_folder_name: String,
) -> Result<Vec<RawImage>, Error> {
    let mut raw_images: Vec<RawImage> = vec![];
    let client: Client = connect().await?;

    for url in params.into_iter() {
        let folder_name = format!("{}/{}", random_folder_name, &url.folder);

        let screen_shot = capture_screenshot_from_url(&client, &url.url).await;

        if screen_shot.is_err() {
            tracing::error!("Failed to capture screenshot from url: {}", url.url);
        } else {
            tracing::info!("Captured screenshot from url: {}", url.url);
            raw_images.push(RawImage {
                raw_image: screen_shot.unwrap(),
                folder: folder_name.to_string(),
                image_name: url.id.to_string(),
            });
        }
    }

    client.close().await?;

    Ok(raw_images)
}

async fn capture_screenshot_from_url(client: &Client, url: &str) -> Result<Vec<u8>, Error> {
    const TIME_OUT: Duration = std::time::Duration::from_secs(5);
    const INTERVAL: Duration = std::time::Duration::from_millis(500);

    client.goto(url).await?;

    client
        .wait()
        .at_most(TIME_OUT)
        .every(INTERVAL)
        .for_element(fantoccini::Locator::XPath("/html/body/div[5]/*"))
        .await?;

    let screenshot = client
        .find(fantoccini::Locator::XPath("/html"))
        .await
        .unwrap()
        .screenshot()
        .await?;

    Ok(screenshot)
}

async fn connect() -> Result<Client, Error> {
    let mut caps: serde_json::Map<String, serde_json::Value> = serde_json::map::Map::new();
    let args = serde_json::json!([
        "--headless",
        "--disable-gpu",
        "--no-sandbox",
        "--disable-dev-shm-usage"
    ]);
    let opts = serde_json::json!({
        "args": args,
    });

    caps.insert("goog:chromeOptions".to_string(), opts.clone());

    let c: Client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&SELENIUM_URL)
        .await?;

    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capture_screen_shots_pass() {
        let urls = vec![
            ScreenShotParams {
                url: "https://www.example.com".to_string(),
                folder: "folder1".to_string(),
                id: "moo".to_string(),
            },
            ScreenShotParams {
                url: "https://www.example.org".to_string(),
                folder: "folder2".to_string(),
                id: "baa".to_string(),
            },
        ];
        let random_folder_name: String = "random_folder".to_string();

        let result = capture_screenshots(&urls, &random_folder_name).await;

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "image1.png");
        assert_eq!(results[1], "image2.png");
    }

    #[tokio::test]
    async fn test_capture_screen_shots_fail() {
        let urls = vec![
            ScreenShotParams {
                url: "https://www.example.com".to_string(),
                folder: "folder1".to_string(),
                id: "moo".to_string(),
            },
            ScreenShotParams {
                url: "https://www.example.org".to_string(),
                folder: "folder2".to_string(),
                id: "baa".to_string(),
            },
        ];
        let random_folder_name = "random_folder".to_string();

        let result = capture_screenshots(&urls, &random_folder_name).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_take_screen_shots_pass() {
        let params = vec![
            ScreenShotParams {
                url: "https://www.example.com".to_string(),
                folder: "folder1".to_string(),
                id: "moo".to_string(),
            },
            ScreenShotParams {
                url: "https://www.example.org".to_string(),
                folder: "folder2".to_string(),
                id: "baa".to_string(),
            },
        ];
        let random_folder_name = "random_folder".to_string();

        let result = take_screenshots(params, random_folder_name).await;

        assert!(result.is_ok());
        let raw_images = result.unwrap();
        assert_eq!(raw_images.len(), 2);
        assert_eq!(raw_images[0].image_name, "1");
        assert_eq!(raw_images[1].image_name, "2");
    }

    #[tokio::test]
    async fn test_take_screen_shots_fail() {
        let params = vec![
            ScreenShotParams {
                url: "https://www.example.com".to_string(),
                folder: "folder1".to_string(),
                id: "moo".to_string(),
            },
            ScreenShotParams {
                url: "https://www.example.org".to_string(),
                folder: "folder2".to_string(),
                id: "baa".to_string(),
            },
        ];
        let random_folder_name = "random_folder".to_string();

        let result = take_screenshots(params, random_folder_name).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_capture_screenshot_from_url_pass() {
        let client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .unwrap();
        let url = "https://www.example.com";

        let result = capture_screenshot_from_url(&client, url).await;

        assert!(result.is_ok());
        let screen_shot = result.unwrap();
        assert!(!screen_shot.is_empty());
    }

    #[tokio::test]
    async fn test_capture_screenshot_from_url_fail() {
        let client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .unwrap();
        let url = "https://www.invalidurl.com";

        let result = capture_screenshot_from_url(&client, url).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_pass() {
        let result = connect().await;

        assert!(result.is_ok());
    }
}
