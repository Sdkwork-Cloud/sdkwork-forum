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
        eprintln!("[forum-search] index {source_type}/{source_id}");
        Ok(())
    }

    fn delete_document(&self, source_type: &str, source_id: &str) -> Result<(), String> {
        eprintln!("[forum-search] delete {source_type}/{source_id}");
        Ok(())
    }

    fn rebuild_index(&self, board_id: Option<i64>) -> Result<(), String> {
        eprintln!("[forum-search] rebuild index board={board_id:?}");
        Ok(())
    }
}
