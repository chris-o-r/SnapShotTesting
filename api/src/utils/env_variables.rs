use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct EnvVariables {
    pub base_url: String,
    pub port: String,
    pub db_url: String,
    pub selenium_port: String,
    pub selenium_host: String,
    pub assets_folder: String,
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

        let db_url = match env::var("DATABASE_URL") {
            Ok(val) => val,
            Err(_) => panic!("DATABASE_URL must be set"),
        };

        let selenium_port = match env::var("SELENIUM_PORT") {
            Ok(val) => val,
            Err(_) => panic!("SELENIUM_PORT must be set"),
        };

        let selenium_host = match env::var("SELENIUM_HOST") {
            Ok(val) => val,
            Err(_) => panic!("SELENIUM_HOST must be set"),
        };

        let assets_folder = match env::var("ASSETS_FOLDER") {
            Ok(val) => val,
            Err(_) => panic!("ASSETS_FOLDER must be set"),
        };

        Self {
            assets_folder,
            base_url,
            port,
            db_url,
            selenium_port,
            selenium_host,
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
        assert_eq!(env_variables.db_url.len() > 0, true);
    }
}
