use sdkwork_communication_forum_repository_sqlx::SqlxForumRepository;
use sdkwork_communication_forum_service::ForumService;
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;
use sqlx::PgPool;
use tracing;

pub struct ForumServiceHost {
    service: ForumService<SqlxForumRepository>,
}

impl ForumServiceHost {
    pub async fn new() -> Self {
        let database_url = "postgresql://forum:forum@localhost:5432/forum".to_string();

        tracing::info!("Connecting to database...");

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        tracing::info!("Database connected successfully");

        let repository = SqlxForumRepository::new(pool);
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
