use std::{env, time::Duration};

use crate::models::{raw_image::RawImage, snapshot::SnapShotType};

use super::env_variables;
use anyhow::Error;
use fantoccini::{Client, ClientBuilder};
use futures_util::{future::join_all, stream::FuturesUnordered};
use lazy_static::lazy_static;



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
    pub image_type: SnapShotType,
}

pub async fn capture_screenshots(
    urls: &Vec<ScreenShotParams>,
) -> Result<Vec<Result<RawImage, Error>>, Error> {
    let handles = FuturesUnordered::new();
    let mut raw_images: Vec<Result<RawImage, Error>> = vec![];

    let chunk_size = urls.len() / *SELENIUM_MAX_INSTANCES;

    for chunk in urls.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let future: tokio::task::JoinHandle<Result<Vec<Result<RawImage, Error>>, Error>> =
            tokio::spawn(take_screenshots(chunk));
        handles.push(future);
    }

    for handle in join_all(handles).await {
        let result = handle?;

        raw_images.extend(result?);
    }

    Ok(raw_images)
}

async fn take_screenshots(
    params: Vec<ScreenShotParams>,
) -> Result<Vec<Result<RawImage, Error>>, Error> {
    let mut raw_images: Vec<Result<RawImage, Error>> = vec![];
    tracing::debug!("Connecting to selenium");
    let client: Client = connect().await.map_err(|err| {
        tracing::error!("Unable to connect to selenium{}", err.to_string());
        err
    })?;

    for param in params.into_iter() {
        let screen_shot = capture_screenshot_from_url(&client, param).await;

        match screen_shot {
            Ok(screen_shot) => {
                raw_images.push(Ok(screen_shot));
            }
            Err(e) => {
                tracing::error!("Error capturing screenshot: {}", e);
                raw_images.push(Err(e));
            }
        }
    }

    Ok(raw_images)
}

async fn capture_screenshot_from_url(
    client: &Client,
    param: ScreenShotParams,
) -> Result<RawImage, Error> {
    const TIME_OUT: Duration = std::time::Duration::from_secs(5);
    const INTERVAL: Duration = std::time::Duration::from_millis(500);

    client.goto(&param.url).await.map_err(|err| {
        tracing::error!(
            "Unable to go to URL {} to take screen shot\n{}",
            &param.url,
            err
        );
        err
    })?;

    client
        .wait()
        .at_most(TIME_OUT)
        .every(INTERVAL)
        .for_element(fantoccini::Locator::XPath("/html/body/div[5]/*"))
        .await
        .map_err(|err| {
            tracing::error!("Unable to find component\n{}", err.to_string());
            err
        })?;

    let element = client
        .find(fantoccini::Locator::XPath("/html"))
        .await
        .unwrap();

    let dimensions = element.rectangle().await?;
    let screenshot = element.screenshot().await?;

    tracing::debug!("Captured sceen shot for {}", &param.url);

    Ok(RawImage {
        raw_image: screenshot,
        width: dimensions.2,
        height: dimensions.3, 
        image_name: param.id,
        image_type: param.image_type,
    })
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

    let c =ClientBuilder::native()
        .capabilities(caps)
        .connect(&SELENIUM_URL)
        .await?;

    Ok(c)

}
