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
        Self { method, path, operation_id, surface: "backend-api", auth_mode, tags }
    }
}

pub const BACKEND_ROUTES: &[RouteDescriptor] = &[
    RouteDescriptor::new("GET", "/backend/v3/api/forum/nodes", "nodes.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/nodes", "nodes.create", "dual-token", &["communication"]),
    RouteDescriptor::new("PATCH", "/backend/v3/api/forum/nodes/{nodeId}", "nodes.update", "dual-token", &["communication"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/forum/nodes/{nodeId}", "nodes.delete", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/topic_prefixes", "topicPrefixes.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/topic_prefixes", "topicPrefixes.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/topics", "topics.list", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/topics/{topicId}", "topics.retrieve", "dual-token", &["communication"]),
    RouteDescriptor::new("PATCH", "/backend/v3/api/forum/topics/{topicId}", "topics.update", "dual-token", &["communication"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/forum/topics/{topicId}", "topics.delete", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/topics/{topicId}/pin", "topics.pin.create", "dual-token", &["communication"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/forum/topics/{topicId}/pin", "topics.pin.delete", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/topics/{topicId}/feature", "topics.feature.create", "dual-token", &["communication"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/forum/topics/{topicId}/feature", "topics.feature.delete", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/topics/{topicId}/lock", "topics.lock.create", "dual-token", &["communication"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/forum/topics/{topicId}/lock", "topics.lock.delete", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/topics/{topicId}/move", "topics.move.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/moderation/queue", "moderation.queue.list", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/moderation/cases", "moderation.cases.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/moderation/cases", "moderation.cases.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/moderation/cases/{caseId}", "moderation.cases.retrieve", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/moderation/cases/{caseId}/decisions", "moderation.cases.decisions.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/sanctions", "sanctions.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/sanctions", "sanctions.create", "dual-token", &["communication"]),
    RouteDescriptor::new("PATCH", "/backend/v3/api/forum/sanctions/{sanctionId}", "sanctions.update", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/reputation/rules", "reputation.rules.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/reputation/rules", "reputation.rules.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/reputation/ledger", "reputation.ledger.list", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/trust_levels", "trustLevels.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/trust_levels", "trustLevels.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/badges", "badges.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/badges", "badges.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/stats/boards", "stats.boards.list", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/stats/topics", "stats.topics.list", "dual-token", &["communication"]),
    RouteDescriptor::new("POST", "/backend/v3/api/forum/search/reindex", "search.reindex.create", "dual-token", &["communication"]),
    RouteDescriptor::new("GET", "/backend/v3/api/forum/audit/actions", "audit.actions.list", "dual-token", &["communication"]),
];

pub fn build_sdkwork_forum_backend_api_router() -> Vec<RouteDescriptor> {
    BACKEND_ROUTES.to_vec()
}

pub fn find_route(method: &str, path: &str) -> Option<&'static RouteDescriptor> {
    BACKEND_ROUTES.iter().find(|r| r.method == method && path_matches(r.path, path))
}

fn path_matches(template: &str, actual: &str) -> bool {
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return false;
    }
    template_segments.iter().zip(actual_segments.iter()).all(|(t, a)| {
        t.starts_with('{') || t == a
    })
}
