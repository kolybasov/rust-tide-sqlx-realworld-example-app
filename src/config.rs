use crate::Error;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        dotenv()?;

        let database_url = env::var("DATABASE_URL")?;
        let jwt_secret = env::var("JWT_SECRET")?;
        let host = env::var("HOST").unwrap_or("127.0.0.1".into());
        let port = env::var("PORT").unwrap_or("8080".into());

        Ok(Config {
            database_url,
            jwt_secret,
            host,
            port,
        })
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
