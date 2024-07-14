use sqlx::{postgres::PgPoolOptions, Postgres};

const MAX_CONNECTIONS: u32 = 25;
pub async fn create_connection_pool(url: &str) -> Result<sqlx::Pool<Postgres>, sqlx::Error> {
    match PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(url)
        .await
        .map_err(|err| {
            tracing::error!("Cannot connect to database [{}]", err.to_string());
            err
        }) {
        Ok(pool) => {
            tracing::info!("Connected to database successfully.");
            return Ok(pool);
        }
        Err(err) => {
            return Err(err);
        }
    }
}
