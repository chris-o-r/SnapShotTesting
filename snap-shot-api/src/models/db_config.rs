use dotenv::dotenv;
use std::env;
#[derive(Clone)]
pub struct DBConfig {
    pub host: String,
    pub port: String,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl DBConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let host = match env::var("DB_HOST") {
            Ok(val) => val,
            Err(_) => panic!("DB_HOST must be set"),
        };

        let port = match env::var("DB_PORT") {
            Ok(val) => val,
            Err(_) => panic!("DB_PORT must be set"),
        };

        let name = match env::var("DB_NAME") {
            Ok(val) => val,
            Err(_) => panic!("DB_NAME must be set"),
        };

        let user = match env::var("DB_USER") {
            Ok(val) => val,
            Err(_) => panic!("DB_USER must be set"),
        };

        let password = match env::var("DB_PASSWORD") {
            Ok(val) => val,
            Err(_) => panic!("DB_PASSWORD must be set"),
        };

        Self {
            host,
            port,
            name,
            user,
            password,
        }
    }

    pub fn get_db_url(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_new() {
        let db_config: DBConfig = DBConfig::new();

        assert_eq!(db_config.host.len() > 0, true);
        assert_eq!(db_config.port.len() > 0, true);
        assert_eq!(db_config.name.len() > 0, true);
        assert_eq!(db_config.user.len() > 0, true);
        assert_eq!(db_config.password.len() > 0, true);
    }

    #[test]
    fn test_get_db_url() {
        let db_config = DBConfig::new();
        let regex = Regex::new(r"^mysql://[^:]+:[^@]+@[^/]+/[^?]+$").unwrap();
        assert_eq!(regex.is_match(&db_config.get_db_url()), true);
    }
}
