use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "sqlite:./videos.db?mode=rwc".to_string(),
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:./videos.db?mode=rwc".to_string());
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        Self {
            database_url,
            server_host,
            server_port,
        }
    }
}
