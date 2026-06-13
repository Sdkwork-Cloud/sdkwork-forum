use super::SqlxForumRepository;
use sdkwork_communication_forum_service::domain::commands::*;
use sdkwork_communication_forum_service::domain::models::*;
use sdkwork_communication_forum_service::domain::results::*;
use sdkwork_communication_forum_service::ports::repository::ForumRepository;
use sdkwork_communication_forum_service::value_objects::ForumRequestContext;
use sdkwork_communication_forum_service::ForumServiceError;
use sqlx::Row;
use uuid::Uuid;
use md5::{Md5, Digest};

macro_rules! run_db {
    ($block:expr) => {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on($block)
        })
    };
}

fn parse_cursor(cursor: &Option<String>) -> i64 {
    cursor.as_deref().unwrap_or("0").parse::<i64>().unwrap_or(0).max(0)
}

fn compute_hash(body: &str) -> String {
    let mut h = Md5::new();
    h.update(body.as_bytes());
    format!("{:x}", h.finalize())
}

fn compute_excerpt(body: &str) -> Option<String> {
    let s: String = body.chars().take(500).collect();
    if s.is_empty() { None } else { Some(s) }
}

fn fmt_ts(dt: chrono::DateTime<chrono::Utc>) -> String {
    dt.to_rfc3339()
}

fn fmt_opt_ts(opt: Option<chrono::DateTime<chrono::Utc>>) -> Option<String> {
    opt.map(|d| d.to_rfc3339())
}

fn fmt_json(v: serde_json::Value) -> String {
    v.to_string()
}

fn row_to_topic(row: &sqlx::postgres::PgRow) -> ForumTopic {
    ForumTopic {
        id: row.get("id"),
        uuid: row.get("uuid"),
        space_id: row.get("space_id"),
        board_id: row.get("board_id"),
        author_user_id: row.get("author_user_id"),
        prefix_id: row.get("prefix_id"),
        slug: row.get("slug"),
        title: row.get("title"),
        body_format: row.get("body_format"),
        body: row.get("body"),
        body_excerpt: row.get("body_excerpt"),
        content_hash: row.get("content_hash"),
        topic_type: row.get("topic_type"),
        moderation_status: row.get("moderation_status"),
        visibility: row.get("visibility"),
        pinned_at: fmt_opt_ts(row.get("pinned_at")),
        featured_at: fmt_opt_ts(row.get("featured_at")),
        locked_at: fmt_opt_ts(row.get("locked_at")),
        locked_by: row.get("locked_by"),
        last_reply_id: row.get("last_reply_id"),
        last_activity_at: fmt_ts(row.get("last_activity_at")),
        accepted_reply_id: row.get("accepted_reply_id"),
        attachment_count: row.get("attachment_count"),
        metadata: fmt_json(row.get("metadata")),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        deleted_at: fmt_opt_ts(row.get("deleted_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        deleted_by: row.get("deleted_by"),
    }
}

fn row_to_reply(row: &sqlx::postgres::PgRow) -> ForumReply {
    ForumReply {
        id: row.get("id"),
        uuid: row.get("uuid"),
        topic_id: row.get("topic_id"),
        board_id: row.get("board_id"),
        parent_reply_id: row.get("parent_reply_id"),
        author_user_id: row.get("author_user_id"),
        reply_no: row.get("reply_no"),
        body_format: row.get("body_format"),
        body: row.get("body"),
        body_excerpt: row.get("body_excerpt"),
        content_hash: row.get("content_hash"),
        moderation_status: row.get("moderation_status"),
        accepted_at: fmt_opt_ts(row.get("accepted_at")),
        accepted_by: row.get("accepted_by"),
        attachment_count: row.get("attachment_count"),
        metadata: fmt_json(row.get("metadata")),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        deleted_at: fmt_opt_ts(row.get("deleted_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        deleted_by: row.get("deleted_by"),
    }
}

fn row_to_node(row: &sqlx::postgres::PgRow) -> ForumNode {
    ForumNode {
        id: row.get("id"),
        uuid: row.get("uuid"),
        space_id: row.get("space_id"),
        parent_id: row.get("parent_id"),
        node_type: row.get("node_type"),
        slug: row.get("slug"),
        name: row.get("name"),
        description: row.get("description"),
        path: row.get("path"),
        level_no: row.get("level_no"),
        sort_order: row.get("sort_order"),
        status: row.get("status"),
        settings: fmt_json(row.get("settings")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        deleted_at: fmt_opt_ts(row.get("deleted_at")),
        deleted_by: row.get("deleted_by"),
    }
}

fn row_to_sanction(row: &sqlx::postgres::PgRow) -> ForumSanction {
    ForumSanction {
        id: row.get("id"),
        uuid: row.get("uuid"),
        user_id: row.get("user_id"),
        case_id: row.get("case_id"),
        decision_id: row.get("decision_id"),
        sanction_type: row.get("sanction_type"),
        reason_code: row.get("reason_code"),
        starts_at: fmt_ts(row.get("starts_at")),
        expires_at: fmt_opt_ts(row.get("expires_at")),
        lifted_at: fmt_opt_ts(row.get("lifted_at")),
        lifted_by: row.get("lifted_by"),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        deleted_at: fmt_opt_ts(row.get("deleted_at")),
        deleted_by: row.get("deleted_by"),
    }
}

fn row_to_idempotency_record(row: &sqlx::postgres::PgRow) -> ForumIdempotencyRecord {
    ForumIdempotencyRecord {
        id: row.get("id"),
        uuid: row.get("uuid"),
        idempotency_key: row.get("idempotency_key"),
        request_hash: row.get("request_hash"),
        operation_id: row.get("operation_id"),
        principal_id: row.get("principal_id"),
        response_status: row.get("response_status"),
        response_body_json: row.get("response_body_json"),
        expires_at: fmt_ts(row.get("expires_at")),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
    }
}

fn row_to_topic_stats(row: &sqlx::postgres::PgRow) -> ForumTopicStats {
    ForumTopicStats {
        id: row.get("id"),
        uuid: row.get("uuid"),
        topic_id: row.get("topic_id"),
        reply_count: row.get("reply_count"),
        view_count: row.get("view_count"),
        reaction_count: row.get("reaction_count"),
        vote_score: row.get("vote_score"),
        bookmark_count: row.get("bookmark_count"),
        report_count: row.get("report_count"),
        last_calculated_at: fmt_ts(row.get("last_calculated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
    }
}

fn row_to_board_stats(row: &sqlx::postgres::PgRow) -> ForumBoardStats {
    ForumBoardStats {
        id: row.get("id"),
        uuid: row.get("uuid"),
        board_id: row.get("board_id"),
        topic_count: row.get("topic_count"),
        reply_count: row.get("reply_count"),
        member_count: row.get("member_count"),
        last_topic_id: row.get("last_topic_id"),
        last_reply_id: row.get("last_reply_id"),
        last_activity_at: fmt_opt_ts(row.get("last_activity_at")),
        last_calculated_at: fmt_ts(row.get("last_calculated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
    }
}

fn row_to_member_stats(row: &sqlx::postgres::PgRow) -> ForumMemberStats {
    ForumMemberStats {
        id: row.get("id"),
        uuid: row.get("uuid"),
        user_id: row.get("user_id"),
        topic_count: row.get("topic_count"),
        reply_count: row.get("reply_count"),
        accepted_answer_count: row.get("accepted_answer_count"),
        reaction_received_count: row.get("reaction_received_count"),
        vote_score_received: row.get("vote_score_received"),
        last_activity_at: fmt_opt_ts(row.get("last_activity_at")),
        last_calculated_at: fmt_ts(row.get("last_calculated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
    }
}

fn row_to_notification_preference(row: &sqlx::postgres::PgRow) -> ForumNotificationPreference {
    ForumNotificationPreference {
        id: row.get("id"),
        uuid: row.get("uuid"),
        user_id: row.get("user_id"),
        event_type: row.get("event_type"),
        channel: row.get("channel"),
        enabled: row.get("enabled"),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
    }
}

fn row_to_space(row: &sqlx::postgres::PgRow) -> ForumSpace {
    ForumSpace {
        id: row.get("id"),
        uuid: row.get("uuid"),
        code: row.get("code"),
        slug: row.get("slug"),
        name: row.get("name"),
        description: row.get("description"),
        visibility: row.get("visibility"),
        default_locale: row.get("default_locale"),
        settings: fmt_json(row.get("settings")),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
        deleted_at: fmt_opt_ts(row.get("deleted_at")),
        deleted_by: row.get("deleted_by"),
    }
}

fn row_to_attachment(row: &sqlx::postgres::PgRow) -> ForumAttachment {
    ForumAttachment {
        id: row.get("id"),
        uuid: row.get("uuid"),
        owner_type: row.get("owner_type"),
        owner_id: row.get("owner_id"),
        drive_space_id: row.get("drive_space_id"),
        drive_node_id: row.get("drive_node_id"),
        media_resource_id: row.get("media_resource_id"),
        file_name: row.get("file_name"),
        mime_type: row.get("mime_type"),
        byte_size: row.get("byte_size"),
        sort_order: row.get("sort_order"),
        scan_status: row.get("scan_status"),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
    }
}

fn row_to_subscription(row: &sqlx::postgres::PgRow) -> ForumSubscription {
    ForumSubscription {
        id: row.get("id"),
        uuid: row.get("uuid"),
        target_type: row.get("target_type"),
        target_id: row.get("target_id"),
        user_id: row.get("user_id"),
        notify_level: row.get("notify_level"),
        delivery_channels: row.get("delivery_channels"),
        status: row.get("status"),
        version: row.get("version"),
        created_at: fmt_ts(row.get("created_at")),
        updated_at: fmt_ts(row.get("updated_at")),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        data_scope: row.get("data_scope"),
    }
}

impl ForumRepository for SqlxForumRepository {
    fn list_node_tree(&self, ctx: &ForumRequestContext, command: &ListNodeTreeCommand) -> Result<NodeTreeResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let rows = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_node
                 WHERE tenant_id = $1
                   AND deleted_at IS NULL
                   AND ($2::bigint IS NULL OR space_id = $2)
                   AND ($3::bigint IS NULL OR parent_id = $3)
                 ORDER BY sort_order ASC"
            )
            .bind(tenant_id)
            .bind(command.space_id)
            .bind(command.parent_id)
            .fetch_all(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(rows.iter().map(row_to_node).collect())
    }

    fn list_topics(&self, ctx: &ForumRequestContext, command: &ListTopicsCommand) -> Result<TopicPageResult, ForumServiceError> {
        let offset = parse_cursor(&command.cursor);
        let limit = command.limit.max(1) as i64;
        let tenant_id = ctx.tenant_id_value();
        let board_filter = command.board_id;
        let status_filter = command.status_filter.clone();

        let rows = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_topic
                 WHERE tenant_id = $1
                   AND deleted_at IS NULL
                   AND ($2::bigint IS NULL OR board_id = $2)
                   AND ($3::text IS NULL OR moderation_status = $3)
                 ORDER BY last_activity_at DESC
                 LIMIT $4 OFFSET $5"
            )
            .bind(tenant_id)
            .bind(board_filter)
            .bind(status_filter.as_deref())
            .bind(limit + 1)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        let has_more = rows.len() as i64 > limit;
        let items: Vec<ForumTopic> = rows.iter().take(limit as usize).map(row_to_topic).collect();
        let next_cursor = if has_more { Some((offset + limit).to_string()) } else { None };
        Ok(CursorPage::new(items, next_cursor, has_more))
    }

    fn create_topic(&self, ctx: &ForumRequestContext, command: &CreateTopicCommand) -> Result<ForumTopic, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();

        let space_id: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT space_id FROM forum_node WHERE id = $1 AND tenant_id = $2 AND node_type = 'board' AND deleted_at IS NULL"
            )
            .bind(command.board_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("board", command.board_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;

        let uuid = Uuid::new_v4().to_string();
        let hash = compute_hash(&command.body);
        let excerpt = compute_excerpt(&command.body);
        let topic_type = command.topic_type.as_deref().unwrap_or("discussion");
        let visibility = command.visibility.as_deref().unwrap_or("public");

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_topic (
                    uuid, space_id, board_id, author_user_id, prefix_id, title, body_format,
                    body, body_excerpt, content_hash, topic_type, moderation_status, visibility,
                    attachment_count, metadata, status, version, created_at, updated_at,
                    last_activity_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'visible', $12,
                    0, $13, 'active', 1, NOW(), NOW(), NOW(), $14, $15, 'default'
                ) RETURNING *"
            )
            .bind(&uuid)
            .bind(space_id)
            .bind(command.board_id)
            .bind(user_id)
            .bind(command.prefix_id)
            .bind(&command.title)
            .bind(&command.body_format)
            .bind(&command.body)
            .bind(excerpt.as_deref())
            .bind(&hash)
            .bind(topic_type)
            .bind(visibility)
            .bind(serde_json::json!({}))
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        Ok(row_to_topic(&row))
    }

    fn retrieve_topic(&self, ctx: &ForumRequestContext, topic_id: i64) -> Result<ForumTopic, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_topic WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_topic(&row))
    }

    fn update_topic(&self, ctx: &ForumRequestContext, command: &UpdateTopicCommand) -> Result<ForumTopic, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let new_hash = command.body.as_deref().map(compute_hash);
        let new_excerpt = command.body.as_ref().and_then(|b| compute_excerpt(b));

        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET title = COALESCE($1, title),
                     body = COALESCE($2, body),
                     body_format = COALESCE($3, body_format),
                     content_hash = COALESCE($6, content_hash),
                     body_excerpt = COALESCE($7, body_excerpt),
                     version = version + 1,
                     updated_at = NOW()
                 WHERE id = $4 AND tenant_id = $5 AND deleted_at IS NULL
                 RETURNING *"
            )
            .bind(command.title.as_deref())
            .bind(command.body.as_deref())
            .bind(command.body_format.as_deref())
            .bind(command.topic_id)
            .bind(tenant_id)
            .bind(new_hash.as_deref())
            .bind(new_excerpt.as_deref())
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_topic(&row))
    }

    fn delete_topic(&self, ctx: &ForumRequestContext, command: &DeleteTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let user_id = ctx.user_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET deleted_at = NOW(), deleted_by = $1, status = 'deleted'
                 WHERE id = $2 AND tenant_id = $3 AND deleted_at IS NULL
                 RETURNING id, uuid"
            )
            .bind(user_id)
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn list_replies(&self, ctx: &ForumRequestContext, command: &ListRepliesCommand) -> Result<ReplyPageResult, ForumServiceError> {
        let offset = parse_cursor(&command.cursor);
        let limit = command.limit.max(1) as i64;
        let tenant_id = ctx.tenant_id_value();

        let rows = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_topic_reply
                 WHERE topic_id = $1 AND tenant_id = $2 AND deleted_at IS NULL
                 ORDER BY reply_no ASC
                 LIMIT $3 OFFSET $4"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .bind(limit + 1)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        let has_more = rows.len() as i64 > limit;
        let items: Vec<ForumReply> = rows.iter().take(limit as usize).map(row_to_reply).collect();
        let next_cursor = if has_more { Some((offset + limit).to_string()) } else { None };
        Ok(CursorPage::new(items, next_cursor, has_more))
    }

    fn create_reply(&self, ctx: &ForumRequestContext, command: &CreateReplyCommand) -> Result<ForumReply, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();

        let board_id: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT board_id FROM forum_topic WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;

        let reply_no: i32 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COALESCE(MAX(reply_no), 0) + 1 FROM forum_topic_reply WHERE topic_id = $1 AND tenant_id = $2"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        let uuid = Uuid::new_v4().to_string();
        let hash = compute_hash(&command.body);
        let excerpt = compute_excerpt(&command.body);

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_topic_reply (
                    uuid, topic_id, board_id, parent_reply_id, author_user_id, reply_no,
                    body_format, body, body_excerpt, content_hash, moderation_status,
                    attachment_count, metadata, status, version, created_at, updated_at,
                    tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'visible',
                    0, $11, 'active', 1, NOW(), NOW(), $12, $13, 'default'
                ) RETURNING *"
            )
            .bind(&uuid)
            .bind(command.topic_id)
            .bind(board_id)
            .bind(command.parent_reply_id)
            .bind(user_id)
            .bind(reply_no)
            .bind(&command.body_format)
            .bind(&command.body)
            .bind(excerpt.as_deref())
            .bind(&hash)
            .bind(serde_json::json!({}))
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        Ok(row_to_reply(&row))
    }

    fn update_reply(&self, ctx: &ForumRequestContext, command: &UpdateReplyCommand) -> Result<ForumReply, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic_reply
                 SET body = COALESCE($1, body),
                     body_format = COALESCE($2, body_format),
                     version = version + 1,
                     updated_at = NOW()
                 WHERE id = $3 AND tenant_id = $4 AND deleted_at IS NULL
                 RETURNING *"
            )
            .bind(command.body.as_deref())
            .bind(command.body_format.as_deref())
            .bind(command.reply_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("reply", command.reply_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_reply(&row))
    }

    fn delete_reply(&self, ctx: &ForumRequestContext, command: &DeleteReplyCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let user_id = ctx.user_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic_reply
                 SET deleted_at = NOW(), deleted_by = $1, status = 'deleted'
                 WHERE id = $2 AND tenant_id = $3 AND deleted_at IS NULL
                 RETURNING id, uuid"
            )
            .bind(user_id)
            .bind(command.reply_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("reply", command.reply_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn accept_reply(&self, ctx: &ForumRequestContext, command: &AcceptReplyCommand) -> Result<ForumTopic, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET accepted_reply_id = $1
                 WHERE id = $2 AND tenant_id = $3 AND topic_type = 'question'
                 RETURNING *"
            )
            .bind(command.reply_id)
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_topic(&row))
    }

    fn clear_accepted_reply(&self, ctx: &ForumRequestContext, command: &ClearAcceptedReplyCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET accepted_reply_id = NULL
                 WHERE id = $1 AND tenant_id = $2
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn create_report(&self, _ctx: &ForumRequestContext, _command: &CreateReportCommand) -> Result<CommandResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_report"))
    }

    fn list_feed(&self, _ctx: &ForumRequestContext, _command: &ListFeedCommand) -> Result<FeedPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_feed"))
    }

    fn query_search(&self, _ctx: &ForumRequestContext, _command: &QuerySearchCommand) -> Result<SearchResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.query_search"))
    }

    fn list_moderation_queue(&self, _ctx: &ForumRequestContext, _command: &ListModerationQueueCommand) -> Result<ModerationQueueResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_moderation_queue"))
    }

    fn create_moderation_decision(&self, _ctx: &ForumRequestContext, _command: &CreateModerationDecisionCommand) -> Result<ModerationDecisionResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_moderation_decision"))
    }

    fn rebuild_search_projection(&self, _ctx: &ForumRequestContext, _command: &RebuildSearchProjectionCommand) -> Result<CommandResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.rebuild_search_projection"))
    }

    fn list_topic_revisions(&self, _ctx: &ForumRequestContext, _command: &ListTopicRevisionsCommand) -> Result<TopicRevisionPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_topic_revisions"))
    }

    fn list_reply_revisions(&self, _ctx: &ForumRequestContext, _command: &ListReplyRevisionsCommand) -> Result<ReplyRevisionPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_reply_revisions"))
    }

    fn create_poll_vote(&self, _ctx: &ForumRequestContext, _command: &CreatePollVoteCommand) -> Result<CommandResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_poll_vote"))
    }

    fn create_reaction(&self, ctx: &ForumRequestContext, command: &CreateReactionCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_reaction (
                    uuid, target_type, target_id, actor_user_id, reaction_type,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5,
                    'active', 1, NOW(), NOW(), $6, $7, 'default'
                ) RETURNING id, uuid"
            )
            .bind(&uuid)
            .bind(&command.target_type)
            .bind(command.target_id)
            .bind(user_id)
            .bind(&command.reaction_type)
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn create_vote(&self, ctx: &ForumRequestContext, command: &CreateVoteCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_vote (
                    uuid, target_type, target_id, actor_user_id, vote_value, reason_code,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6,
                    'active', 1, NOW(), NOW(), $7, $8, 'default'
                ) RETURNING id, uuid"
            )
            .bind(&uuid)
            .bind(&command.target_type)
            .bind(command.target_id)
            .bind(user_id)
            .bind(command.vote_value)
            .bind(command.reason_code.as_deref())
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn update_bookmark(&self, ctx: &ForumRequestContext, command: &UpdateBookmarkCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_bookmark (
                    uuid, target_type, target_id, user_id, note,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5,
                    'active', 1, NOW(), NOW(), $6, $7, 'default'
                )
                ON CONFLICT (tenant_id, target_type, target_id, user_id)
                DO UPDATE SET note = EXCLUDED.note, updated_at = NOW()
                RETURNING id, uuid"
            )
            .bind(&uuid)
            .bind(&command.target_type)
            .bind(command.target_id)
            .bind(user_id)
            .bind(command.note.as_deref())
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn update_read_state(&self, ctx: &ForumRequestContext, command: &UpdateReadStateCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_read_state (
                    uuid, topic_id, user_id, last_read_reply_id, last_read_at, unread_count,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, NOW(), 0,
                    'active', 1, NOW(), NOW(), $5, $6, 'default'
                )
                ON CONFLICT (tenant_id, topic_id, user_id)
                DO UPDATE SET last_read_reply_id = EXCLUDED.last_read_reply_id, last_read_at = NOW(), unread_count = 0
                RETURNING id, uuid"
            )
            .bind(&uuid)
            .bind(command.topic_id)
            .bind(user_id)
            .bind(command.last_read_reply_id)
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn pin_topic(&self, ctx: &ForumRequestContext, command: &PinTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET pinned_at = CASE WHEN pinned_at IS NULL THEN NOW() ELSE NULL END,
                     updated_at = NOW()
                 WHERE id = $1 AND tenant_id = $2
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn unpin_topic(&self, ctx: &ForumRequestContext, command: &PinTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET pinned_at = NULL, updated_at = NOW()
                 WHERE id = $1 AND tenant_id = $2 AND pinned_at IS NOT NULL
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn feature_topic(&self, ctx: &ForumRequestContext, command: &FeatureTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET featured_at = CASE WHEN featured_at IS NULL THEN NOW() ELSE NULL END,
                     updated_at = NOW()
                 WHERE id = $1 AND tenant_id = $2
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn unfeature_topic(&self, ctx: &ForumRequestContext, command: &FeatureTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET featured_at = NULL, updated_at = NOW()
                 WHERE id = $1 AND tenant_id = $2 AND featured_at IS NOT NULL
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn lock_topic(&self, ctx: &ForumRequestContext, command: &LockTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let user_id = ctx.user_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET locked_at = CASE WHEN locked_at IS NULL THEN NOW() ELSE NULL END,
                     locked_by = CASE WHEN locked_at IS NULL THEN $2 ELSE NULL END,
                     updated_at = NOW()
                 WHERE id = $1 AND tenant_id = $3
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(user_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn unlock_topic(&self, ctx: &ForumRequestContext, command: &LockTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET locked_at = NULL, locked_by = NULL, updated_at = NOW()
                 WHERE id = $1 AND tenant_id = $2 AND locked_at IS NOT NULL
                 RETURNING id, uuid"
            )
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn move_topic(&self, ctx: &ForumRequestContext, command: &MoveTopicCommand) -> Result<CommandResult, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_topic
                 SET board_id = $1, updated_at = NOW()
                 WHERE id = $2 AND tenant_id = $3
                 RETURNING id, uuid"
            )
            .bind(command.target_board_id)
            .bind(command.topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic", command.topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(CommandResult::success(row.get("id"), row.get::<String, _>("uuid")))
    }

    fn create_node(&self, ctx: &ForumRequestContext, command: &CreateNodeCommand) -> Result<ForumNode, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let uuid = Uuid::new_v4().to_string();

        let (parent_path, parent_level) = if let Some(parent_id) = command.parent_id {
            let prow = run_db!(async {
                sqlx::query("SELECT path, level_no FROM forum_node WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL")
                    .bind(parent_id)
                    .bind(tenant_id)
                    .fetch_one(&self.pool)
                    .await
            }).map_err(|e| match e {
                sqlx::Error::RowNotFound => ForumServiceError::not_found("node", parent_id.to_string()),
                e => ForumServiceError::internal(e.to_string()),
            })?;
            (prow.get::<String, _>("path"), prow.get::<i32, _>("level_no"))
        } else {
            (String::new(), -1i32)
        };

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_node (
                    uuid, space_id, parent_id, node_type, slug, name, description,
                    path, level_no, sort_order, settings, status, version,
                    created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7,
                    '', $8, $9, $10, 'active', 1,
                    NOW(), NOW(), $11, $12, 'default'
                ) RETURNING *"
            )
            .bind(&uuid)
            .bind(command.space_id)
            .bind(command.parent_id)
            .bind(&command.node_type)
            .bind(&command.slug)
            .bind(&command.name)
            .bind(command.description.as_deref())
            .bind(parent_level + 1)
            .bind(command.sort_order)
            .bind(serde_json::json!({}))
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        let node_id: i64 = row.get("id");
        let new_path = if parent_path.is_empty() {
            format!("/{}", node_id)
        } else {
            format!("{}/{}", parent_path, node_id)
        };

        let row = run_db!(async {
            sqlx::query("UPDATE forum_node SET path = $1 WHERE id = $2 AND tenant_id = $3 RETURNING *")
                .bind(&new_path)
                .bind(node_id)
                .bind(tenant_id)
                .fetch_one(&self.pool)
                .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        Ok(row_to_node(&row))
    }

    fn update_node(&self, _ctx: &ForumRequestContext, _command: &UpdateNodeCommand) -> Result<ForumNode, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.update_node"))
    }

    fn delete_node(&self, _ctx: &ForumRequestContext, _command: &DeleteNodeCommand) -> Result<CommandResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.delete_node"))
    }

    fn list_moderation_cases(&self, _ctx: &ForumRequestContext, _command: &ListModerationCasesCommand) -> Result<ModerationCasePageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_moderation_cases"))
    }

    fn create_moderation_case(&self, _ctx: &ForumRequestContext, _command: &CreateModerationCaseCommand) -> Result<ForumModerationCase, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_moderation_case"))
    }

    fn retrieve_moderation_case(&self, _ctx: &ForumRequestContext, _command: &RetrieveModerationCaseCommand) -> Result<ForumModerationCase, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.retrieve_moderation_case"))
    }

    fn list_sanctions(&self, _ctx: &ForumRequestContext, _command: &ListSanctionsCommand) -> Result<SanctionPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_sanctions"))
    }

    fn create_sanction(&self, _ctx: &ForumRequestContext, _command: &CreateSanctionCommand) -> Result<ForumSanction, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_sanction"))
    }

    fn update_sanction(&self, _ctx: &ForumRequestContext, _command: &UpdateSanctionCommand) -> Result<ForumSanction, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.update_sanction"))
    }

    fn list_reputation_rules(&self, _ctx: &ForumRequestContext, _command: &ListReputationRulesCommand) -> Result<ReputationRulePageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_reputation_rules"))
    }

    fn create_reputation_rule(&self, _ctx: &ForumRequestContext, _command: &CreateReputationRuleCommand) -> Result<ForumReputationRule, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_reputation_rule"))
    }

    fn list_reputation_ledger(&self, _ctx: &ForumRequestContext, _command: &ListReputationLedgerCommand) -> Result<ReputationLedgerPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_reputation_ledger"))
    }

    fn list_trust_levels(&self, _ctx: &ForumRequestContext, _command: &ListTrustLevelsCommand) -> Result<TrustLevelPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_trust_levels"))
    }

    fn create_trust_level(&self, _ctx: &ForumRequestContext, _command: &CreateTrustLevelCommand) -> Result<ForumTrustLevel, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_trust_level"))
    }

    fn list_badges(&self, _ctx: &ForumRequestContext, _command: &ListBadgesCommand) -> Result<BadgePageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_badges"))
    }

    fn create_badge(&self, _ctx: &ForumRequestContext, _command: &CreateBadgeCommand) -> Result<ForumBadge, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_badge"))
    }

    fn list_board_stats(&self, _ctx: &ForumRequestContext, _command: &ListBoardStatsCommand) -> Result<BoardStatsPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_board_stats"))
    }

    fn list_topic_stats(&self, _ctx: &ForumRequestContext, _command: &ListTopicStatsCommand) -> Result<TopicStatsPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_topic_stats"))
    }

    fn create_audit_action(&self, _ctx: &ForumRequestContext, _command: &CreateAuditActionCommand) -> Result<ForumAuditAction, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_audit_action"))
    }

    fn list_topic_prefixes(&self, _ctx: &ForumRequestContext, _command: &ListTopicPrefixesCommand) -> Result<TopicPrefixPageResult, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.list_topic_prefixes"))
    }

    fn create_topic_prefix(&self, _ctx: &ForumRequestContext, _command: &CreateTopicPrefixCommand) -> Result<ForumTopicPrefix, ForumServiceError> {
        Err(ForumServiceError::not_implemented("repository.create_topic_prefix"))
    }

    fn create_space(&self, ctx: &ForumRequestContext, command: &CreateSpaceCommand) -> Result<ForumSpace, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_space (
                    uuid, code, slug, name, description, visibility, default_locale, settings,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8,
                    'active', 1, NOW(), NOW(), $9, $10, 'default'
                ) RETURNING *"
            )
            .bind(&uuid)
            .bind(&command.code)
            .bind(&command.slug)
            .bind(&command.name)
            .bind(command.description.as_deref())
            .bind(&command.visibility)
            .bind(command.default_locale.as_deref())
            .bind(&command.settings)
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        Ok(row_to_space(&row))
    }

    fn update_space(&self, ctx: &ForumRequestContext, command: &UpdateSpaceCommand) -> Result<ForumSpace, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_space
                 SET name = COALESCE($1, name),
                     description = COALESCE($2, description),
                     visibility = COALESCE($3, visibility),
                     version = version + 1,
                     updated_at = NOW()
                 WHERE id = $4 AND tenant_id = $5
                 RETURNING *"
            )
            .bind(command.name.as_deref())
            .bind(command.description.as_deref())
            .bind(command.visibility.as_deref())
            .bind(command.space_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("space", command.space_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_space(&row))
    }

    fn create_attachment(&self, ctx: &ForumRequestContext, command: &CreateAttachmentCommand) -> Result<ForumAttachment, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_attachment (
                    uuid, owner_type, owner_id, drive_space_id, drive_node_id, media_resource_id,
                    file_name, mime_type, byte_size, sort_order, scan_status,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7, $8, $9, $10, 'pending',
                    'active', 1, NOW(), NOW(), $11, $12, 'default'
                ) RETURNING *"
            )
            .bind(&uuid)
            .bind(&command.owner_type)
            .bind(command.owner_id)
            .bind(&command.drive_space_id)
            .bind(&command.drive_node_id)
            .bind(command.media_resource_id.as_deref())
            .bind(&command.file_name)
            .bind(&command.mime_type)
            .bind(command.byte_size)
            .bind(command.sort_order)
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(row_to_attachment(&row))
    }

    fn create_subscription(&self, ctx: &ForumRequestContext, command: &CreateSubscriptionCommand) -> Result<ForumSubscription, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        let user_id = ctx.user_id_value();
        let uuid = Uuid::new_v4().to_string();

        let row = run_db!(async {
            sqlx::query(
                "INSERT INTO forum_subscription (
                    uuid, target_type, target_id, user_id, notify_level, delivery_channels,
                    status, version, created_at, updated_at, tenant_id, organization_id, data_scope
                ) VALUES (
                    $1, $2, $3, $4, $5, $6,
                    'active', 1, NOW(), NOW(), $7, $8, 'default'
                ) RETURNING *"
            )
            .bind(&uuid)
            .bind(&command.target_type)
            .bind(command.target_id)
            .bind(user_id)
            .bind(&command.notify_level)
            .bind(command.delivery_channels.as_deref())
            .bind(tenant_id)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(row_to_subscription(&row))
    }

    fn update_subscription(&self, ctx: &ForumRequestContext, command: &UpdateSubscriptionCommand) -> Result<ForumSubscription, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "UPDATE forum_subscription
                 SET notify_level = COALESCE($1, notify_level),
                     delivery_channels = COALESCE($2, delivery_channels),
                     version = version + 1,
                     updated_at = NOW()
                 WHERE id = $3 AND tenant_id = $4 AND status = 'active'
                 RETURNING *"
            )
            .bind(command.notify_level.as_deref())
            .bind(command.delivery_channels.as_deref())
            .bind(command.subscription_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("subscription", command.subscription_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_subscription(&row))
    }

    fn list_subscriptions(&self, ctx: &ForumRequestContext, command: &ListSubscriptionsCommand) -> Result<SubscriptionPageResult, ForumServiceError> {
        let offset = parse_cursor(&command.cursor);
        let limit = command.limit.max(1) as i64;
        let tenant_id = ctx.tenant_id_value();

        let rows = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_subscription
                 WHERE tenant_id = $1
                   AND ($2::text IS NULL OR target_type = $2)
                   AND status = 'active'
                 ORDER BY created_at DESC
                 LIMIT $3 OFFSET $4"
            )
            .bind(tenant_id)
            .bind(command.target_type.as_deref())
            .bind(limit + 1)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;

        let has_more = rows.len() as i64 > limit;
        let items: Vec<ForumSubscription> = rows.iter().take(limit as usize).map(row_to_subscription).collect();
        let next_cursor = if has_more { Some((offset + limit).to_string()) } else { None };
        Ok(CursorPage::new(items, next_cursor, has_more))
    }

    fn check_space_has_topics(&self, ctx: &ForumRequestContext, space_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_topic WHERE space_id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(space_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_node_cycle(&self, ctx: &ForumRequestContext, node_id: i64, new_parent_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "WITH RECURSIVE ancestors AS (
                    SELECT id, parent_id FROM forum_node WHERE id = $1 AND tenant_id = $2
                    UNION ALL
                    SELECT n.id, n.parent_id FROM forum_node n JOIN ancestors a ON n.id = a.parent_id WHERE n.tenant_id = $2
                ) SELECT COUNT(*) FROM ancestors WHERE id = $3"
            )
            .bind(node_id)
            .bind(tenant_id)
            .bind(new_parent_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_node_is_board(&self, ctx: &ForumRequestContext, node_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let node_type: Option<String> = run_db!(async {
            sqlx::query_scalar(
                "SELECT node_type FROM forum_node WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(node_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("node", node_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(node_type.as_deref() == Some("board"))
    }

    fn check_board_exists(&self, ctx: &ForumRequestContext, board_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_node WHERE id = $1 AND tenant_id = $2 AND node_type = 'board' AND deleted_at IS NULL"
            )
            .bind(board_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_owner_exists(&self, ctx: &ForumRequestContext, owner_type: &str, owner_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let sql = match owner_type {
            "topic" => "SELECT COUNT(*) FROM forum_topic WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
            "reply" => "SELECT COUNT(*) FROM forum_topic_reply WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
            _ => return Err(ForumServiceError::validation(format!("unsupported owner_type: {}", owner_type))),
        };
        let count: i64 = run_db!(async {
            sqlx::query_scalar(sql)
                .bind(owner_id)
                .bind(tenant_id)
                .fetch_one(&self.pool)
                .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_poll_exists(&self, ctx: &ForumRequestContext, poll_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_poll WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(poll_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn count_poll_votes(&self, ctx: &ForumRequestContext, poll_id: i64) -> Result<i64, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_poll_vote WHERE poll_id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(poll_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count)
    }

    fn check_poll_selection_mode(&self, ctx: &ForumRequestContext, poll_id: i64) -> Result<String, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let mode: Option<String> = run_db!(async {
            sqlx::query_scalar(
                "SELECT selection_mode FROM forum_poll WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(poll_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("poll", poll_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(mode.unwrap_or_else(|| "single".to_string()))
    }

    fn check_active_vote(&self, ctx: &ForumRequestContext, target_type: &str, target_id: i64, actor_user_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_vote WHERE target_type = $1 AND target_id = $2 AND actor_user_id = $3 AND tenant_id = $4 AND status = 'active'"
            )
            .bind(target_type)
            .bind(target_id)
            .bind(actor_user_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_active_sanctions(&self, ctx: &ForumRequestContext, user_id: i64) -> Result<Vec<ForumSanction>, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let rows = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_sanction
                 WHERE user_id = $1 AND tenant_id = $2
                   AND status = 'active'
                   AND (expires_at IS NULL OR expires_at > NOW())
                   AND lifted_at IS NULL"
            )
            .bind(user_id)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(rows.iter().map(row_to_sanction).collect())
    }

    fn check_active_appeal(&self, ctx: &ForumRequestContext, sanction_id: Option<i64>, case_id: Option<i64>, appellant_user_id: i64) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_appeal
                 WHERE appellant_user_id = $1 AND tenant_id = $2
                   AND appeal_status IN ('open', 'reviewing')
                   AND ((sanction_id = $3 AND $3 IS NOT NULL) OR (case_id = $4 AND $4 IS NOT NULL))"
            )
            .bind(appellant_user_id)
            .bind(tenant_id)
            .bind(sanction_id)
            .bind(case_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn count_topics_in_space(&self, ctx: &ForumRequestContext, space_id: i64) -> Result<i64, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_topic WHERE space_id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(space_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count)
    }

    fn get_next_revision_no(&self, ctx: &ForumRequestContext, topic_id: i64) -> Result<i32, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let next_no: i32 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COALESCE(MAX(revision_no), 0) + 1 FROM forum_topic_revision WHERE topic_id = $1 AND tenant_id = $2"
            )
            .bind(topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(next_no)
    }

    fn get_next_reply_no(&self, ctx: &ForumRequestContext, topic_id: i64) -> Result<i32, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let next_no: i32 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COALESCE(MAX(reply_no), 0) + 1 FROM forum_topic_reply WHERE topic_id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
            )
            .bind(topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(next_no)
    }

    fn get_next_case_no(&self, _ctx: &ForumRequestContext, tenant_id: i64) -> Result<String, ForumServiceError> {
        let case_no: String = run_db!(async {
            sqlx::query_scalar(
                "SELECT 'MOD-' || TO_CHAR(NOW(), 'YYYY') || '-' || LPAD(COALESCE(MAX(CAST(SUBSTRING(case_no FROM 10) AS INTEGER)), 0) + 1, 4, '0')
                 FROM forum_moderation_case
                 WHERE tenant_id = $1 AND case_no LIKE 'MOD-' || TO_CHAR(NOW(), 'YYYY') || '-%'"
            )
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(case_no)
    }

    fn check_duplicate_queue_item(&self, ctx: &ForumRequestContext, target_type: &str, target_id: i64, source_type: &str) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_moderation_queue_item
                 WHERE tenant_id = $1 AND target_type = $2 AND target_id = $3 AND source_type = $4
                   AND queue_status IN ('open', 'assigned', 'in_review')"
            )
            .bind(tenant_id)
            .bind(target_type)
            .bind(target_id)
            .bind(source_type)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_idempotency_key(&self, ctx: &ForumRequestContext, key: &str, operation_id: &str) -> Result<Option<ForumIdempotencyRecord>, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_idempotency_record
                 WHERE tenant_id = $1 AND idempotency_key = $2 AND operation_id = $3 AND expires_at > NOW()"
            )
            .bind(tenant_id)
            .bind(key)
            .bind(operation_id)
            .fetch_optional(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(row.map(|r| row_to_idempotency_record(&r)))
    }

    fn check_message_id_exists(&self, ctx: &ForumRequestContext, source_system: &str, message_id: &str, consumer_name: &str) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_inbox_event
                 WHERE tenant_id = $1 AND source_system = $2 AND message_id = $3 AND consumer_name = $4"
            )
            .bind(tenant_id)
            .bind(source_system)
            .bind(message_id)
            .bind(consumer_name)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn check_message_payload_hash(&self, ctx: &ForumRequestContext, source_system: &str, message_id: &str, consumer_name: &str, payload_hash: &str) -> Result<bool, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let count: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM forum_inbox_event
                 WHERE tenant_id = $1 AND source_system = $2 AND message_id = $3 AND consumer_name = $4 AND payload_hash = $5"
            )
            .bind(tenant_id)
            .bind(source_system)
            .bind(message_id)
            .bind(consumer_name)
            .bind(payload_hash)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(count > 0)
    }

    fn get_reputation_balance(&self, ctx: &ForumRequestContext, user_id: i64) -> Result<i64, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let balance: i64 = run_db!(async {
            sqlx::query_scalar(
                "SELECT COALESCE(SUM(CASE WHEN direction = 'credit' THEN points ELSE -points END), 0)
                 FROM forum_reputation_ledger WHERE user_id = $1 AND tenant_id = $2"
            )
            .bind(user_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(balance)
    }

    fn get_topic_stats(&self, ctx: &ForumRequestContext, topic_id: i64) -> Result<ForumTopicStats, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_topic_stats WHERE topic_id = $1 AND tenant_id = $2"
            )
            .bind(topic_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("topic_stats", topic_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_topic_stats(&row))
    }

    fn get_board_stats(&self, ctx: &ForumRequestContext, board_id: i64) -> Result<ForumBoardStats, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_board_stats WHERE board_id = $1 AND tenant_id = $2"
            )
            .bind(board_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("board_stats", board_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_board_stats(&row))
    }

    fn get_member_stats(&self, ctx: &ForumRequestContext, user_id: i64) -> Result<ForumMemberStats, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let row = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_member_stats WHERE user_id = $1 AND tenant_id = $2"
            )
            .bind(user_id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
        }).map_err(|e| match e {
            sqlx::Error::RowNotFound => ForumServiceError::not_found("member_stats", user_id.to_string()),
            e => ForumServiceError::internal(e.to_string()),
        })?;
        Ok(row_to_member_stats(&row))
    }

    fn update_tag_usage_count(&self, ctx: &ForumRequestContext, tag_id: i64) -> Result<(), ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        run_db!(async {
            sqlx::query(
                "UPDATE forum_tag SET usage_count = (
                    SELECT COUNT(*) FROM forum_topic_tag WHERE tag_id = $1 AND tenant_id = $2 AND deleted_at IS NULL
                ) WHERE id = $1 AND tenant_id = $2"
            )
            .bind(tag_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(())
    }

    fn update_unread_count(&self, ctx: &ForumRequestContext, topic_id: i64, user_id: i64) -> Result<(), ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        run_db!(async {
            sqlx::query(
                "UPDATE forum_read_state SET unread_count = (
                    SELECT COUNT(*) FROM forum_topic_reply
                    WHERE topic_id = $1
                      AND id > COALESCE((SELECT last_read_reply_id FROM forum_read_state WHERE topic_id = $1 AND user_id = $2 AND tenant_id = $3), 0)
                      AND tenant_id = $3 AND deleted_at IS NULL
                ) WHERE topic_id = $1 AND user_id = $2 AND tenant_id = $3"
            )
            .bind(topic_id)
            .bind(user_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(())
    }

    fn get_notification_preferences(&self, ctx: &ForumRequestContext, user_id: i64, event_type: &str) -> Result<Vec<ForumNotificationPreference>, ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let rows = run_db!(async {
            sqlx::query(
                "SELECT * FROM forum_notification_preference WHERE user_id = $1 AND event_type = $2 AND tenant_id = $3 AND status = 'active'"
            )
            .bind(user_id)
            .bind(event_type)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(rows.iter().map(row_to_notification_preference).collect())
    }

    fn insert_outbox_event(&self, ctx: &ForumRequestContext, event: &ForumOutboxEvent) -> Result<(), ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        let org_id = ctx.organization_id_value();
        run_db!(async {
            sqlx::query(
                "INSERT INTO forum_outbox_event (
                    uuid, event_key, aggregate_type, aggregate_id, event_type, event_version,
                    payload_json, headers_json, status, publish_attempts, created_at, updated_at,
                    tenant_id, organization_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW(), $11, $12)"
            )
            .bind(&event.uuid)
            .bind(&event.event_key)
            .bind(&event.aggregate_type)
            .bind(&event.aggregate_id)
            .bind(&event.event_type)
            .bind(event.event_version)
            .bind(&event.payload_json)
            .bind(&event.headers_json)
            .bind(&event.status)
            .bind(event.publish_attempts)
            .bind(tenant_id)
            .bind(org_id)
            .execute(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(())
    }

    fn update_topic_stats(&self, ctx: &ForumRequestContext, topic_id: i64) -> Result<(), ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        run_db!(async {
            sqlx::query(
                "UPDATE forum_topic_stats SET
                    reply_count = (SELECT COUNT(*) FROM forum_topic_reply WHERE topic_id = $1 AND tenant_id = $2 AND deleted_at IS NULL),
                    last_calculated_at = NOW()
                 WHERE topic_id = $1 AND tenant_id = $2"
            )
            .bind(topic_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(())
    }

    fn update_board_stats(&self, ctx: &ForumRequestContext, board_id: i64) -> Result<(), ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        run_db!(async {
            sqlx::query(
                "UPDATE forum_board_stats SET
                    topic_count = (SELECT COUNT(*) FROM forum_topic WHERE board_id = $1 AND tenant_id = $2 AND deleted_at IS NULL),
                    reply_count = (SELECT COUNT(*) FROM forum_topic_reply WHERE board_id = $1 AND tenant_id = $2 AND deleted_at IS NULL),
                    last_calculated_at = NOW()
                 WHERE board_id = $1 AND tenant_id = $2"
            )
            .bind(board_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(())
    }

    fn update_member_stats(&self, ctx: &ForumRequestContext, user_id: i64) -> Result<(), ForumServiceError> {
        let tenant_id = ctx.tenant_id_value();
        run_db!(async {
            sqlx::query(
                "UPDATE forum_member_stats SET
                    topic_count = (SELECT COUNT(*) FROM forum_topic WHERE author_user_id = $1 AND tenant_id = $2 AND deleted_at IS NULL),
                    reply_count = (SELECT COUNT(*) FROM forum_topic_reply WHERE author_user_id = $1 AND tenant_id = $2 AND deleted_at IS NULL),
                    last_calculated_at = NOW()
                 WHERE user_id = $1 AND tenant_id = $2"
            )
            .bind(user_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
        }).map_err(|e| ForumServiceError::internal(e.to_string()))?;
        Ok(())
    }
}
