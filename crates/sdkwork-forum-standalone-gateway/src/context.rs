use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
};
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;

use crate::auth::parse_access_token_header;

const DEFAULT_IAM_TENANT_ID: i64 = 100_001;
const DEFAULT_IAM_ORGANIZATION_ID: i64 = 0;
const DEFAULT_IAM_USER_ID: i64 = 1;

#[derive(Clone, Debug)]
pub struct ResolvedForumContext(pub ForumRequestContext);

pub struct ForumCtx(pub ForumRequestContext);

impl<S> FromRequestParts<S> for ForumCtx
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(resolved) = parts.extensions.get::<ResolvedForumContext>() {
            let mut ctx = resolved.0.clone();
            if let Some(request_id) = header_string(&parts.headers, "x-request-id") {
                ctx = ctx.with_request_id(request_id);
            }
            return Ok(ForumCtx(ctx));
        }

        Ok(ForumCtx(build_context(&parts.headers)))
    }
}

pub fn build_context(headers: &HeaderMap) -> ForumRequestContext {
    if let Some(claims) = parse_access_token_header(headers) {
        let mut ctx = ForumRequestContext::new(
            claims.tenant_id,
            claims.organization_id,
            claims.user_id,
        );
        if let Some(request_id) = header_string(headers, "x-request-id") {
            ctx = ctx.with_request_id(request_id);
        }
        return ctx;
    }

    let tenant_id = header_i64(headers, "x-sdkwork-tenant-id")
        .or_else(|| env_i64("SDKWORK_FORUM_DEFAULT_TENANT_ID"))
        .unwrap_or(DEFAULT_IAM_TENANT_ID);
    let organization_id = header_i64(headers, "x-sdkwork-organization-id")
        .or_else(|| env_i64("SDKWORK_FORUM_DEFAULT_ORGANIZATION_ID"))
        .unwrap_or(DEFAULT_IAM_ORGANIZATION_ID);
    let user_id = header_i64(headers, "x-sdkwork-user-id")
        .or_else(|| env_i64("SDKWORK_FORUM_DEFAULT_USER_ID"))
        .unwrap_or(DEFAULT_IAM_USER_ID);

    let mut ctx = ForumRequestContext::new(tenant_id, organization_id, user_id);
    if let Some(request_id) = header_string(headers, "x-request-id") {
        ctx = ctx.with_request_id(request_id);
    }
    ctx
}

fn header_i64(headers: &HeaderMap, name: &str) -> Option<i64> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
}

fn header_string(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

fn env_i64(name: &str) -> Option<i64> {
    std::env::var(name).ok()?.parse().ok()
}

pub fn page_json<T: serde::Serialize>(
    page: &sdkwork_communication_forum_service::domain::results::CursorPage<T>,
) -> serde_json::Value {
    serde_json::json!({
        "items": page.items,
        "nextCursor": page.next_cursor,
        "hasMore": page.has_more
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn build_context_prefers_access_token_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Access-Token",
            HeaderValue::from_static("tenant_id=100001;organization_id=0;user_id=1"),
        );

        let ctx = build_context(&headers);
        assert_eq!(ctx.tenant_id_value(), 100_001);
        assert_eq!(ctx.organization_id_value(), 0);
        assert_eq!(ctx.user_id_value(), 1);
    }
}
