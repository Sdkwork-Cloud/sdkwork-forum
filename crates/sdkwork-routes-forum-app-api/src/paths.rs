pub const PREFIX: &str = "/app/v3/api";

pub const ROUTES: &[(&str, &str, &str)] = &[
    ("GET", "/app/v3/api/forum/nodes/tree", "nodes.tree.list"),
    (
        "GET",
        "/app/v3/api/forum/boards/{boardId}/topics",
        "topics.list",
    ),
    ("POST", "/app/v3/api/forum/topics", "topics.create"),
    (
        "GET",
        "/app/v3/api/forum/topics/{topicId}",
        "topics.retrieve",
    ),
    (
        "PATCH",
        "/app/v3/api/forum/topics/{topicId}",
        "topics.update",
    ),
    (
        "DELETE",
        "/app/v3/api/forum/topics/{topicId}",
        "topics.delete",
    ),
    (
        "GET",
        "/app/v3/api/forum/topics/{topicId}/replies",
        "topics.replies.list",
    ),
    (
        "POST",
        "/app/v3/api/forum/topics/{topicId}/replies",
        "topics.replies.create",
    ),
    (
        "PATCH",
        "/app/v3/api/forum/replies/{replyId}",
        "replies.update",
    ),
    (
        "DELETE",
        "/app/v3/api/forum/replies/{replyId}",
        "replies.delete",
    ),
    (
        "GET",
        "/app/v3/api/forum/topics/{topicId}/revisions",
        "topics.revisions.list",
    ),
    (
        "GET",
        "/app/v3/api/forum/replies/{replyId}/revisions",
        "replies.revisions.list",
    ),
    (
        "PUT",
        "/app/v3/api/forum/questions/{topicId}/accepted_reply",
        "questions.acceptedReply.update",
    ),
    (
        "DELETE",
        "/app/v3/api/forum/questions/{topicId}/accepted_reply",
        "questions.acceptedReply.delete",
    ),
    (
        "POST",
        "/app/v3/api/forum/polls/{pollId}/votes",
        "polls.votes.create",
    ),
    ("POST", "/app/v3/api/forum/reactions", "reactions.create"),
    ("POST", "/app/v3/api/forum/votes", "votes.create"),
    ("POST", "/app/v3/api/forum/bookmarks", "bookmarks.create"),
    (
        "PATCH",
        "/app/v3/api/forum/read_state/topics/{topicId}",
        "readState.topics.update",
    ),
    ("POST", "/app/v3/api/forum/reports", "reports.create"),
    ("GET", "/app/v3/api/forum/feed", "feed.list"),
    ("GET", "/app/v3/api/forum/search", "search.query"),
];
