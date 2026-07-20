#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDescriptor {
    pub method: &'static str,
    pub path: &'static str,
    pub operation_id: &'static str,
    pub surface: &'static str,
    pub auth_mode: &'static str,
    pub tags: &'static [&'static str],
}

impl RouteDescriptor {
    pub const fn new(
        method: &'static str,
        path: &'static str,
        operation_id: &'static str,
        auth_mode: &'static str,
        tags: &'static [&'static str],
    ) -> Self {
        Self {
            method,
            path,
            operation_id,
            surface: "app-api",
            auth_mode,
            tags,
        }
    }
}

pub const APP_ROUTES: &[RouteDescriptor] = &[
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/nodes/tree",
        "nodes.tree.list",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/boards/{boardId}/topics",
        "topics.list",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/topics",
        "topics.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/topics/{topicId}",
        "topics.retrieve",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "PATCH",
        "/app/v3/api/forum/topics/{topicId}",
        "topics.update",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "DELETE",
        "/app/v3/api/forum/topics/{topicId}",
        "topics.delete",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/topics/{topicId}/replies",
        "topics.replies.list",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/topics/{topicId}/replies",
        "topics.replies.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "PATCH",
        "/app/v3/api/forum/replies/{replyId}",
        "replies.update",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "DELETE",
        "/app/v3/api/forum/replies/{replyId}",
        "replies.delete",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/topics/{topicId}/revisions",
        "topics.revisions.list",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/replies/{replyId}/revisions",
        "replies.revisions.list",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "PUT",
        "/app/v3/api/forum/questions/{topicId}/accepted_reply",
        "questions.acceptedReply.update",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "DELETE",
        "/app/v3/api/forum/questions/{topicId}/accepted_reply",
        "questions.acceptedReply.delete",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/polls/{pollId}/votes",
        "polls.votes.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/reactions",
        "reactions.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/votes",
        "votes.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/bookmarks",
        "bookmarks.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "PATCH",
        "/app/v3/api/forum/read_state/topics/{topicId}",
        "readState.topics.update",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "POST",
        "/app/v3/api/forum/reports",
        "reports.create",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/feed",
        "feed.list",
        "dual-token",
        &["communication"],
    ),
    RouteDescriptor::new(
        "GET",
        "/app/v3/api/forum/search",
        "search.query",
        "dual-token",
        &["communication"],
    ),
];

pub fn build_sdkwork_forum_app_api_router() -> Vec<RouteDescriptor> {
    APP_ROUTES.to_vec()
}

pub fn find_route(method: &str, path: &str) -> Option<&'static RouteDescriptor> {
    APP_ROUTES
        .iter()
        .find(|r| r.method == method && path_matches(r.path, path))
}

fn path_matches(template: &str, actual: &str) -> bool {
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return false;
    }
    template_segments
        .iter()
        .zip(actual_segments.iter())
        .all(|(t, a)| t.starts_with('{') || t == a)
}
