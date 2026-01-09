use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database: String,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            username: std::env::var("DB_USERNAME")?,
            password: std::env::var("DB_PASSWORD")?,
            host: std::env::var("DB_HOST")?,
            port: std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()),
            database: std::env::var("DB_DBNAME")?,
        })
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

pub async fn build_pool(config: &DbConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.connection_string())
        .await
}
