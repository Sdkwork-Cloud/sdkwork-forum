use sdkwork_communication_forum_repository_sqlx::SqlxForumRepository;
use sdkwork_communication_forum_service::ForumService;
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;
use tracing;

pub struct ForumServiceHost {
    service: ForumService<SqlxForumRepository>,
}

impl ForumServiceHost {
    pub async fn new() -> Self {
        // Load .env file if present
        let _ = dotenvy::dotenv();

        tracing::info!("Connecting to database...");

        let pool = sdkwork_database_sqlx::create_pool_from_env("FORUM")
            .await
            .expect("Failed to create database pool")
            .expect("SDKWORK_FORUM_DATABASE_URL not set");

        let pg_pool = pool.as_postgres()
            .expect("Expected PostgreSQL pool for forum service");

        tracing::info!("Database connected successfully");

        let repository = SqlxForumRepository::new(pg_pool.clone());
        let service = ForumService::new(repository);

        tracing::info!("Forum service initialized");

        Self { service }
    }

    pub fn service(&self) -> &ForumService<SqlxForumRepository> {
        &self.service
    }

    pub fn build_request_context(&self, tenant_id: i64, organization_id: i64, user_id: i64) -> ForumRequestContext {
        ForumRequestContext::new(tenant_id, organization_id, user_id)
    }
}

pub fn build_forum_service() -> ForumService<SqlxForumRepository> {
    ForumService::new(SqlxForumRepository::new_placeholder())
}
