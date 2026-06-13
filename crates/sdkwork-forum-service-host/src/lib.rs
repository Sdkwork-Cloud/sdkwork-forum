use sdkwork_communication_forum_repository_sqlx::SqlxForumRepository;
use sdkwork_communication_forum_service::ForumService;
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;
use tracing;

pub struct ForumServiceHost {
    service: ForumService<SqlxForumRepository>,
}

impl ForumServiceHost {
    pub fn new() -> Self {
        tracing::info!("Initializing forum service...");

        let repository = SqlxForumRepository::new_placeholder();
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

impl Default for ForumServiceHost {
    fn default() -> Self {
        Self::new()
    }
}

pub fn build_forum_service() -> ForumService<SqlxForumRepository> {
    ForumService::new(SqlxForumRepository::new_placeholder())
}
