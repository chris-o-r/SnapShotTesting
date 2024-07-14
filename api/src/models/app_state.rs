use bb8_redis::{bb8, RedisConnectionManager};
use sqlx::{Pool, Postgres};

use crate::{
    db::{connection::create_connection_pool, redis_connection::create_redis_pool},
    models::env_variables::EnvVariables,
};

#[derive(Clone)]
pub struct AppState {
    pub env_variables: EnvVariables,
    pub db_pool: Pool<Postgres>,
    pub redis_pool: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
}

impl AppState {
    pub async fn new() -> Self {
        let env_variables = EnvVariables::new();
        let db_url = env_variables.db_config.get_db_url();
        println!("DB URL: {}", db_url);
        let pool = create_connection_pool(&db_url).await;

        let db_pool: Pool<Postgres> = match pool {
            Ok(pool) => pool,
            Err(err) => panic!("Cannot connect to postgres database [{}]", err.to_string()),
        };

        let redis_pool: bb8::Pool<RedisConnectionManager> = create_redis_pool().await;

        Self {
            env_variables,
            db_pool,
            redis_pool,
        }
    }
}
