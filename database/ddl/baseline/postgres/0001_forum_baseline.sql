-- ============================================================================
-- SDKWork Forum PostgreSQL DDL
-- Generated from: specs/forum-database.schema.yaml
-- Schema version: 1
-- Domain: communication / forum
-- Tables: 46 (no foreign keys)
-- ============================================================================

-- ============================================================================
-- GROUP: taxonomy
-- ============================================================================

CREATE TABLE forum_space (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    code            VARCHAR(64)   NOT NULL,
    slug            VARCHAR(120)  NOT NULL,
    name            VARCHAR(160)  NOT NULL,
    description     VARCHAR(1000),
    visibility      VARCHAR(32)   NOT NULL,
    default_locale  VARCHAR(16),
    settings        JSONB         NOT NULL,
    CONSTRAINT pk_forum_space PRIMARY KEY (id),
    CONSTRAINT uk_forum_space_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_space_tenant_code UNIQUE (tenant_id, code),
    CONSTRAINT uk_forum_space_tenant_slug UNIQUE (tenant_id, slug)
);

CREATE INDEX idx_forum_space_tenant_status_updated
    ON forum_space (tenant_id, organization_id, status, updated_at);

-- --------------------------------------------------------------------------

CREATE TABLE forum_node (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    space_id        BIGINT        NOT NULL,
    parent_id       BIGINT,
    node_type       VARCHAR(32)   NOT NULL,
    slug            VARCHAR(120)  NOT NULL,
    name            VARCHAR(160)  NOT NULL,
    description     VARCHAR(1000),
    path            VARCHAR(1000) NOT NULL,
    level_no        INT           NOT NULL,
    sort_order      INT           NOT NULL,
    icon_media_id   VARCHAR(128),
    cover_media_id  VARCHAR(128),
    settings        JSONB         NOT NULL,
    CONSTRAINT pk_forum_node PRIMARY KEY (id),
    CONSTRAINT uk_forum_node_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_node_tenant_space_parent_slug UNIQUE (tenant_id, space_id, parent_id, slug)
);

CREATE INDEX idx_forum_node_tenant_parent_status_sort
    ON forum_node (tenant_id, organization_id, parent_id, status, sort_order);

CREATE INDEX idx_forum_node_tenant_space_type_status
    ON forum_node (tenant_id, space_id, node_type, status, sort_order);

-- --------------------------------------------------------------------------

CREATE TABLE forum_board_profile (
    id                  BIGINT        NOT NULL,
    uuid                UUID          NOT NULL,
    tenant_id           BIGINT        NOT NULL,
    organization_id     BIGINT        NOT NULL,
    data_scope          VARCHAR(32)   NOT NULL,
    status              VARCHAR(32)   NOT NULL,
    version             BIGINT        NOT NULL,
    created_at          TIMESTAMPTZ   NOT NULL,
    updated_at          TIMESTAMPTZ   NOT NULL,
    deleted_at          TIMESTAMPTZ,
    deleted_by          BIGINT,
    node_id             BIGINT        NOT NULL,
    topic_create_mode   VARCHAR(32)   NOT NULL,
    reply_create_mode   VARCHAR(32)   NOT NULL,
    default_topic_sort  VARCHAR(32)   NOT NULL,
    moderation_mode     VARCHAR(32)   NOT NULL,
    attachment_policy   JSONB         NOT NULL,
    board_rules         JSONB         NOT NULL,
    CONSTRAINT pk_forum_board_profile PRIMARY KEY (id),
    CONSTRAINT uk_forum_board_profile_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_board_profile_node UNIQUE (tenant_id, node_id)
);

CREATE INDEX idx_forum_board_profile_tenant_status_updated
    ON forum_board_profile (tenant_id, organization_id, status, updated_at);

-- --------------------------------------------------------------------------

CREATE TABLE forum_tag (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    space_id        BIGINT        NOT NULL,
    slug            VARCHAR(120)  NOT NULL,
    name            VARCHAR(120)  NOT NULL,
    description     VARCHAR(500),
    color           VARCHAR(32),
    usage_count     BIGINT        NOT NULL,
    CONSTRAINT pk_forum_tag PRIMARY KEY (id),
    CONSTRAINT uk_forum_tag_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_tag_tenant_space_slug UNIQUE (tenant_id, space_id, slug)
);

CREATE INDEX idx_forum_tag_tenant_usage
    ON forum_tag (tenant_id, space_id, status, usage_count);

-- --------------------------------------------------------------------------

CREATE TABLE forum_topic_tag (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    topic_id        BIGINT        NOT NULL,
    tag_id          BIGINT        NOT NULL,
    applied_by      BIGINT        NOT NULL,
    CONSTRAINT pk_forum_topic_tag PRIMARY KEY (id),
    CONSTRAINT uk_forum_topic_tag_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_topic_tag_topic_tag UNIQUE (tenant_id, topic_id, tag_id)
);

CREATE INDEX idx_forum_topic_tag_tenant_tag_created
    ON forum_topic_tag (tenant_id, tag_id, created_at, id);

CREATE INDEX idx_forum_topic_tag_tenant_topic
    ON forum_topic_tag (tenant_id, topic_id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_topic_prefix (
    id                   BIGINT        NOT NULL,
    uuid                 UUID          NOT NULL,
    tenant_id            BIGINT        NOT NULL,
    organization_id      BIGINT        NOT NULL,
    data_scope           VARCHAR(32)   NOT NULL,
    status               VARCHAR(32)   NOT NULL,
    version              BIGINT        NOT NULL,
    created_at           TIMESTAMPTZ   NOT NULL,
    updated_at           TIMESTAMPTZ   NOT NULL,
    deleted_at           TIMESTAMPTZ,
    deleted_by           BIGINT,
    board_id             BIGINT        NOT NULL,
    code                 VARCHAR(64)   NOT NULL,
    label                VARCHAR(80)   NOT NULL,
    color                VARCHAR(32),
    sort_order           INT           NOT NULL,
    required_trust_level INT,
    CONSTRAINT pk_forum_topic_prefix PRIMARY KEY (id),
    CONSTRAINT uk_forum_topic_prefix_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_topic_prefix_board_code UNIQUE (tenant_id, board_id, code)
);

CREATE INDEX idx_forum_topic_prefix_tenant_board_status_sort
    ON forum_topic_prefix (tenant_id, board_id, status, sort_order);

-- --------------------------------------------------------------------------

CREATE TABLE forum_node_acl (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    node_id         BIGINT        NOT NULL,
    principal_type  VARCHAR(32)   NOT NULL,
    principal_id    VARCHAR(128)  NOT NULL,
    permission_code VARCHAR(120)  NOT NULL,
    effect          VARCHAR(16)   NOT NULL,
    condition_json  JSONB,
    CONSTRAINT pk_forum_node_acl PRIMARY KEY (id),
    CONSTRAINT uk_forum_node_acl_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_node_acl_scope UNIQUE (tenant_id, node_id, principal_type, principal_id, permission_code)
);

CREATE INDEX idx_forum_node_acl_tenant_principal
    ON forum_node_acl (tenant_id, principal_type, principal_id, status);

CREATE INDEX idx_forum_node_acl_tenant_node_permission
    ON forum_node_acl (tenant_id, node_id, permission_code, status);


-- ============================================================================
-- GROUP: member
-- ============================================================================

CREATE TABLE forum_member_profile (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    user_id           BIGINT        NOT NULL,
    display_name      VARCHAR(120)  NOT NULL,
    avatar_media_id   VARCHAR(128),
    bio               VARCHAR(1000),
    trust_level       INT           NOT NULL,
    reputation_score  BIGINT        NOT NULL,
    joined_at         TIMESTAMPTZ   NOT NULL,
    suspended_until   TIMESTAMPTZ,
    settings          JSONB         NOT NULL,
    CONSTRAINT pk_forum_member_profile PRIMARY KEY (id),
    CONSTRAINT uk_forum_member_profile_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_member_profile_user UNIQUE (tenant_id, user_id)
);

CREATE INDEX idx_forum_member_profile_tenant_reputation
    ON forum_member_profile (tenant_id, reputation_score, id);

CREATE INDEX idx_forum_member_profile_tenant_trust
    ON forum_member_profile (tenant_id, trust_level, status);

-- --------------------------------------------------------------------------

CREATE TABLE forum_trust_level (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    data_scope       VARCHAR(32)   NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    deleted_at       TIMESTAMPTZ,
    deleted_by       BIGINT,
    level_no         INT           NOT NULL,
    code             VARCHAR(64)   NOT NULL,
    name             VARCHAR(120)  NOT NULL,
    threshold_rules  JSONB         NOT NULL,
    privileges       JSONB         NOT NULL,
    CONSTRAINT pk_forum_trust_level PRIMARY KEY (id),
    CONSTRAINT uk_forum_trust_level_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_trust_level_tenant_level UNIQUE (tenant_id, level_no),
    CONSTRAINT uk_forum_trust_level_tenant_code UNIQUE (tenant_id, code)
);

CREATE INDEX idx_forum_trust_level_tenant_status_level
    ON forum_trust_level (tenant_id, status, level_no);

-- --------------------------------------------------------------------------

CREATE TABLE forum_privilege_grant (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    principal_type  VARCHAR(32)   NOT NULL,
    principal_id    VARCHAR(128)  NOT NULL,
    privilege_code  VARCHAR(120)  NOT NULL,
    scope_type      VARCHAR(32)   NOT NULL,
    scope_id        BIGINT,
    expire_at       TIMESTAMPTZ,
    granted_by      BIGINT        NOT NULL,
    CONSTRAINT pk_forum_privilege_grant PRIMARY KEY (id),
    CONSTRAINT uk_forum_privilege_grant_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_privilege_grant_scope UNIQUE (tenant_id, principal_type, principal_id, privilege_code, scope_type, scope_id)
);

CREATE INDEX idx_forum_privilege_grant_tenant_principal
    ON forum_privilege_grant (tenant_id, principal_type, principal_id, status);

CREATE INDEX idx_forum_privilege_grant_tenant_expire
    ON forum_privilege_grant (tenant_id, expire_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_badge (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    code            VARCHAR(64)   NOT NULL,
    name            VARCHAR(120)  NOT NULL,
    description     VARCHAR(500),
    icon_media_id   VARCHAR(128),
    grant_mode      VARCHAR(32)   NOT NULL,
    rule_json       JSONB,
    CONSTRAINT pk_forum_badge PRIMARY KEY (id),
    CONSTRAINT uk_forum_badge_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_badge_tenant_code UNIQUE (tenant_id, code)
);

CREATE INDEX idx_forum_badge_tenant_status_updated
    ON forum_badge (tenant_id, status, updated_at);

-- --------------------------------------------------------------------------

CREATE TABLE forum_reputation_rule (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    code            VARCHAR(80)   NOT NULL,
    event_type      VARCHAR(120)  NOT NULL,
    points          BIGINT        NOT NULL,
    daily_limit     BIGINT,
    rule_json       JSONB         NOT NULL,
    CONSTRAINT pk_forum_reputation_rule PRIMARY KEY (id),
    CONSTRAINT uk_forum_reputation_rule_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_reputation_rule_tenant_code UNIQUE (tenant_id, code)
);

CREATE INDEX idx_forum_reputation_rule_tenant_event
    ON forum_reputation_rule (tenant_id, event_type, status);


-- ============================================================================
-- GROUP: discussion
-- ============================================================================

CREATE TABLE forum_topic (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    space_id          BIGINT        NOT NULL,
    board_id          BIGINT        NOT NULL,
    author_user_id    BIGINT        NOT NULL,
    author_profile_id BIGINT,
    prefix_id         BIGINT,
    slug              VARCHAR(180),
    title             VARCHAR(240)  NOT NULL,
    body_format       VARCHAR(32)   NOT NULL,
    body              TEXT          NOT NULL,
    body_excerpt      VARCHAR(500),
    content_hash      VARCHAR(128)  NOT NULL,
    topic_type        VARCHAR(32)   NOT NULL,
    moderation_status VARCHAR(32)   NOT NULL,
    visibility        VARCHAR(32)   NOT NULL,
    pinned_at         TIMESTAMPTZ,
    pinned_until      TIMESTAMPTZ,
    featured_at       TIMESTAMPTZ,
    locked_at         TIMESTAMPTZ,
    locked_by         BIGINT,
    last_reply_id     BIGINT,
    last_activity_at  TIMESTAMPTZ   NOT NULL,
    accepted_reply_id BIGINT,
    attachment_count  INT           NOT NULL,
    metadata          JSONB         NOT NULL,
    CONSTRAINT pk_forum_topic PRIMARY KEY (id),
    CONSTRAINT uk_forum_topic_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_topic_board_slug UNIQUE (tenant_id, board_id, slug)
);

CREATE INDEX idx_forum_topic_tenant_board_status_activity
    ON forum_topic (tenant_id, board_id, moderation_status, last_activity_at, id);

CREATE INDEX idx_forum_topic_tenant_author_updated
    ON forum_topic (tenant_id, author_user_id, updated_at, id);

CREATE INDEX idx_forum_topic_tenant_featured
    ON forum_topic (tenant_id, featured_at, id);

CREATE INDEX idx_forum_topic_tenant_pinned
    ON forum_topic (tenant_id, board_id, pinned_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_topic_revision (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    topic_id        BIGINT        NOT NULL,
    revision_no     INT           NOT NULL,
    editor_user_id  BIGINT        NOT NULL,
    title           VARCHAR(240)  NOT NULL,
    body_format     VARCHAR(32)   NOT NULL,
    body            TEXT          NOT NULL,
    edit_reason     VARCHAR(500),
    content_hash    VARCHAR(128)  NOT NULL,
    CONSTRAINT pk_forum_topic_revision PRIMARY KEY (id),
    CONSTRAINT uk_forum_topic_revision_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_topic_revision_topic_no UNIQUE (tenant_id, topic_id, revision_no)
);

CREATE INDEX idx_forum_topic_revision_tenant_topic_created
    ON forum_topic_revision (tenant_id, topic_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_topic_reply (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    topic_id          BIGINT        NOT NULL,
    board_id          BIGINT        NOT NULL,
    parent_reply_id   BIGINT,
    author_user_id    BIGINT        NOT NULL,
    author_profile_id BIGINT,
    reply_no          INT           NOT NULL,
    body_format       VARCHAR(32)   NOT NULL,
    body              TEXT          NOT NULL,
    body_excerpt      VARCHAR(500),
    content_hash      VARCHAR(128)  NOT NULL,
    moderation_status VARCHAR(32)   NOT NULL,
    accepted_at       TIMESTAMPTZ,
    accepted_by       BIGINT,
    attachment_count  INT           NOT NULL,
    metadata          JSONB         NOT NULL,
    CONSTRAINT pk_forum_topic_reply PRIMARY KEY (id),
    CONSTRAINT uk_forum_topic_reply_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_topic_reply_topic_no UNIQUE (tenant_id, topic_id, reply_no)
);

CREATE INDEX idx_forum_topic_reply_tenant_topic_status_created
    ON forum_topic_reply (tenant_id, topic_id, moderation_status, created_at, id);

CREATE INDEX idx_forum_topic_reply_tenant_author_updated
    ON forum_topic_reply (tenant_id, author_user_id, updated_at, id);

CREATE INDEX idx_forum_topic_reply_tenant_parent
    ON forum_topic_reply (tenant_id, parent_reply_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_reply_revision (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    reply_id        BIGINT        NOT NULL,
    topic_id        BIGINT        NOT NULL,
    revision_no     INT           NOT NULL,
    editor_user_id  BIGINT        NOT NULL,
    body_format     VARCHAR(32)   NOT NULL,
    body            TEXT          NOT NULL,
    edit_reason     VARCHAR(500),
    content_hash    VARCHAR(128)  NOT NULL,
    CONSTRAINT pk_forum_reply_revision PRIMARY KEY (id),
    CONSTRAINT uk_forum_reply_revision_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_reply_revision_reply_no UNIQUE (tenant_id, reply_id, revision_no)
);

CREATE INDEX idx_forum_reply_revision_tenant_reply_created
    ON forum_reply_revision (tenant_id, reply_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_attachment (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    owner_type        VARCHAR(32)   NOT NULL,
    owner_id          BIGINT        NOT NULL,
    drive_space_id    VARCHAR(128)  NOT NULL,
    drive_node_id     VARCHAR(128)  NOT NULL,
    media_resource_id VARCHAR(128),
    file_name         VARCHAR(260)  NOT NULL,
    mime_type         VARCHAR(120)  NOT NULL,
    byte_size         BIGINT        NOT NULL,
    sort_order        INT           NOT NULL,
    scan_status       VARCHAR(32)   NOT NULL,
    CONSTRAINT pk_forum_attachment PRIMARY KEY (id),
    CONSTRAINT uk_forum_attachment_uuid UNIQUE (uuid)
);

CREATE INDEX idx_forum_attachment_tenant_owner_sort
    ON forum_attachment (tenant_id, owner_type, owner_id, status, sort_order);

CREATE INDEX idx_forum_attachment_tenant_drive_node
    ON forum_attachment (tenant_id, drive_node_id);


-- ============================================================================
-- GROUP: qa_poll
-- ============================================================================

CREATE TABLE forum_question_profile (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    topic_id          BIGINT        NOT NULL,
    bounty_points     BIGINT        NOT NULL,
    bounty_expire_at  TIMESTAMPTZ,
    accepted_reply_id BIGINT,
    accepted_at       TIMESTAMPTZ,
    answer_count      BIGINT        NOT NULL,
    solved_status     VARCHAR(32)   NOT NULL,
    CONSTRAINT pk_forum_question_profile PRIMARY KEY (id),
    CONSTRAINT uk_forum_question_profile_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_question_profile_topic UNIQUE (tenant_id, topic_id)
);

CREATE INDEX idx_forum_question_profile_tenant_solved_updated
    ON forum_question_profile (tenant_id, solved_status, updated_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_poll (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    topic_id          BIGINT        NOT NULL,
    title             VARCHAR(200)  NOT NULL,
    selection_mode    VARCHAR(32)   NOT NULL,
    min_choices       INT           NOT NULL,
    max_choices       INT           NOT NULL,
    vote_visibility   VARCHAR(32)   NOT NULL,
    open_at           TIMESTAMPTZ,
    close_at          TIMESTAMPTZ,
    total_vote_count  BIGINT        NOT NULL,
    settings          JSONB         NOT NULL,
    CONSTRAINT pk_forum_poll PRIMARY KEY (id),
    CONSTRAINT uk_forum_poll_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_poll_topic UNIQUE (tenant_id, topic_id)
);

CREATE INDEX idx_forum_poll_tenant_status_close
    ON forum_poll (tenant_id, status, close_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_poll_option (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    poll_id         BIGINT        NOT NULL,
    option_key      VARCHAR(64)   NOT NULL,
    label           VARCHAR(240)  NOT NULL,
    description     VARCHAR(500),
    sort_order      INT           NOT NULL,
    vote_count      BIGINT        NOT NULL,
    CONSTRAINT pk_forum_poll_option PRIMARY KEY (id),
    CONSTRAINT uk_forum_poll_option_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_poll_option_poll_key UNIQUE (tenant_id, poll_id, option_key)
);

CREATE INDEX idx_forum_poll_option_tenant_poll_sort
    ON forum_poll_option (tenant_id, poll_id, status, sort_order);

-- --------------------------------------------------------------------------

CREATE TABLE forum_poll_vote (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    poll_id         BIGINT        NOT NULL,
    option_id       BIGINT        NOT NULL,
    voter_user_id   BIGINT        NOT NULL,
    idempotency_key VARCHAR(160),
    vote_weight     INT           NOT NULL,
    CONSTRAINT pk_forum_poll_vote PRIMARY KEY (id),
    CONSTRAINT uk_forum_poll_vote_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_poll_vote_single UNIQUE (tenant_id, poll_id, voter_user_id, option_id)
);

CREATE INDEX idx_forum_poll_vote_tenant_poll_user
    ON forum_poll_vote (tenant_id, poll_id, voter_user_id);

CREATE INDEX idx_forum_poll_vote_tenant_option_created
    ON forum_poll_vote (tenant_id, option_id, created_at, id);


-- ============================================================================
-- GROUP: engagement
-- ============================================================================

CREATE TABLE forum_reaction (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    actor_user_id   BIGINT        NOT NULL,
    reaction_type   VARCHAR(64)   NOT NULL,
    CONSTRAINT pk_forum_reaction PRIMARY KEY (id),
    CONSTRAINT uk_forum_reaction_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_reaction_target_actor_type UNIQUE (tenant_id, target_type, target_id, actor_user_id, reaction_type)
);

CREATE INDEX idx_forum_reaction_tenant_target
    ON forum_reaction (tenant_id, target_type, target_id, status);

CREATE INDEX idx_forum_reaction_tenant_actor_created
    ON forum_reaction (tenant_id, actor_user_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_vote (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    actor_user_id   BIGINT        NOT NULL,
    vote_value      INT           NOT NULL,
    reason_code     VARCHAR(64),
    CONSTRAINT pk_forum_vote PRIMARY KEY (id),
    CONSTRAINT uk_forum_vote_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_vote_target_actor UNIQUE (tenant_id, target_type, target_id, actor_user_id)
);

CREATE INDEX idx_forum_vote_tenant_target
    ON forum_vote (tenant_id, target_type, target_id, status);

CREATE INDEX idx_forum_vote_tenant_actor_created
    ON forum_vote (tenant_id, actor_user_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_bookmark (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    user_id         BIGINT        NOT NULL,
    folder_code     VARCHAR(64),
    note            VARCHAR(500),
    CONSTRAINT pk_forum_bookmark PRIMARY KEY (id),
    CONSTRAINT uk_forum_bookmark_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_bookmark_target_user UNIQUE (tenant_id, target_type, target_id, user_id)
);

CREATE INDEX idx_forum_bookmark_tenant_user_updated
    ON forum_bookmark (tenant_id, user_id, updated_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_subscription (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    target_type       VARCHAR(32)   NOT NULL,
    target_id         BIGINT        NOT NULL,
    user_id           BIGINT        NOT NULL,
    notify_level      VARCHAR(32)   NOT NULL,
    delivery_channels JSONB         NOT NULL,
    CONSTRAINT pk_forum_subscription PRIMARY KEY (id),
    CONSTRAINT uk_forum_subscription_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_subscription_target_user UNIQUE (tenant_id, target_type, target_id, user_id)
);

CREATE INDEX idx_forum_subscription_tenant_target_level
    ON forum_subscription (tenant_id, target_type, target_id, notify_level);

CREATE INDEX idx_forum_subscription_tenant_user_updated
    ON forum_subscription (tenant_id, user_id, updated_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_read_state (
    id                BIGINT        NOT NULL,
    uuid              UUID          NOT NULL,
    tenant_id         BIGINT        NOT NULL,
    organization_id   BIGINT        NOT NULL,
    data_scope        VARCHAR(32)   NOT NULL,
    status            VARCHAR(32)   NOT NULL,
    version           BIGINT        NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL,
    updated_at        TIMESTAMPTZ   NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by        BIGINT,
    topic_id          BIGINT        NOT NULL,
    user_id           BIGINT        NOT NULL,
    last_read_reply_id BIGINT,
    last_read_at      TIMESTAMPTZ   NOT NULL,
    unread_count      INT           NOT NULL,
    CONSTRAINT pk_forum_read_state PRIMARY KEY (id),
    CONSTRAINT uk_forum_read_state_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_read_state_topic_user UNIQUE (tenant_id, topic_id, user_id)
);

CREATE INDEX idx_forum_read_state_tenant_user_updated
    ON forum_read_state (tenant_id, user_id, updated_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_notification_preference (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    user_id         BIGINT        NOT NULL,
    event_type      VARCHAR(80)   NOT NULL,
    channel         VARCHAR(32)   NOT NULL,
    enabled         BOOLEAN       NOT NULL,
    quiet_hours     JSONB,
    CONSTRAINT pk_forum_notification_preference PRIMARY KEY (id),
    CONSTRAINT uk_forum_notification_preference_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_notification_preference_user_event_channel UNIQUE (tenant_id, user_id, event_type, channel)
);

CREATE INDEX idx_forum_notification_preference_tenant_user
    ON forum_notification_preference (tenant_id, user_id, status);

-- --------------------------------------------------------------------------

CREATE TABLE forum_user_badge (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    badge_id        BIGINT        NOT NULL,
    user_id         BIGINT        NOT NULL,
    granted_by      BIGINT,
    granted_reason  VARCHAR(500),
    revoked_at      TIMESTAMPTZ,
    revoked_by      BIGINT,
    CONSTRAINT pk_forum_user_badge PRIMARY KEY (id),
    CONSTRAINT uk_forum_user_badge_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_user_badge_user_badge UNIQUE (tenant_id, user_id, badge_id)
);

CREATE INDEX idx_forum_user_badge_tenant_user_created
    ON forum_user_badge (tenant_id, user_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_reputation_ledger (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    data_scope       VARCHAR(32)   NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    deleted_at       TIMESTAMPTZ,
    deleted_by       BIGINT,
    user_id          BIGINT        NOT NULL,
    source_type      VARCHAR(64)   NOT NULL,
    source_id        BIGINT,
    direction        VARCHAR(16)   NOT NULL,
    points           BIGINT        NOT NULL,
    balance_after    BIGINT        NOT NULL,
    reason_code      VARCHAR(80)   NOT NULL,
    idempotency_key  VARCHAR(160)  NOT NULL,
    CONSTRAINT pk_forum_reputation_ledger PRIMARY KEY (id),
    CONSTRAINT uk_forum_reputation_ledger_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_reputation_ledger_key UNIQUE (tenant_id, idempotency_key)
);

CREATE INDEX idx_forum_reputation_ledger_tenant_user_created
    ON forum_reputation_ledger (tenant_id, user_id, created_at, id);

CREATE INDEX idx_forum_reputation_ledger_tenant_source
    ON forum_reputation_ledger (tenant_id, source_type, source_id);


-- ============================================================================
-- GROUP: moderation
-- ============================================================================

CREATE TABLE forum_report (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    reporter_user_id BIGINT,
    reason_code     VARCHAR(80)   NOT NULL,
    description     VARCHAR(2000),
    evidence_json   JSONB,
    report_status   VARCHAR(32)   NOT NULL,
    linked_case_id  BIGINT,
    CONSTRAINT pk_forum_report PRIMARY KEY (id),
    CONSTRAINT uk_forum_report_uuid UNIQUE (uuid)
);

CREATE INDEX idx_forum_report_tenant_target_status
    ON forum_report (tenant_id, target_type, target_id, report_status);

CREATE INDEX idx_forum_report_tenant_status_created
    ON forum_report (tenant_id, report_status, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_moderation_case (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    case_no         VARCHAR(64)   NOT NULL,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    case_status     VARCHAR(32)   NOT NULL,
    severity        VARCHAR(32)   NOT NULL,
    opened_by       BIGINT        NOT NULL,
    assigned_to     BIGINT,
    summary         VARCHAR(1000),
    resolved_at     TIMESTAMPTZ,
    CONSTRAINT pk_forum_moderation_case PRIMARY KEY (id),
    CONSTRAINT uk_forum_moderation_case_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_moderation_case_no UNIQUE (tenant_id, case_no)
);

CREATE INDEX idx_forum_moderation_case_tenant_status_updated
    ON forum_moderation_case (tenant_id, case_status, updated_at, id);

CREATE INDEX idx_forum_moderation_case_tenant_target
    ON forum_moderation_case (tenant_id, target_type, target_id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_moderation_decision (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    data_scope       VARCHAR(32)   NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    deleted_at       TIMESTAMPTZ,
    deleted_by       BIGINT,
    case_id          BIGINT        NOT NULL,
    target_type      VARCHAR(32)   NOT NULL,
    target_id        BIGINT        NOT NULL,
    decision_action  VARCHAR(64)   NOT NULL,
    reason_code      VARCHAR(80)   NOT NULL,
    note             VARCHAR(2000),
    decided_by       BIGINT        NOT NULL,
    before_state     JSONB         NOT NULL,
    after_state      JSONB         NOT NULL,
    idempotency_key  VARCHAR(160),
    CONSTRAINT pk_forum_moderation_decision PRIMARY KEY (id),
    CONSTRAINT uk_forum_moderation_decision_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_moderation_decision_key UNIQUE (tenant_id, idempotency_key)
);

CREATE INDEX idx_forum_moderation_decision_tenant_case_created
    ON forum_moderation_decision (tenant_id, case_id, created_at, id);

CREATE INDEX idx_forum_moderation_decision_tenant_target
    ON forum_moderation_decision (tenant_id, target_type, target_id, created_at);

-- --------------------------------------------------------------------------

CREATE TABLE forum_moderation_queue_item (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    source_type     VARCHAR(32)   NOT NULL,
    source_id       BIGINT,
    severity        VARCHAR(32)   NOT NULL,
    queue_status    VARCHAR(32)   NOT NULL,
    assigned_to     BIGINT,
    due_at          TIMESTAMPTZ,
    case_id         BIGINT,
    CONSTRAINT pk_forum_moderation_queue_item PRIMARY KEY (id),
    CONSTRAINT uk_forum_moderation_queue_item_uuid UNIQUE (uuid)
);

CREATE INDEX idx_forum_moderation_queue_tenant_status_severity_due
    ON forum_moderation_queue_item (tenant_id, queue_status, severity, due_at, id);

CREATE INDEX idx_forum_moderation_queue_tenant_target
    ON forum_moderation_queue_item (tenant_id, target_type, target_id, queue_status);

-- --------------------------------------------------------------------------

CREATE TABLE forum_moderation_policy (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    data_scope       VARCHAR(32)   NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    deleted_at       TIMESTAMPTZ,
    deleted_by       BIGINT,
    code             VARCHAR(80)   NOT NULL,
    name             VARCHAR(160)  NOT NULL,
    target_scope     VARCHAR(32)   NOT NULL,
    target_scope_id  BIGINT,
    trigger_json     JSONB         NOT NULL,
    action_json      JSONB         NOT NULL,
    priority         INT           NOT NULL,
    CONSTRAINT pk_forum_moderation_policy PRIMARY KEY (id),
    CONSTRAINT uk_forum_moderation_policy_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_moderation_policy_code UNIQUE (tenant_id, code)
);

CREATE INDEX idx_forum_moderation_policy_tenant_scope_priority
    ON forum_moderation_policy (tenant_id, target_scope, target_scope_id, status, priority);

-- --------------------------------------------------------------------------

CREATE TABLE forum_sanction (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    user_id         BIGINT        NOT NULL,
    case_id         BIGINT,
    decision_id     BIGINT,
    sanction_type   VARCHAR(64)   NOT NULL,
    reason_code     VARCHAR(80)   NOT NULL,
    starts_at       TIMESTAMPTZ   NOT NULL,
    expires_at      TIMESTAMPTZ,
    lifted_at       TIMESTAMPTZ,
    lifted_by       BIGINT,
    CONSTRAINT pk_forum_sanction PRIMARY KEY (id),
    CONSTRAINT uk_forum_sanction_uuid UNIQUE (uuid)
);

CREATE INDEX idx_forum_sanction_tenant_user_active
    ON forum_sanction (tenant_id, user_id, sanction_type, status, expires_at);

CREATE INDEX idx_forum_sanction_tenant_case
    ON forum_sanction (tenant_id, case_id, created_at);

-- --------------------------------------------------------------------------

CREATE TABLE forum_appeal (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    data_scope       VARCHAR(32)   NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    deleted_at       TIMESTAMPTZ,
    deleted_by       BIGINT,
    case_id          BIGINT,
    sanction_id      BIGINT,
    appellant_user_id BIGINT       NOT NULL,
    appeal_reason    VARCHAR(2000) NOT NULL,
    appeal_status    VARCHAR(32)   NOT NULL,
    reviewed_by      BIGINT,
    reviewed_at      TIMESTAMPTZ,
    resolution_note  VARCHAR(2000),
    CONSTRAINT pk_forum_appeal PRIMARY KEY (id),
    CONSTRAINT uk_forum_appeal_uuid UNIQUE (uuid)
);

CREATE INDEX idx_forum_appeal_tenant_status_created
    ON forum_appeal (tenant_id, appeal_status, created_at, id);

CREATE INDEX idx_forum_appeal_tenant_appellant
    ON forum_appeal (tenant_id, appellant_user_id, created_at, id);


-- ============================================================================
-- GROUP: projection
-- ============================================================================

CREATE TABLE forum_feed_item (
    id                 BIGINT        NOT NULL,
    uuid               UUID          NOT NULL,
    tenant_id          BIGINT        NOT NULL,
    organization_id    BIGINT        NOT NULL,
    data_scope         VARCHAR(32)   NOT NULL,
    status             VARCHAR(32)   NOT NULL,
    version            BIGINT        NOT NULL,
    created_at         TIMESTAMPTZ   NOT NULL,
    updated_at         TIMESTAMPTZ   NOT NULL,
    deleted_at         TIMESTAMPTZ,
    deleted_by         BIGINT,
    feed_type          VARCHAR(32)   NOT NULL,
    feed_owner_id      VARCHAR(128),
    topic_id           BIGINT        NOT NULL,
    reply_id           BIGINT,
    rank_score         NUMERIC(18,6) NOT NULL,
    activity_at        TIMESTAMPTZ   NOT NULL,
    projection_version BIGINT        NOT NULL,
    CONSTRAINT pk_forum_feed_item PRIMARY KEY (id),
    CONSTRAINT uk_forum_feed_item_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_feed_item_scope_topic UNIQUE (tenant_id, feed_type, feed_owner_id, topic_id)
);

CREATE INDEX idx_forum_feed_item_tenant_scope_rank
    ON forum_feed_item (tenant_id, feed_type, feed_owner_id, status, rank_score, id);

CREATE INDEX idx_forum_feed_item_tenant_activity
    ON forum_feed_item (tenant_id, activity_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_public_topic_projection (
    id                   BIGINT        NOT NULL,
    uuid                 UUID          NOT NULL,
    tenant_id            BIGINT        NOT NULL,
    organization_id      BIGINT        NOT NULL,
    data_scope           VARCHAR(32)   NOT NULL,
    status               VARCHAR(32)   NOT NULL,
    version              BIGINT        NOT NULL,
    created_at           TIMESTAMPTZ   NOT NULL,
    updated_at           TIMESTAMPTZ   NOT NULL,
    deleted_at           TIMESTAMPTZ,
    deleted_by           BIGINT,
    site_slug            VARCHAR(120)  NOT NULL,
    topic_id             BIGINT        NOT NULL,
    board_id             BIGINT        NOT NULL,
    topic_slug           VARCHAR(180),
    title                VARCHAR(240)  NOT NULL,
    excerpt              VARCHAR(500),
    author_display_name  VARCHAR(120)  NOT NULL,
    tag_slugs            JSONB         NOT NULL,
    stats_json           JSONB         NOT NULL,
    source_version       BIGINT        NOT NULL,
    projected_at         TIMESTAMPTZ   NOT NULL,
    CONSTRAINT pk_forum_public_topic_projection PRIMARY KEY (id),
    CONSTRAINT uk_forum_public_topic_projection_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_public_topic_projection_site_topic UNIQUE (tenant_id, site_slug, topic_id)
);

CREATE INDEX idx_forum_public_topic_projection_site_updated
    ON forum_public_topic_projection (tenant_id, site_slug, status, updated_at, id);

CREATE INDEX idx_forum_public_topic_projection_board_updated
    ON forum_public_topic_projection (tenant_id, site_slug, board_id, updated_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_topic_stats (
    id                 BIGINT        NOT NULL,
    uuid               UUID          NOT NULL,
    tenant_id          BIGINT        NOT NULL,
    organization_id    BIGINT        NOT NULL,
    data_scope         VARCHAR(32)   NOT NULL,
    status             VARCHAR(32)   NOT NULL,
    version            BIGINT        NOT NULL,
    created_at         TIMESTAMPTZ   NOT NULL,
    updated_at         TIMESTAMPTZ   NOT NULL,
    deleted_at         TIMESTAMPTZ,
    deleted_by         BIGINT,
    topic_id           BIGINT        NOT NULL,
    reply_count        BIGINT        NOT NULL,
    view_count         BIGINT        NOT NULL,
    reaction_count     BIGINT        NOT NULL,
    vote_score         BIGINT        NOT NULL,
    bookmark_count     BIGINT        NOT NULL,
    report_count       BIGINT        NOT NULL,
    last_calculated_at TIMESTAMPTZ   NOT NULL,
    CONSTRAINT pk_forum_topic_stats PRIMARY KEY (id),
    CONSTRAINT uk_forum_topic_stats_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_topic_stats_topic UNIQUE (tenant_id, topic_id)
);

CREATE INDEX idx_forum_topic_stats_tenant_score
    ON forum_topic_stats (tenant_id, vote_score, topic_id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_board_stats (
    id                 BIGINT        NOT NULL,
    uuid               UUID          NOT NULL,
    tenant_id          BIGINT        NOT NULL,
    organization_id    BIGINT        NOT NULL,
    data_scope         VARCHAR(32)   NOT NULL,
    status             VARCHAR(32)   NOT NULL,
    version            BIGINT        NOT NULL,
    created_at         TIMESTAMPTZ   NOT NULL,
    updated_at         TIMESTAMPTZ   NOT NULL,
    deleted_at         TIMESTAMPTZ,
    deleted_by         BIGINT,
    board_id           BIGINT        NOT NULL,
    topic_count        BIGINT        NOT NULL,
    reply_count        BIGINT        NOT NULL,
    member_count       BIGINT        NOT NULL,
    last_topic_id      BIGINT,
    last_reply_id      BIGINT,
    last_activity_at   TIMESTAMPTZ,
    last_calculated_at TIMESTAMPTZ   NOT NULL,
    CONSTRAINT pk_forum_board_stats PRIMARY KEY (id),
    CONSTRAINT uk_forum_board_stats_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_board_stats_board UNIQUE (tenant_id, board_id)
);

CREATE INDEX idx_forum_board_stats_tenant_activity
    ON forum_board_stats (tenant_id, last_activity_at, board_id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_member_stats (
    id                      BIGINT        NOT NULL,
    uuid                    UUID          NOT NULL,
    tenant_id               BIGINT        NOT NULL,
    organization_id         BIGINT        NOT NULL,
    data_scope              VARCHAR(32)   NOT NULL,
    status                  VARCHAR(32)   NOT NULL,
    version                 BIGINT        NOT NULL,
    created_at              TIMESTAMPTZ   NOT NULL,
    updated_at              TIMESTAMPTZ   NOT NULL,
    deleted_at              TIMESTAMPTZ,
    deleted_by              BIGINT,
    user_id                 BIGINT        NOT NULL,
    topic_count             BIGINT        NOT NULL,
    reply_count             BIGINT        NOT NULL,
    accepted_answer_count   BIGINT        NOT NULL,
    reaction_received_count BIGINT        NOT NULL,
    vote_score_received     BIGINT        NOT NULL,
    last_activity_at        TIMESTAMPTZ,
    last_calculated_at      TIMESTAMPTZ   NOT NULL,
    CONSTRAINT pk_forum_member_stats PRIMARY KEY (id),
    CONSTRAINT uk_forum_member_stats_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_member_stats_user UNIQUE (tenant_id, user_id)
);

CREATE INDEX idx_forum_member_stats_tenant_activity
    ON forum_member_stats (tenant_id, last_activity_at, user_id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_search_document (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    deleted_at      TIMESTAMPTZ,
    deleted_by      BIGINT,
    source_type     VARCHAR(32)   NOT NULL,
    source_id       BIGINT        NOT NULL,
    board_id        BIGINT        NOT NULL,
    title           VARCHAR(240),
    body_text       TEXT          NOT NULL,
    tag_text        VARCHAR(1000),
    author_user_id  BIGINT        NOT NULL,
    visibility      VARCHAR(32)   NOT NULL,
    source_version  BIGINT        NOT NULL,
    indexed_at      TIMESTAMPTZ,
    index_status    VARCHAR(32)   NOT NULL,
    CONSTRAINT pk_forum_search_document PRIMARY KEY (id),
    CONSTRAINT uk_forum_search_document_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_search_document_source UNIQUE (tenant_id, source_type, source_id)
);

CREATE INDEX idx_forum_search_document_tenant_status_updated
    ON forum_search_document (tenant_id, index_status, updated_at, id);

CREATE INDEX idx_forum_search_document_tenant_board_updated
    ON forum_search_document (tenant_id, board_id, updated_at, id);


-- ============================================================================
-- GROUP: integration
-- ============================================================================

CREATE TABLE forum_outbox_event (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    event_key        VARCHAR(160)  NOT NULL,
    aggregate_type   VARCHAR(80)   NOT NULL,
    aggregate_id     VARCHAR(128)  NOT NULL,
    event_type       VARCHAR(160)  NOT NULL,
    event_version    INT           NOT NULL,
    payload_json     JSONB         NOT NULL,
    headers_json     JSONB,
    publish_attempts INT           NOT NULL,
    next_attempt_at  TIMESTAMPTZ,
    published_at     TIMESTAMPTZ,
    CONSTRAINT pk_forum_outbox_event PRIMARY KEY (id),
    CONSTRAINT uk_forum_outbox_event_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_outbox_event_key UNIQUE (event_key)
);

CREATE INDEX idx_forum_outbox_event_status_next
    ON forum_outbox_event (status, next_attempt_at, id);

CREATE INDEX idx_forum_outbox_event_tenant_aggregate
    ON forum_outbox_event (tenant_id, aggregate_type, aggregate_id, created_at);

-- --------------------------------------------------------------------------

CREATE TABLE forum_inbox_event (
    id               BIGINT        NOT NULL,
    uuid             UUID          NOT NULL,
    tenant_id        BIGINT        NOT NULL,
    organization_id  BIGINT        NOT NULL,
    status           VARCHAR(32)   NOT NULL,
    version          BIGINT        NOT NULL,
    created_at       TIMESTAMPTZ   NOT NULL,
    updated_at       TIMESTAMPTZ   NOT NULL,
    source_system    VARCHAR(80)   NOT NULL,
    message_id       VARCHAR(160)  NOT NULL,
    consumer_name    VARCHAR(120)  NOT NULL,
    event_type       VARCHAR(160)  NOT NULL,
    event_version    INT           NOT NULL,
    payload_hash     VARCHAR(128)  NOT NULL,
    processed_at     TIMESTAMPTZ,
    failure_reason   VARCHAR(2000),
    CONSTRAINT pk_forum_inbox_event PRIMARY KEY (id),
    CONSTRAINT uk_forum_inbox_event_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_inbox_event_message_consumer UNIQUE (source_system, message_id, consumer_name)
);

CREATE INDEX idx_forum_inbox_event_status_updated
    ON forum_inbox_event (status, updated_at, id);

CREATE INDEX idx_forum_inbox_event_tenant_type_created
    ON forum_inbox_event (tenant_id, event_type, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_idempotency_record (
    id                  BIGINT        NOT NULL,
    uuid                UUID          NOT NULL,
    tenant_id           BIGINT        NOT NULL,
    organization_id     BIGINT        NOT NULL,
    data_scope          VARCHAR(32)   NOT NULL,
    status              VARCHAR(32)   NOT NULL,
    version             BIGINT        NOT NULL,
    created_at          TIMESTAMPTZ   NOT NULL,
    updated_at          TIMESTAMPTZ   NOT NULL,
    deleted_at          TIMESTAMPTZ,
    deleted_by          BIGINT,
    idempotency_key     VARCHAR(160)  NOT NULL,
    request_hash        VARCHAR(128)  NOT NULL,
    operation_id        VARCHAR(160)  NOT NULL,
    principal_id        VARCHAR(128)  NOT NULL,
    response_status     INT,
    response_body_json  JSONB,
    expires_at          TIMESTAMPTZ   NOT NULL,
    CONSTRAINT pk_forum_idempotency_record PRIMARY KEY (id),
    CONSTRAINT uk_forum_idempotency_record_uuid UNIQUE (uuid),
    CONSTRAINT uk_forum_idempotency_record_key UNIQUE (tenant_id, idempotency_key)
);

CREATE INDEX idx_forum_idempotency_record_tenant_expires
    ON forum_idempotency_record (tenant_id, expires_at, id);

CREATE INDEX idx_forum_idempotency_record_tenant_operation_created
    ON forum_idempotency_record (tenant_id, operation_id, created_at, id);

-- --------------------------------------------------------------------------

CREATE TABLE forum_audit_action (
    id              BIGINT        NOT NULL,
    uuid            UUID          NOT NULL,
    tenant_id       BIGINT        NOT NULL,
    organization_id BIGINT        NOT NULL,
    data_scope      VARCHAR(32)   NOT NULL,
    status          VARCHAR(32)   NOT NULL,
    version         BIGINT        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL,
    updated_at      TIMESTAMPTZ   NOT NULL,
    action          VARCHAR(120)  NOT NULL,
    target_type     VARCHAR(32)   NOT NULL,
    target_id       BIGINT        NOT NULL,
    operator_id     BIGINT        NOT NULL,
    detail          VARCHAR(2000),
    request_id      VARCHAR(128),
    CONSTRAINT pk_forum_audit_action PRIMARY KEY (id),
    CONSTRAINT uk_forum_audit_action_uuid UNIQUE (uuid)
);

CREATE INDEX idx_forum_audit_action_tenant_target
    ON forum_audit_action (tenant_id, target_type, target_id, created_at);

CREATE INDEX idx_forum_audit_action_tenant_operator
    ON forum_audit_action (tenant_id, operator_id, created_at, id);
