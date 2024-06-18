use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CRedisConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    // pub username: String,
    // pub database_name: String,
}

impl CRedisConfig {
    pub fn get_url(&self) -> String {
        format!(
            "rediss://{host}:{port}",
            // password = self.password,
            host = self.host,
            port = self.port
        )
    }
}
