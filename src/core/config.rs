use sea_orm::ConnectOptions;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::{env::var as get_env, time::Duration};

use crate::core::{constant::*, error::SwanError};

macro_rules! env {
    ($key:expr) => {
        get_env($key).map_err(|_| SwanError::MissingEnvVar($key.into()))
    };
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DatabaseConfig {
    pub db_name: String,
    pub db_user: String,
    #[serde(skip)]
    pub db_password: SecretString,
    pub db_host: String,
    pub db_port: String,
    pub db_type: String,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, SwanError> {
        Ok(Self {
            db_name: env!("DATABASE_NAME")?,
            db_user: env!("DATABASE_USER")?,
            db_password: env!("DATABASE_PASSWORD")?.into(),
            db_host: env!("DATABASE_HOST")?,
            db_port: env!("DATABASE_PORT")?,
            db_type: env!("DATABASE_TYPE")?,
        })
    }

    pub fn create_db_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.db_type,
            self.db_user,
            self.db_password.expose_secret(),
            self.db_host,
            self.db_port,
            self.db_name
        )
    }

    pub fn create_sqlite_url(&self) -> String {
        format!("sqlite://{}?mode=rwc", "./data/db_crypto.db")
    }

    pub async fn create_options(&self, db_type: String) -> Result<ConnectOptions, SwanError> {
        let mut opt = match db_type.as_str() {
            "SQLite" => ConnectOptions::new(self.create_sqlite_url()),
            "Postgres" => ConnectOptions::new(self.create_db_url()),
            _ => ConnectOptions::new(self.create_db_url()),
        };
        opt.max_connections(DB_MAX_CONNECTION)
            .min_connections(DB_MIN_CONNECTION)
            .connect_timeout(Duration::from_secs(DB_CONNECTION_TIMEOUT_SECS))
            .acquire_timeout(Duration::from_secs(DB_ACQUIRE_TIMEOUT_SECS))
            .idle_timeout(Duration::from_secs(DB_IDLE_TIMEOUT_SECS))
            .max_lifetime(Duration::from_secs(DB_MAX_LIFETIME_SECS))
            .sqlx_logging(true);

        Ok(opt)
    }
}
