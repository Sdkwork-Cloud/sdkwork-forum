pub mod schema;
pub mod repo_impl;

#[derive(Debug, Clone)]
pub struct SqlxForumRepository {
    pub database_url_name: &'static str,
}

impl SqlxForumRepository {
    pub fn new_placeholder() -> Self {
        Self {
            database_url_name: "SDKWORK_FORUM_DATABASE_URL",
        }
    }
}
