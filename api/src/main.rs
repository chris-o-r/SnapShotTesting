use lib::{api, utils::env_variables::EnvVariables};
use sqlx::{migrate::Migrator, PgPool};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

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
                let env_variables = EnvVariables::new();
                let db_url = env_variables.db_config.get_db_url();
                let pool = PgPool::connect(&db_url).await.unwrap();
                tracing::info!(db_url);

                // Run migrations
                match MIGRATOR.run(&pool).await {
                    Ok(()) => {
                        tracing::info!("Success")
                    }
                    Err(err) => {
                        tracing::error!("Error occured while running migrate:up {}", err);
                        panic!("{}", err);
                    }
                };
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
