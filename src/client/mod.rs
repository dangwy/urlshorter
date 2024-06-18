use crate::{configure::AppConfig, error::AppResult};

pub mod database;
pub mod http;
pub mod redis;
// pub mod redis_cluster;

pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> AppResult<Self>;
}
