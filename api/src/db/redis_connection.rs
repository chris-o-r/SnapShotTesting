use std::env;

use bb8_redis::{bb8, RedisConnectionManager};

pub async fn create_redis_pool() -> bb8::Pool<RedisConnectionManager> {
    let redis_url = match env::var("REDIS_URL") {
        Ok(val) => val,
        Err(_) => panic!("REDIS_URL must be set"),
    };
    tracing::info!("Connecting to redis at [{}]", redis_url);

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool: bb8::Pool<RedisConnectionManager> = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();

    return pool;
}
