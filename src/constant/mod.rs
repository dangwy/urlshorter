use crate::{
    client::{http::HttpClient, redis::RedisClient, ClientBuilder},
    configure::env::get_env_source,
    // handler::openapi::ApiDoc,
};
use once_cell::sync::Lazy;

// use jsonwebtoken::{DecodingKey, EncodingKey};
use std::time::Duration;
// use serde_json::from_str;
// use utoipa::OpenApi;

pub const REDIS_KEY_PREFIX: &str = "linkshorter";
pub const ENV_PREFIX: &str = "APP";
pub const CODE_LEN: usize = 5;
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);
pub const EXPIRE_LINK_SECS: Duration = Duration::from_secs(0);
pub const EXPIRE_RELA_SECS: Duration = Duration::from_secs(0);
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);
pub const QUEUE_EMPTY_DELAY_SECS: Duration = Duration::from_secs(60);
pub const COMPLETE_TASK_DELAY_SECS: Duration = Duration::from_secs(10);
pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";
pub const APP_DOMAIN: &str = "";
pub const APP_EMAIL_ADDR: &str = "dangwy@gmail.com";

pub static CONFIG: Lazy<crate::configure::AppConfig> =
    Lazy::new(|| crate::configure::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap());

pub static HTTP: Lazy<reqwest::Client> =
    Lazy::new(|| HttpClient::build_from_config(&CONFIG).unwrap());

// pub static CREDIS: Lazy<RedisCClient> =
//     Lazy::new(|| RedisCClient::build_from_config(&CONFIG).unwrap());

pub static REDIS: Lazy<RedisClient> =
    Lazy::new(|| RedisClient::build_from_config(&CONFIG).unwrap());

pub const MAX_RETRY: u32 = 10;
pub const MINIMUM_DELAY_TIME: std::time::Duration = std::time::Duration::from_millis(100);
// pub static REFRESH_TOKEN_ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
//     let key = CONFIG.secret.read_private_refresh_key().unwrap();
//     EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
// });
// pub static REFRESH_TOKEN_DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
//     let key = CONFIG.secret.read_public_refresh_key().unwrap();
//     DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
// });
// pub static ACCESS_TOKEN_ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
//     let key = CONFIG.secret.read_private_access_key().unwrap();
//     EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
// });
// pub static ACCESS_TOKEN_DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
//     let key = CONFIG.secret.read_public_access_key().unwrap();
//     DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
// });
// pub static API_DOC: Lazy<utoipa::openapi::OpenApi> = Lazy::new(ApiDoc::openapi);
// pub static TEMPLATE_ENGIN: Lazy<TemplateEngine> = Lazy::new(|| {
//     let path = get_static_dir()
//         .unwrap()
//         .join("template/**/*")
//         .into_os_string()
//         .into_string()
//         .unwrap();
//     TemplateEngine::new(&path).unwrap()
// });
