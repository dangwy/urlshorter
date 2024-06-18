/*
use crate::{
    configure::{redis_cluster::CRedisConfig, AppConfig},
    constant::CONFIG,
    error::AppResult,
};
use redis::RedisError;
use super::ClientBuilder;
use redis::cluster::ClusterClientBuilder;
use redis::cluster::ClusterClient;
use redis::AsyncCommands;

use std::time::Duration;
use test_context::AsyncTestContext;
use tracing::log::info;


pub type RedisCClient = ClusterClient;

pub trait RedisClientExt: ClientBuilder {
    fn ping(&self) -> impl std::future::Future<Output = Result<Option<String>, RedisError>>;
    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration,
    ) -> impl std::future::Future<Output = Result<(), RedisError>>;
    fn exist(&self, key: &str) -> impl std::future::Future<Output = Result<bool, RedisError>>;
    fn get(&self, key: &str)
           -> impl std::future::Future<Output = Result<Option<String>, RedisError>>;
    fn del(&self, key: &str) -> impl std::future::Future<Output = Result<bool, RedisError>>;
    fn ttl(&self, key: &str) -> impl std::future::Future<Output = Result<i64, RedisError>>;
    fn incr(&self, key: &str) -> impl std::future::Future<Output = Result<i64, RedisError>>;
}

impl ClientBuilder for RedisCClient {
    fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        // Ok(ClusterClient::new(vec![config.redis.get_url()])?)
        let initial_nodes = vec![config.redis.get_url()];
        let builder = ClusterClientBuilder::new(initial_nodes)
            .password(config.redis.password.to_string())
            // .tls(redis::TlsMode::Secure)
            .read_from_replicas();
        Ok(builder.build()?)
        // cClient.get_connection().unwrap()
        // Ok(redis::Client::open(config.redis.get_url())?)
    }
}

pub struct RedisTestContext {
    pub config: CRedisConfig,
    pub redis: RedisCClient,
}

impl AsyncTestContext for RedisTestContext {
    async fn setup() -> Self {
        info!("setup redis config for the test");
        // let database_name = util::string::generate_random_string_with_prefix("test_db");
        let redis = RedisCClient::build_from_config(&CONFIG).unwrap();
        Self {
            config: CONFIG.redis_cluster.clone(),
            redis,
        }
    }

    async fn teardown(self) {
        // TODO drop db
    }
}

impl RedisClientExt for RedisCClient {
    async fn ping(&self) -> Result<Option<String>, RedisError> {
        let mut conn = self.get_connection().unwrap();
        let value: Option<String> = redis::cmd("PING").query(&mut conn).unwrap();
        info!("ping redis server");
        Ok(value)
    }

    async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError> {
        let mut conn = self.get_connection().unwrap();
        // let mut conn = self.get_multiplexed_async_connection().await?;
        let msg: String = redis::cmd("SET")
            .arg(&[key, value])
            .query(&mut conn)
            .unwrap();
        info!("set key redis: {msg}");
        let msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query(&mut conn)
            .unwrap();
        info!("set expire time redis: {msg}");
        Ok(())
    }

    async fn exist(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.get_connection().unwrap();
        // let mut conn = self.get_multiplexed_async_connection().await?;
        let value: bool = redis::cmd("EXISTS").arg(key).query(&mut conn).unwrap();
        info!("check key exists: {key}");
        Ok(value)
    }

    async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.get_connection().unwrap();
        // let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query(&mut conn).unwrap();
        info!("get value: {key}");
        Ok(value)
    }

    async fn del(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.get_connection().unwrap();
        // let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i32 = redis::cmd("DEL").arg(key).query(&mut conn).unwrap();
        info!("delete value: {key}");
        Ok(value == 1)
    }
    async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.get_connection().unwrap();
        // let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i64 = redis::cmd("TTL").arg(key).query(&mut conn).unwrap();
        info!("get TTL value: {key}");
        Ok(value)
    }
    async fn incr(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.get_connection().unwrap();
        let value: i64 = redis::cmd("INCR").arg(key).query(&mut conn).unwrap();
        info!("incr value: {key}");
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::constant::CREDIS;

    use super::*;

    use fake::{Fake, Faker};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_ping_redis_server() {
        let resp = CREDIS.ping().await.unwrap();
        // let pong = "PONG";
        // assert!(matches!(resp, Some(p) if p == pong));
        if let Some(p) = resp {
            println!("{p}")
        }
    }

    #[tokio::test]
    async fn test_set_key_redis() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        CREDIS.set(&key, &value, Duration::from_secs(5)).await.unwrap();
        let resp = CREDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = CREDIS.ttl(&key).await.unwrap();
        assert!(resp > 0);
    }

    #[tokio::test]
    async fn test_exist_key_redis() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        CREDIS.set(&key, &value, Duration::from_secs(4)).await.unwrap();
        let resp = CREDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = CREDIS.exist(&key).await.unwrap();
        assert!(resp);
        let key: String = Faker.fake();
        let resp = CREDIS.exist(&key).await.unwrap();
        assert!(!resp);
    }

    #[tokio::test]
    async fn test_del_key_redis() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        CREDIS.set(&key, &value, Duration::from_secs(4)).await.unwrap();
        let resp = CREDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = CREDIS.exist(&key).await.unwrap();
        assert!(resp);
        CREDIS.del(&key).await.unwrap();
        let resp = CREDIS.exist(&key).await.unwrap();
        assert!(!resp);
    }

    #[tokio::test]
    async fn test_key_ttl_redis() {
        let key: String = Faker.fake();
        let ttl = 4;
        let value = Uuid::new_v4().to_string();
        CREDIS.set(&key, &value, Duration::from_secs(ttl)).await.unwrap();
        let resp = CREDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = CREDIS.ttl(&key).await.unwrap();
        assert!(resp <= ttl as i64 && resp > 0);
        CREDIS.del(&key).await.unwrap();
        let resp = CREDIS.ttl(&key).await.unwrap();
        assert!(resp < 0);
    }
}
*/