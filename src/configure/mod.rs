use std::str::FromStr;

use self::{db::DatabaseConfig,http::HttpClientConfig, redis::RedisConfig, server::ServerConfig};
use crate::utils::dir::get_project_root;
use ::tracing::info;
use config::{ConfigError, Environment};
use serde::Deserialize;

pub mod db;
pub mod env;
pub mod http;
pub mod redis;
pub mod secret;
pub mod server;
pub mod tracing;
// pub mod redis_cluster;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub redis: RedisConfig,
    // pub redis_cluster: CRedisConfig,
    pub db: DatabaseConfig,
    pub http: HttpClientConfig,
    // pub secret: SecretConfig,
}

impl AppConfig {
    pub fn read(env_src: Environment) -> Result<Self, config::ConfigError> {
        let config_dir = get_settings_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_e| Ok(Profile::Dev))?;
        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("local.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(env_src)
            .build()?;
        info!("Successfully read config profile: {profile}.");
        config.try_deserialize()
    }
}

pub fn get_settings_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("settings"))
}

#[derive(
    Debug,
    strum::Display,
    strum::EnumString,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "local")]
    #[strum(serialize = "local")]
    Local,
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "sandbox")]
    #[strum(serialize = "sandbox")]
    Sandbox,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}

#[cfg(test)]
mod tests {
    use self::env::get_env_source;

    pub use super::*;

    #[test]
    pub fn test_read_app_config() {
        let _config = AppConfig::read(get_env_source("TEST_APP")).unwrap();
    }

    #[test]
    pub fn test_profile_to_string() {
        let profile: Profile = Profile::try_from("dev").unwrap();
        assert_eq!(profile, Profile::Dev)
    }
}
