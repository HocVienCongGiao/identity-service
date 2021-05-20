#[derive(Debug)]
pub struct Config {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub os_type: String,
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let ostype_env_result = std::env::var("OSTYPE");
        let mut os_type: String = "windows".to_string();
        let ostype_env_lowercase = ostype_env_result
            .unwrap_or_else(|_| String::from("windows"))
            .to_lowercase();
        match &ostype_env_lowercase[0..5] {
            "linux" => {
                os_type = String::from("linux");
            }
            "darwi" => {
                os_type = String::from("darwin");
            }
            _ => {}
        }

        let db_host = std::env::var("APP_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_port = std::env::var("APP_DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let db_user = std::env::var("APP_DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password =
            std::env::var("APP_DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let db_name = std::env::var("APP_DB_NAME").unwrap_or_else(|_| "database_name".to_string());

        Config {
            db_host,
            db_port,
            db_user,
            db_password,
            db_name,
            os_type,
        }
    }
}
