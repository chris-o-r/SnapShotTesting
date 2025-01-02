use sqlx::{Pool, Postgres};

use crate::{db::connection::create_connection_pool, utils::env_variables::EnvVariables};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> Self {
        let env_variables = EnvVariables::new();
        let db_url = env_variables.db_url;
        let pool = create_connection_pool(&db_url).await;

        let db_pool: Pool<Postgres> = match pool {
            Ok(pool) => pool,
            Err(err) => panic!("Cannot connect to postgres database [{}]", err.to_string()),
        };

        Self { db_pool }
    }
}
