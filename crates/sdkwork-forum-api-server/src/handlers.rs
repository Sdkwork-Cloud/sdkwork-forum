use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sdkwork_communication_forum_service::domain::commands::*;
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;
use super::dto::{ApiResponse, CreateTopicRequest, CreateReplyRequest};
use super::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub board_id: Option<i64>,
    pub limit: Option<u16>,
}

fn make_ctx() -> ForumRequestContext {
    ForumRequestContext::new(1, 0, 1)
}

pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "sdkwork-forum",
        "version": "0.1.0"
    }))
}

pub async fn list_boards(
    State(state): State<AppState>,
) -> Json<ApiResponse<Value>> {
    let ctx = make_ctx();
    let command = ListNodeTreeCommand {
        space_id: None,
        parent_id: None,
    };

    match state.service_host.service().list_node_tree(&ctx, command) {
        Ok(nodes) => {
            let boards: Vec<Value> = nodes.iter()
                .filter(|n| n.node_type == "board")
                .map(|n| json!({
                    "id": n.id,
                    "uuid": n.uuid,
                    "name": n.name,
                    "description": n.description,
                    "slug": n.slug,
                    "nodeType": n.node_type
                }))
                .collect();
            Json(ApiResponse::ok(json!({ "items": boards })))
        }
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn list_topics(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Json<ApiResponse<Value>> {
    let ctx = make_ctx();
    let command = ListTopicsCommand {
        board_id: query.board_id,
        cursor: None,
        limit: query.limit.unwrap_or(20),
        sort: Some("latest".to_string()),
        status_filter: None,
    };

    match state.service_host.service().list_topics(&ctx, command) {
        Ok(topics) => Json(ApiResponse::ok(json!({
            "items": topics.items,
            "nextCursor": topics.next_cursor,
            "hasMore": topics.has_more
        }))),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn create_topic(
    State(state): State<AppState>,
    Json(req): Json<CreateTopicRequest>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let ctx = make_ctx();
    let command = CreateTopicCommand {
        board_id: req.board_id,
        title: req.title,
        body_format: req.body_format.unwrap_or_else(|| "markdown".to_string()),
        body: req.body,
        tag_ids: vec![],
        prefix_id: None,
        topic_type: req.topic_type,
        visibility: req.visibility,
    };

    match state.service_host.service().create_topic(&ctx, command) {
        Ok(topic) => Ok(Json(ApiResponse::ok(json!(topic)))),
        Err(e) => Ok(Json(ApiResponse::err(e.to_string()))),
    }
}

pub async fn retrieve_topic(
    State(state): State<AppState>,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    let ctx = make_ctx();

    match state.service_host.service().retrieve_topic(&ctx, topic_id) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn list_replies(
    State(state): State<AppState>,
    Path(topic_id): Path<i64>,
    Query(query): Query<ListQuery>,
) -> Json<ApiResponse<Value>> {
    let ctx = make_ctx();
    let command = ListRepliesCommand {
        topic_id,
        cursor: None,
        limit: query.limit.unwrap_or(20),
    };

    match state.service_host.service().list_replies(&ctx, command) {
        Ok(replies) => Json(ApiResponse::ok(json!({
            "items": replies.items,
            "nextCursor": replies.next_cursor,
            "hasMore": replies.has_more
        }))),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn create_reply(
    State(state): State<AppState>,
    Path(topic_id): Path<i64>,
    Json(req): Json<CreateReplyRequest>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let ctx = make_ctx();
    let command = CreateReplyCommand {
        topic_id,
        parent_reply_id: req.parent_reply_id,
        body_format: req.body_format.unwrap_or_else(|| "markdown".to_string()),
        body: req.body,
    };

    match state.service_host.service().create_reply(&ctx, command) {
        Ok(reply) => Ok(Json(ApiResponse::ok(json!(reply)))),
        Err(e) => Ok(Json(ApiResponse::err(e.to_string()))),
    }
}

pub async fn vote_topic(
    State(state): State<AppState>,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    let ctx = make_ctx();
    let command = CreateVoteCommand {
        target_type: "topic".to_string(),
        target_id: topic_id,
        vote_value: 1,
        reason_code: None,
    };

    match state.service_host.service().create_vote(&ctx, command) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn bookmark_topic(
    State(state): State<AppState>,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    let ctx = make_ctx();
    let command = UpdateBookmarkCommand {
        target_type: "topic".to_string(),
        target_id: topic_id,
        note: None,
    };

    match state.service_host.service().update_bookmark(&ctx, command) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
