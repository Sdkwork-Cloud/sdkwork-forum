use axum::{
    extract::Path,
    response::Json,
};
use serde_json::{json, Value};

use crate::dto::{ApiResponse, CreateReplyRequest, CreateTopicRequest};

pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "sdkwork-forum",
        "version": "0.1.0"
    }))
}

pub async fn list_topics() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "items": [
            {
                "id": 1,
                "uuid": "550e8400-e29b-41d4-a716-446655440001",
                "title": "Welcome to SDKWork Forum",
                "body": "This is the first topic in our forum. Feel free to discuss anything related to SDKWork!",
                "body_format": "markdown",
                "topic_type": "discussion",
                "moderation_status": "visible",
                "visibility": "public",
                "board_id": 1,
                "author_user_id": 1,
                "reply_count": 5,
                "view_count": 120,
                "vote_score": 15,
                "created_at": "2026-06-13T10:00:00Z",
                "updated_at": "2026-06-13T10:00:00Z",
                "last_activity_at": "2026-06-13T12:30:00Z"
            },
            {
                "id": 2,
                "uuid": "550e8400-e29b-41d4-a716-446655440002",
                "title": "How to integrate SDK with your app",
                "body": "A comprehensive guide on integrating SDKWork SDKs into your application.",
                "body_format": "markdown",
                "topic_type": "article",
                "moderation_status": "visible",
                "visibility": "public",
                "board_id": 1,
                "author_user_id": 2,
                "reply_count": 12,
                "view_count": 350,
                "vote_score": 42,
                "created_at": "2026-06-12T15:00:00Z",
                "updated_at": "2026-06-12T15:00:00Z",
                "last_activity_at": "2026-06-13T09:15:00Z"
            },
            {
                "id": 3,
                "uuid": "550e8400-e29b-41d4-a716-446655440003",
                "title": "Best practices for API design",
                "body": "Let's discuss the best practices for designing RESTful APIs.",
                "body_format": "markdown",
                "topic_type": "discussion",
                "moderation_status": "visible",
                "visibility": "public",
                "board_id": 2,
                "author_user_id": 3,
                "reply_count": 8,
                "view_count": 200,
                "vote_score": 28,
                "created_at": "2026-06-11T08:00:00Z",
                "updated_at": "2026-06-11T08:00:00Z",
                "last_activity_at": "2026-06-12T16:45:00Z"
            }
        ],
        "next_cursor": null,
        "has_more": false
    })))
}

pub async fn create_topic(Json(req): Json<CreateTopicRequest>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "id": 4,
        "uuid": "550e8400-e29b-41d4-a716-446655440004",
        "title": req.title,
        "body": req.body,
        "body_format": req.body_format.unwrap_or_else(|| "markdown".to_string()),
        "topic_type": req.topic_type.unwrap_or_else(|| "discussion".to_string()),
        "moderation_status": "visible",
        "visibility": req.visibility.unwrap_or_else(|| "public".to_string()),
        "board_id": req.board_id,
        "author_user_id": 1,
        "reply_count": 0,
        "view_count": 0,
        "vote_score": 0,
        "created_at": "2026-06-13T14:00:00Z",
        "updated_at": "2026-06-13T14:00:00Z",
        "last_activity_at": "2026-06-13T14:00:00Z"
    })))
}

pub async fn retrieve_topic(Path(topic_id): Path<i64>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "id": topic_id,
        "uuid": format!("550e8400-e29b-41d4-a716-44665544000{}", topic_id),
        "title": "Welcome to SDKWork Forum",
        "body": "# Welcome\n\nThis is the first topic in our forum. Feel free to discuss anything related to SDKWork!\n\n## Getting Started\n\n1. Create an account\n2. Browse topics\n3. Join discussions\n4. Share your knowledge\n\n## Community Guidelines\n\n- Be respectful\n- Stay on topic\n- Help others\n- Share knowledge",
        "body_format": "markdown",
        "topic_type": "discussion",
        "moderation_status": "visible",
        "visibility": "public",
        "board_id": 1,
        "author_user_id": 1,
        "reply_count": 5,
        "view_count": 120,
        "vote_score": 15,
        "created_at": "2026-06-13T10:00:00Z",
        "updated_at": "2026-06-13T10:00:00Z",
        "last_activity_at": "2026-06-13T12:30:00Z"
    })))
}

pub async fn list_replies(Path(topic_id): Path<i64>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "items": [
            {
                "id": 1,
                "uuid": "660e8400-e29b-41d4-a716-446655440001",
                "topic_id": topic_id,
                "body": "Great topic! Thanks for sharing.",
                "body_format": "markdown",
                "reply_no": 1,
                "author_user_id": 2,
                "moderation_status": "visible",
                "vote_score": 5,
                "created_at": "2026-06-13T10:30:00Z"
            },
            {
                "id": 2,
                "uuid": "660e8400-e29b-41d4-a716-446655440002",
                "topic_id": topic_id,
                "body": "I agree with the previous comment. This is very helpful!",
                "body_format": "markdown",
                "reply_no": 2,
                "author_user_id": 3,
                "moderation_status": "visible",
                "vote_score": 3,
                "created_at": "2026-06-13T11:00:00Z"
            },
            {
                "id": 3,
                "uuid": "660e8400-e29b-41d4-a716-446655440003",
                "topic_id": topic_id,
                "body": "Has anyone tried the new SDK integration? It works great!",
                "body_format": "markdown",
                "reply_no": 3,
                "author_user_id": 4,
                "moderation_status": "visible",
                "vote_score": 8,
                "created_at": "2026-06-13T11:30:00Z"
            }
        ],
        "next_cursor": null,
        "has_more": false
    })))
}

pub async fn create_reply(Path(topic_id): Path<i64>, Json(req): Json<CreateReplyRequest>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "id": 4,
        "uuid": "660e8400-e29b-41d4-a716-446655440004",
        "topic_id": topic_id,
        "body": req.body,
        "body_format": req.body_format.unwrap_or_else(|| "markdown".to_string()),
        "reply_no": 4,
        "author_user_id": 1,
        "moderation_status": "visible",
        "vote_score": 0,
        "created_at": "2026-06-13T14:00:00Z"
    })))
}

pub async fn list_boards() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "items": [
            {
                "id": 1,
                "uuid": "770e8400-e29b-41d4-a716-446655440001",
                "name": "General Discussion",
                "description": "General topics about SDKWork",
                "slug": "general",
                "node_type": "board",
                "topic_count": 25,
                "reply_count": 150
            },
            {
                "id": 2,
                "uuid": "770e8400-e29b-41d4-a716-446655440002",
                "name": "API Design",
                "description": "Discuss API design patterns and best practices",
                "slug": "api-design",
                "node_type": "board",
                "topic_count": 18,
                "reply_count": 95
            },
            {
                "id": 3,
                "uuid": "770e8400-e29b-41d4-a716-446655440003",
                "name": "SDK Integration",
                "description": "Help with SDK integration",
                "slug": "sdk-integration",
                "node_type": "board",
                "topic_count": 32,
                "reply_count": 210
            }
        ]
    })))
}

pub async fn vote_topic(Path(topic_id): Path<i64>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "success": true,
        "topic_id": topic_id,
        "new_vote_score": 16
    })))
}

pub async fn bookmark_topic(Path(topic_id): Path<i64>) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(json!({
        "success": true,
        "topic_id": topic_id,
        "bookmarked": true
    })))
}
