use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;

use crate::client::redis::RedisClient;
use crate::client::redis::RedisClientExt;
use crate::error::AppResult;
use tracing::info;
use uuid::Uuid;

use crate::constant::*;
use fake::Dummy;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// use crate::entities::urls::Model;

pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;
    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}


#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionKey {
    pub u_id: Uuid,
}

impl RedisKey for SessionKey {
    type Value = Uuid;
    const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSION_KEY_{}", self.u_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct UrlKey {
    pub domain: String,
}

impl RedisKey for UrlKey {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_LINK_SECS;
}

impl Display for UrlKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_{}", self.domain)
    }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct RelaIdKey {
    pub domain: String,
}

impl RedisKey for crate::service::redis::RelaIdKey {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_RELA_SECS;
}

impl Display for crate::service::redis::RelaIdKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KEY_{}", self.domain)
    }
}

// set
pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
where
    K: RedisKey,
{
    info!("Set value to redis key :{key:?} value :{value:?}");
    let value = serde_json::to_string(value)?;
    client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
    Ok(())
}

// get
pub async fn get<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>>
where
    K: RedisKey,
{
    info!("Get value from redis key :{key}");
    Ok(client
        .get(&key.to_string())
        .await?
        .map(|v| serde_json::from_str::<K::Value>(&v))
        .transpose()?)
}

// del
pub async fn del(client: &RedisClient, key: &impl RedisKey) -> Result<bool, redis::RedisError> {
    info!("Delete key in redis :{key:?}");
    client.del(&key.to_string()).await
}

// get_tll
pub async fn get_tll(client: &RedisClient, key: &impl RedisKey) -> Result<i64, redis::RedisError> {
    info!("Get ttl key in redis :{key:?}");
    client.ttl(&key.to_string()).await
}

// check_exist_key
pub async fn check_exist_key(redis: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
    Ok(redis.exist(&key.to_string()).await?)
}

// incr
pub async fn incr(client: &RedisClient, key: &impl RedisKey) -> AppResult<i64> {
    info!("Incr key in redis :{key:?}");
    Ok(client.incr(&key.to_string()).await?)
}
// // hset
// pub async fn hset(client: &RedisClient, key: &str, value: &Model) -> AppResult<()>
// {
//     info!("Set value to redis key :{key:?} value :{value:?}");
//     // 使用管道来批量执行命令，提高效率
//     let mut pipe = client.pipeline();
//     pipe.atomic()
//         .hset(&key, "id", value.id)
//         .hset(&key, "domain", value.domain.clone())
//         .hset(&key, "shorter_url", value.shorter_url.clone())
//         .hset(&key, "deleted", value.deleted)
//         // .hset(&key, "tags", value.tags)
//         .hset(&key, "created_at", value.created_at)
//         .hset(&key, "expires_at", value.expires_at)
//         .hset(&key, "url", value.url.clone())
//         .hset(&key, "description", value.description.clone())
//         .hset(&key, "hits", value.hits)
//         .execute_async(client.get_multiplexed_async_connection().await?);
//
//     // 等待管道中的所有命令执行完毕
//     pipe.query_async::<()>(client.get_multiplexed_async_connection().await?).await?;
//
//     let tag_key = format!("{}:{}",key.clone(), "tags");
//     let tag_value = value.tags;
//     client.sadd(&tag_key, &tag_value).await?;
//
//
//     // let value = serde_json::to_string(value)?;
//     // client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
//     Ok(())
// }

mod tests {
    use fake::{Fake, Faker};

    use super::*;

    #[tokio::test]
    async fn test_set_and_get_str_redis_service() {
        let key: UrlKey = Faker.fake();
        let value: String = Faker.fake();
        set(&REDIS, (&key, &value)).await.unwrap();
        let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
        info!(?actual_value, ?key);
        assert_eq!(actual_value, value);
    }

    #[tokio::test]
    async fn test_incr_redis_service() {
        let key: UrlKey = Faker.fake();
        let incr_id = incr(&REDIS, &key).await.unwrap();
        info!(?incr_id);
    }
}
