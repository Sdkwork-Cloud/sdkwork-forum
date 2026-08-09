-- sdkwork:migration
-- id: 0001_organization_id_not_null
-- engine: postgres
-- module: sdkwork-forum
-- purpose: Enforce organization_id NOT NULL DEFAULT on all tables in the
--   consolidated baseline. NULL rows (pre-standard data anomalies) are
--   backfilled with the platform sentinel before NOT NULL is set, and
--   NOT NULL columns without an explicit default receive the sentinel
--   default, keeping existing deployments consistent with fresh baseline
--   installs.
-- reversible: false
-- rollback: forward-fix (sentinel backfill is the canonical fix; NULL
--   organization rows are data anomalies)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

UPDATE forum_space SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_space ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_space ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_node SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_node ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_node ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_board_profile SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_board_profile ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_board_profile ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_tag SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_tag ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_tag ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_topic_tag SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_topic_tag ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_topic_tag ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_topic_prefix SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_topic_prefix ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_topic_prefix ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_node_acl SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_node_acl ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_node_acl ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_member_profile SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_member_profile ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_member_profile ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_trust_level SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_trust_level ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_trust_level ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_privilege_grant SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_privilege_grant ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_privilege_grant ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_badge SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_badge ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_badge ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_reputation_rule SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_reputation_rule ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_reputation_rule ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_topic SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_topic ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_topic ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_topic_revision SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_topic_revision ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_topic_revision ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_topic_reply SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_topic_reply ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_topic_reply ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_reply_revision SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_reply_revision ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_reply_revision ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_attachment SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_attachment ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_attachment ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_question_profile SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_question_profile ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_question_profile ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_poll SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_poll ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_poll ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_poll_option SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_poll_option ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_poll_option ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_poll_vote SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_poll_vote ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_poll_vote ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_reaction SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_reaction ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_reaction ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_vote SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_vote ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_vote ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_bookmark SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_bookmark ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_bookmark ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_subscription SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_subscription ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_subscription ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_read_state SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_read_state ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_read_state ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_notification_preference SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_notification_preference ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_notification_preference ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_user_badge SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_user_badge ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_user_badge ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_reputation_ledger SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_reputation_ledger ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_reputation_ledger ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_report SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_report ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_report ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_moderation_case SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_moderation_case ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_moderation_case ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_moderation_decision SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_moderation_decision ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_moderation_decision ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_moderation_queue_item SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_moderation_queue_item ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_moderation_queue_item ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_moderation_policy SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_moderation_policy ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_moderation_policy ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_sanction SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_sanction ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_sanction ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_appeal SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_appeal ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_appeal ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_feed_item SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_feed_item ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_feed_item ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_public_topic_projection SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_public_topic_projection ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_public_topic_projection ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_topic_stats SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_topic_stats ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_topic_stats ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_board_stats SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_board_stats ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_board_stats ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_member_stats SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_member_stats ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_member_stats ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_search_document SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_search_document ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_search_document ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_outbox_event SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_outbox_event ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_outbox_event ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_inbox_event SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_inbox_event ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_inbox_event ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_idempotency_record SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_idempotency_record ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_idempotency_record ALTER COLUMN organization_id SET NOT NULL;

UPDATE forum_audit_action SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE forum_audit_action ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE forum_audit_action ALTER COLUMN organization_id SET NOT NULL;

COMMIT;
