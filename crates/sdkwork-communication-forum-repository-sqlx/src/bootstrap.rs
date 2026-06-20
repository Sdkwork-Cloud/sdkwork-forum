pub use sdkwork_forum_database_host::{
    bootstrap_forum_database, bootstrap_forum_database_from_env, ForumDatabaseHost,
};

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};

pub async fn connect_forum_database_pool_from_env() -> Result<DatabasePool, String> {
    let _ = dotenvy::dotenv();
    let config = DatabaseConfig::from_env("FORUM")
        .map_err(|error| format!("read forum database config failed: {error}"))?;
    create_pool_from_config(config)
        .await
        .map_err(|error| format!("create forum database pool failed: {error}"))
}

pub async fn connect_and_bootstrap_forum_database_from_env() -> Result<ForumDatabaseHost, String> {
    let pool = connect_forum_database_pool_from_env().await?;
    bootstrap_forum_database(pool).await
}
