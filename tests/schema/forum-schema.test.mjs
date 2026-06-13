import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const failures = [];

function fail(message) {
  failures.push(message);
}

function read(file) {
  return readFileSync(join(root, file), "utf8");
}

console.log("Running forum schema tests...");

const schemaFile = "specs/forum-database.schema.yaml";
if (!existsSync(join(root, schemaFile))) {
  console.error("schema file not found, skipping schema tests");
  process.exit(0);
}

const schema = read(schemaFile);

const requiredTables = [
  "forum_space", "forum_node", "forum_board_profile", "forum_tag",
  "forum_topic_tag", "forum_topic_prefix", "forum_node_acl",
  "forum_topic", "forum_topic_revision", "forum_topic_reply", "forum_reply_revision",
  "forum_attachment", "forum_question_profile",
  "forum_poll", "forum_poll_option", "forum_poll_vote",
  "forum_reaction", "forum_vote", "forum_bookmark", "forum_subscription",
  "forum_read_state", "forum_notification_preference",
  "forum_member_profile", "forum_trust_level", "forum_privilege_grant",
  "forum_badge", "forum_user_badge",
  "forum_reputation_ledger", "forum_reputation_rule",
  "forum_report", "forum_moderation_queue_item", "forum_moderation_case",
  "forum_moderation_decision", "forum_moderation_policy",
  "forum_sanction", "forum_appeal",
  "forum_feed_item", "forum_public_topic_projection",
  "forum_topic_stats", "forum_board_stats", "forum_member_stats",
  "forum_search_document",
  "forum_outbox_event", "forum_inbox_event", "forum_idempotency_record",
];

const tableMatches = [...schema.matchAll(/^\s+- name: (forum_[a-z0-9_]+)/gm)].map((m) => m[1]);

for (const table of requiredTables) {
  if (!tableMatches.includes(table)) {
    fail(`missing table contract: ${table}`);
  }
}

for (const table of tableMatches) {
  if (!table.startsWith("forum_")) {
    fail(`table does not use forum_ prefix: ${table}`);
  }
}

const requiredFieldSets = ["tenant_entity", "integration_log"];
for (const fieldSet of requiredFieldSets) {
  if (!schema.includes(`${fieldSet}:`)) {
    fail(`missing field set: ${fieldSet}`);
  }
}

const requiredTenantFields = ["tenant_id", "organization_id", "data_scope", "created_at", "updated_at", "version", "status"];
for (const field of requiredTenantFields) {
  if (!schema.includes(`name: ${field}`)) {
    fail(`missing required tenant field: ${field}`);
  }
}

const groups = [...schema.matchAll(/^\s+group: ([a-z_]+)/gm)].map((m) => m[1]);
const uniqueGroups = [...new Set(groups)];
const expectedGroups = ["taxonomy", "discussion", "qa_poll", "engagement", "member", "moderation", "projection", "integration"];
for (const group of expectedGroups) {
  if (!uniqueGroups.includes(group)) {
    fail(`missing table group: ${group}`);
  }
}

for (const table of tableMatches) {
  const tableSection = schema.split(`name: ${table}`)[1]?.split(/\n\s+- name:/)[0] ?? "";
  if (!tableSection.includes("profile:")) {
    fail(`table ${table} missing profile`);
  }
  if (!tableSection.includes("owner:")) {
    fail(`table ${table} missing owner`);
  }
  if (!tableSection.includes("description:")) {
    fail(`table ${table} missing description`);
  }
}

if (failures.length > 0) {
  console.error("Schema test failures:");
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}

console.log(`forum schema tests passed (${tableMatches.length} tables, ${uniqueGroups.length} groups verified)`);
