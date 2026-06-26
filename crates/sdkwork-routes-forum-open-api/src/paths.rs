pub const PREFIX: &str = "/forum/v3/api";

pub const ROUTES: &[(&str, &str, &str)] = &[
    ("GET", "/forum/v3/api/sites/{siteSlug}/boards", "boards.list"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/boards/{boardId}/topics", "boards.topics.list"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/topics", "topics.list"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/topics/{topicId}", "topics.retrieve"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/topics/by_slug/{topicSlug}", "topics.bySlug.retrieve"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/topics/{topicId}/replies", "topics.replies.list"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/tags", "tags.list"),
    ("GET", "/forum/v3/api/sites/{siteSlug}/search", "search.query"),
];
