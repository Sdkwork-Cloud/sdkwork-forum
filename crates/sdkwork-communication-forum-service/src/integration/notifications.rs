pub trait ForumNotificationPort: Send + Sync {
    fn publish_forum_event(&self, event_type: &str, aggregate_id: &str) -> Result<(), String>;
    fn publish_moderation_alert(&self, case_id: i64, severity: &str) -> Result<(), String>;
    fn publish_subscription_notification(&self, user_id: i64, event_type: &str, target_id: i64) -> Result<(), String>;
}

pub struct NoopForumNotificationPort;

impl ForumNotificationPort for NoopForumNotificationPort {
    fn publish_forum_event(&self, _event_type: &str, _aggregate_id: &str) -> Result<(), String> {
        Ok(())
    }

    fn publish_moderation_alert(&self, _case_id: i64, _severity: &str) -> Result<(), String> {
        Ok(())
    }

    fn publish_subscription_notification(&self, _user_id: i64, _event_type: &str, _target_id: i64) -> Result<(), String> {
        Ok(())
    }
}

pub struct LoggingForumNotificationPort;

impl ForumNotificationPort for LoggingForumNotificationPort {
    fn publish_forum_event(&self, event_type: &str, aggregate_id: &str) -> Result<(), String> {
        eprintln!("[forum-notification] event={event_type} aggregate={aggregate_id}");
        Ok(())
    }

    fn publish_moderation_alert(&self, case_id: i64, severity: &str) -> Result<(), String> {
        eprintln!("[forum-notification] moderation alert case={case_id} severity={severity}");
        Ok(())
    }

    fn publish_subscription_notification(&self, user_id: i64, event_type: &str, target_id: i64) -> Result<(), String> {
        eprintln!("[forum-notification] subscription user={user_id} event={event_type} target={target_id}");
        Ok(())
    }
}
