use std::path::PathBuf;
use std::sync::Arc;

use sdkwork_communication_forum_repository_sqlx::SqlxForumRepository;
use sqlx::PgPool;
use sdkwork_communication_forum_service::ForumService;
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;
use sdkwork_database_ops::DatabaseOpsService;
use sdkwork_database_spi::{DefaultDatabaseModule, LocaleTag, SeedProfile};
use sdkwork_database_sqlx::DatabasePool;
use tracing;

mod ports;

pub struct ForumServiceHost {
    service: ForumService<SqlxForumRepository>,
    pool: DatabasePool,
    pg_pool: PgPool,
    iam_pool: Option<PgPool>,
    database_module: Arc<DefaultDatabaseModule>,
}

impl ForumServiceHost {
    pub async fn new() -> Self {
        let _ = dotenvy::dotenv();

        tracing::info!("Connecting to database...");

        let pool = sdkwork_database_sqlx::create_pool_from_env("FORUM")
            .await
            .expect("Failed to create database pool")
            .expect("SDKWORK_FORUM_DATABASE_URL not set");

        let app_root = resolve_app_root();
        let database_module = Arc::new(
            DefaultDatabaseModule::from_app_root(&app_root)
                .expect("failed to load forum database module"),
        );

        let pg_pool = pool
            .as_postgres()
            .expect("Expected PostgreSQL pool for forum service")
            .clone();

        let iam_pool = if iam_enabled_from_env() {
            Some(load_iam_pool(&pg_pool).await)
        } else {
            None
        };

        tracing::info!("Database connected successfully");

        let repository = SqlxForumRepository::new(pg_pool.clone());
        let service = ForumService::new_with_ports(
            repository,
            ports::build_drive_port(),
            ports::build_search_port(),
            ports::build_notification_port(),
        );

        tracing::info!("Forum service initialized");

        Self {
            service,
            pool,
            pg_pool,
            iam_pool,
            database_module,
        }
    }

    pub fn service(&self) -> &ForumService<SqlxForumRepository> {
        &self.service
    }

    pub fn database_pool(&self) -> DatabasePool {
        self.pool.clone()
    }

    pub fn postgres_pool(&self) -> &PgPool {
        &self.pg_pool
    }

    pub fn iam_pool(&self) -> Option<&PgPool> {
        self.iam_pool.as_ref()
    }

    pub fn database_module(&self) -> Arc<DefaultDatabaseModule> {
        self.database_module.clone()
    }

    pub fn database_ops_service(&self) -> DatabaseOpsService {
        DatabaseOpsService::new(self.pool.clone(), self.database_module.clone())
    }

    pub fn build_request_context(
        &self,
        tenant_id: i64,
        organization_id: i64,
        user_id: i64,
    ) -> ForumRequestContext {
        ForumRequestContext::new(tenant_id, organization_id, user_id)
    }
}

fn iam_enabled_from_env() -> bool {
    matches!(
        std::env::var("SDKWORK_FORUM_IAM_ENABLED").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

async fn load_iam_pool(forum_pool: &PgPool) -> PgPool {
    if let Ok(url) = std::env::var("SDKWORK_FORUM_IAM_DATABASE_URL") {
        if !url.trim().is_empty() {
            return PgPool::connect(&url)
                .await
                .expect("Failed to connect SDKWORK_FORUM_IAM_DATABASE_URL");
        }
    }
    forum_pool.clone()
}

fn resolve_app_root() -> PathBuf {
    std::env::var("SDKWORK_FORUM_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        })
}

pub fn build_forum_service() -> ForumService<SqlxForumRepository> {
    ForumService::new(SqlxForumRepository::new_placeholder())
}

pub fn default_seed_locale() -> LocaleTag {
    LocaleTag(
        std::env::var("SDKWORK_FORUM_DATABASE_SEED_LOCALE").unwrap_or_else(|_| "zh-CN".to_string()),
    )
}

pub fn default_seed_profile() -> SeedProfile {
    SeedProfile(
        std::env::var("SDKWORK_FORUM_DATABASE_SEED_PROFILE")
            .unwrap_or_else(|_| "standard".to_string()),
    )
}
