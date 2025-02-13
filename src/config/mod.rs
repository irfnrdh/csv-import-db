use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub notification: NotificationConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub types: HashSet<DatabaseType>,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum DatabaseType {
    #[serde(rename = "postgresql")]
    PostgreSQL,
    #[serde(rename = "mysql")]
    MySQL,
}

#[derive(Debug, Deserialize)]
pub struct NotificationConfig {
    pub telegram_token: Option<String>,
    pub telegram_chat_id: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name("config"))?;
        settings.try_into()
    }
}