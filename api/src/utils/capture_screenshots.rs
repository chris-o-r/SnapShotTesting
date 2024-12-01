use std::{env, time::Duration};

use super::{env_variables, save_images::safe_save_image};
use anyhow::Error;
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
    static ref SELENIUM_MAX_INSTANCES: usize =
        env_variables::EnvVariables::new().selenium_max_instances;
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
) -> Result<Vec<Result<String, Error>>, Error> {
    let mut raw_images: Vec<Result<RawImage, Error>> = vec![];
    let handles = FuturesUnordered::new();
    let mut results: Vec<Result<String, Error>> = vec![];

    let chunk_size = urls.len() / *SELENIUM_MAX_INSTANCES;

    for chunk in urls.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let random_folder_name = random_folder_name.to_string();

        let future: tokio::task::JoinHandle<Result<Vec<Result<RawImage, Error>>, Error>> =
            tokio::spawn(take_screenshots(chunk, random_folder_name));
        handles.push(future);
    }

    for handle in join_all(handles).await {
        let result = handle?;

        raw_images.extend(result?);
    }

    for raw_image in raw_images {
        match raw_image {
            Ok(raw_image) => {
                let file_name = safe_save_image(
                    raw_image.raw_image,
                    &raw_image.folder,
                    &raw_image.image_name,
                )?;
                results.push(Ok(file_name));
            }
            Err(e) => results.push(Err(e)),
        }
    }

    Ok(results)
}

async fn take_screenshots(
    params: Vec<ScreenShotParams>,
    random_folder_name: String,
) -> Result<Vec<Result<RawImage, Error>>, Error> {
    let mut raw_images: Vec<Result<RawImage, Error>> = vec![];
    tracing::info!("Connecting to selenium");
    let client: Client = connect().await.map_err(|err| {
        tracing::error!("Unable to connect to selenium{}", err.to_string());
        err
    })?;

    for url in params.into_iter() {
        let folder_name = format!("{}/{}", random_folder_name, &url.folder);

        let screen_shot = capture_screenshot_from_url(&client, &url.url).await;

        match screen_shot {
            Ok(screen_shot) => {
                raw_images.push(Ok(RawImage {
                    raw_image: screen_shot,
                    folder: folder_name.to_string(),
                    image_name: url.id.to_string(),
                }));

            }
            Err(e) => {
                tracing::error!("Error capturing screenshot: {}", e);
                raw_images.push(Err(e));
            }
        }
    }

    client.close().await?;

    Ok(raw_images)
}

async fn capture_screenshot_from_url(client: &Client, url: &str) -> Result<Vec<u8>, Error> {
    const TIME_OUT: Duration = std::time::Duration::from_secs(5);
    const INTERVAL: Duration = std::time::Duration::from_millis(500);

    client.goto(url).await.map_err(|err| {
        tracing::error!("Unable to go to URL {} to take screen shot\n{}", url, err);
        err
    })?;


    client
        .wait()
        .at_most(TIME_OUT)
        .every(INTERVAL)
        .for_element(fantoccini::Locator::XPath("/html/body/div[5]/*"))
        .await.map_err(
            |err|  {
                tracing::error!("Unable to find component {}", err.to_string());
                err
            }
        )?;

    let screenshot = client
        .find(fantoccini::Locator::XPath("/html"))
        .await
        .unwrap()
        .screenshot()
        .await?;

    tracing::info!("Captured sceen shot for {}", url);

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
