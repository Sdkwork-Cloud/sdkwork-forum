use sdkwork_communication_forum_service::integration::drive::{ForumDrivePort, LoggingForumDrivePort, NoopForumDrivePort};
use sdkwork_communication_forum_service::integration::notifications::{
    ForumNotificationPort, HttpForumNotificationPort, LoggingForumNotificationPort,
    NoopForumNotificationPort,
};
use sdkwork_communication_forum_service::integration::search::{
    ForumSearchPort, HttpForumSearchPort, LoggingForumSearchPort, NoopForumSearchPort,
};

pub fn build_drive_port() -> Box<dyn ForumDrivePort> {
    match std::env::var("SDKWORK_FORUM_DRIVE_URL") {
        Ok(url) if !url.trim().is_empty() => Box::new(LoggingForumDrivePort),
        _ => Box::new(NoopForumDrivePort),
    }
}

pub fn build_search_port() -> Box<dyn ForumSearchPort> {
    match std::env::var("SDKWORK_FORUM_SEARCH_URL") {
        Ok(url) if !url.trim().is_empty() => Box::new(HttpForumSearchPort::configured(
            url,
            std::env::var("SDKWORK_FORUM_SEARCH_INDEX_ID").unwrap_or_else(|_| "forum".to_string()),
            None,
            std::env::var("SDKWORK_ACCESS_TOKEN").ok(),
        )),
        _ if logging_ports_enabled() => Box::new(LoggingForumSearchPort),
        _ => Box::new(NoopForumSearchPort),
    }
}

pub fn build_notification_port() -> Box<dyn ForumNotificationPort> {
    match std::env::var("SDKWORK_FORUM_NOTIFICATION_URL") {
        Ok(url) if !url.trim().is_empty() => Box::new(HttpForumNotificationPort::new(url)),
        _ if logging_ports_enabled() => Box::new(LoggingForumNotificationPort),
        _ => Box::new(NoopForumNotificationPort),
    }
}

fn logging_ports_enabled() -> bool {
    matches!(
        std::env::var("SDKWORK_FORUM_INTEGRATION_LOG").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}
