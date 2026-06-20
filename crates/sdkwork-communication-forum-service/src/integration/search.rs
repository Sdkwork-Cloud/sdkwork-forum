use std::sync::Arc;

pub trait ForumSearchPort: Send + Sync {
    fn index_document(&self, source_type: &str, source_id: &str) -> Result<(), String>;
    fn delete_document(&self, source_type: &str, source_id: &str) -> Result<(), String>;
    fn rebuild_index(&self, board_id: Option<i64>) -> Result<(), String>;
}

pub struct NoopForumSearchPort;

impl ForumSearchPort for NoopForumSearchPort {
    fn index_document(&self, _source_type: &str, _source_id: &str) -> Result<(), String> {
        Ok(())
    }

    fn delete_document(&self, _source_type: &str, _source_id: &str) -> Result<(), String> {
        Ok(())
    }

    fn rebuild_index(&self, _board_id: Option<i64>) -> Result<(), String> {
        Ok(())
    }
}

pub struct LoggingForumSearchPort;

impl ForumSearchPort for LoggingForumSearchPort {
    fn index_document(&self, source_type: &str, source_id: &str) -> Result<(), String> {
        tracing::info!(source_type, source_id, "forum search index requested");
        Ok(())
    }

    fn delete_document(&self, source_type: &str, source_id: &str) -> Result<(), String> {
        tracing::info!(source_type, source_id, "forum search delete requested");
        Ok(())
    }

    fn rebuild_index(&self, board_id: Option<i64>) -> Result<(), String> {
        tracing::info!(?board_id, "forum search rebuild requested");
        Ok(())
    }
}

pub struct HttpForumSearchPort {
    base_url: Arc<String>,
    client: ureq::Agent,
}

impl HttpForumSearchPort {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Arc::new(base_url.into().trim_end_matches('/').to_string()),
            client: ureq::Agent::new(),
        }
    }

    fn post_action(&self, action: &str, body: serde_json::Value) -> Result<(), String> {
        let url = format!("{}/forum/v1/search/{}", self.base_url, action);
        self.client
            .post(&url)
            .set("Content-Type", "application/json")
            .send_json(body)
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

impl ForumSearchPort for HttpForumSearchPort {
    fn index_document(&self, source_type: &str, source_id: &str) -> Result<(), String> {
        self.post_action(
            "index",
            serde_json::json!({
                "sourceType": source_type,
                "sourceId": source_id,
            }),
        )
    }

    fn delete_document(&self, source_type: &str, source_id: &str) -> Result<(), String> {
        self.post_action(
            "delete",
            serde_json::json!({
                "sourceType": source_type,
                "sourceId": source_id,
            }),
        )
    }

    fn rebuild_index(&self, board_id: Option<i64>) -> Result<(), String> {
        self.post_action("rebuild", serde_json::json!({ "boardId": board_id }))
    }
}
