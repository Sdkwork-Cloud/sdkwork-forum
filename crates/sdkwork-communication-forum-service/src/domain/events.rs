#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForumDomainEvent {
    pub event_type: &'static str,
    pub aggregate_type: &'static str,
    pub aggregate_id: String,
    pub event_version: u16,
}

impl ForumDomainEvent {
    pub fn topic_created(topic_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.topic.created",
            aggregate_type: "forum_topic",
            aggregate_id: topic_id.into(),
            event_version: 1,
        }
    }

    pub fn topic_updated(topic_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.topic.updated",
            aggregate_type: "forum_topic",
            aggregate_id: topic_id.into(),
            event_version: 1,
        }
    }

    pub fn topic_deleted(topic_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.topic.deleted",
            aggregate_type: "forum_topic",
            aggregate_id: topic_id.into(),
            event_version: 1,
        }
    }

    pub fn reply_created(reply_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.reply.created",
            aggregate_type: "forum_topic_reply",
            aggregate_id: reply_id.into(),
            event_version: 1,
        }
    }

    pub fn reply_deleted(reply_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.reply.deleted",
            aggregate_type: "forum_topic_reply",
            aggregate_id: reply_id.into(),
            event_version: 1,
        }
    }

    pub fn moderation_case_created(case_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.moderation.case.created",
            aggregate_type: "forum_moderation_case",
            aggregate_id: case_id.into(),
            event_version: 1,
        }
    }

    pub fn moderation_decision_created(case_id: impl Into<String>) -> Self {
        Self {
            event_type: "forum.moderation.decision.created",
            aggregate_type: "forum_moderation_case",
            aggregate_id: case_id.into(),
            event_version: 1,
        }
    }
}
