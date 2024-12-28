use lib::{api, db};

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "save-doc" => match api::swagger_config::save_doc_to_file() {
                Ok(_) => {
                    tracing::info!("Doc saved to file");
                }
                Err(e) => {
                    tracing::error!("Failed to save doc to file: {}", e);
                    std::process::exit(1);
                }
            },
            "migrate:up" => {
                db::migrator::up().await;
            }
            _ => {
                tracing::error!("Invalid argument. Exiting...");
                std::process::exit(1);
            }
        }
    } else {
        api::serve().await;
    }
}
