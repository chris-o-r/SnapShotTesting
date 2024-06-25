use dotenv::dotenv;
use std::env;

use super::db_config::DBConfig;

#[derive(Clone)]
pub struct EnvVariables {
    pub base_url: String,
    pub port: String,
    pub db_config: DBConfig,
}

impl EnvVariables {
    pub fn new() -> Self {
        dotenv().ok();

        let base_url = match env::var("BASE_URL") {
            Ok(val) => val,
            Err(_) => panic!("BASE_URL must be set"),
        };

        let port = match env::var("PORT") {
            Ok(val) => val,
            Err(_) => panic!("PORT must be set"),
        };

        let db_config = DBConfig::new();

        Self {
            base_url,
            port,
            db_config,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_variables() {
        let env_variables = EnvVariables::new();

        assert_eq!(env_variables.base_url.len() > 0, true);
        assert_eq!(env_variables.port.len() > 0, true);
        assert_eq!(env_variables.db_config.get_db_url().len() > 0, true);
    }
}
