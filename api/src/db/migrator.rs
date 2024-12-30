use sqlx::{migrate::Migrator, PgPool};

use crate::utils::env_variables::EnvVariables;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");



pub async  fn up() {
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