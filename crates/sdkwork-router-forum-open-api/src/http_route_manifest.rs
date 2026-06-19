use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/boards",
        "communication",
        "boards.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/boards/{boardId}/topics",
        "communication",
        "boards.topics.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/topics",
        "communication",
        "topics.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/topics/{topicId}",
        "communication",
        "topics.retrieve",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/topics/by_slug/{topicSlug}",
        "communication",
        "topics.bySlug.retrieve",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/topics/{topicId}/replies",
        "communication",
        "topics.replies.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/tags",
        "communication",
        "tags.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/forum/v3/api/sites/{siteSlug}/search",
        "communication",
        "search.query",
    ),
];

pub fn open_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
