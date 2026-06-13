export interface ForumAppSdkDependencies {
  forum: ForumGeneratedClient;
  appbase: AppbaseClient;
  drive: DriveClient;
  search: SearchClient;
  messaging: MessagingClient;
}

export interface ForumGeneratedClient {
  topics: {
    list(params: { boardId?: string; cursor?: string; limit?: number; sort?: string }): Promise<TopicPage>;
    create(body: CreateTopicRequest): Promise<ForumTopic>;
    retrieve(topicId: string): Promise<ForumTopic>;
    update(topicId: string, body: UpdateTopicRequest): Promise<ForumTopic>;
    delete(topicId: string): Promise<void>;
    replies: {
      list(topicId: string, params?: { cursor?: string; limit?: number }): Promise<ReplyPage>;
      create(topicId: string, body: CreateReplyRequest): Promise<ForumReply>;
    };
    revisions: {
      list(topicId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage>;
    };
  };
  replies: {
    update(replyId: string, body: UpdateReplyRequest): Promise<ForumReply>;
    delete(replyId: string): Promise<void>;
    revisions: {
      list(replyId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage>;
    };
  };
  nodes: {
    tree(params?: { spaceId?: string; parentId?: string }): Promise<ForumNode[]>;
  };
  questions: {
    acceptedReply: {
      update(topicId: string, body: { replyId: string }): Promise<ForumTopic>;
      delete(topicId: string): Promise<void>;
    };
  };
  polls: {
    votes: {
      create(pollId: string, body: { optionIds: string[] }): Promise<void>;
    };
  };
  reactions: { create(body: { targetType: string; targetId: string; reactionType: string }): Promise<void> };
  votes: { create(body: { targetType: string; targetId: string; voteValue: number }): Promise<void> };
  bookmarks: { create(body: { targetType: string; targetId: string; note?: string }): Promise<void> };
  readState: {
    topics: { update(topicId: string, body?: { lastReadReplyId?: string }): Promise<void> };
  };
  reports: { create(body: CreateReportRequest): Promise<void> };
  feed: { list(params?: { feedType?: string; cursor?: string; limit?: number }): Promise<FeedPage> };
  search: { query(params: { q: string; cursor?: string; limit?: number }): Promise<SearchResultPage> };
}

export interface AppbaseClient {
  getCurrentUser(): Promise<{ userId: string; tenantId: string; organizationId: string }>;
}

export interface DriveClient {
  validateMediaReference(mediaResourceId: string): Promise<boolean>;
  createDownloadGrant(mediaResourceId: string): Promise<{ grantId: string; url: string }>;
}

export interface SearchClient {
  indexDocument(sourceType: string, sourceId: string): Promise<void>;
  deleteDocument(sourceType: string, sourceId: string): Promise<void>;
}

export interface MessagingClient {
  publishEvent(eventType: string, aggregateId: string): Promise<void>;
}

export interface ForumTopic {
  id: string;
  uuid: string;
  boardId: string;
  title: string;
  bodyFormat: string;
  body: string;
  topicType: string;
  moderationStatus: string;
  visibility: string;
  version: number;
  createdAt: string;
  updatedAt: string;
}

export interface ForumReply {
  id: string;
  uuid: string;
  topicId: string;
  replyNo: number;
  bodyFormat: string;
  body: string;
  moderationStatus: string;
  version: number;
  createdAt: string;
  updatedAt: string;
}

export interface ForumNode {
  id: string;
  parentId: string | null;
  nodeType: string;
  slug: string;
  name: string;
  levelNo: number;
  sortOrder: number;
}

export interface CreateTopicRequest {
  boardId: string;
  title: string;
  bodyFormat: string;
  body: string;
  tagIds?: string[];
  prefixId?: string;
  topicType?: string;
  visibility?: string;
}

export interface UpdateTopicRequest {
  title?: string;
  bodyFormat?: string;
  body?: string;
  editReason?: string;
}

export interface CreateReplyRequest {
  parentReplyId?: string;
  bodyFormat: string;
  body: string;
}

export interface UpdateReplyRequest {
  bodyFormat?: string;
  body: string;
  editReason?: string;
}

export interface CreateReportRequest {
  targetType: string;
  targetId: string;
  reasonCode: string;
  description?: string;
}

export interface TopicPage {
  items: ForumTopic[];
  nextCursor: string | null;
  hasMore: boolean;
}

export interface ReplyPage {
  items: ForumReply[];
  nextCursor: string | null;
  hasMore: boolean;
}

export interface RevisionPage {
  items: Array<{
    id: string;
    revisionNo: number;
    editorUserId: string;
    title?: string;
    bodyFormat: string;
    body: string;
    editReason: string | null;
    createdAt: string;
  }>;
  nextCursor: string | null;
  hasMore: boolean;
}

export interface FeedPage {
  items: Array<{
    id: string;
    feedType: string;
    topicId: string;
    rankScore: string;
    activityAt: string;
  }>;
  nextCursor: string | null;
  hasMore: boolean;
}

export interface SearchResultPage {
  items: Array<{
    id: string;
    sourceType: string;
    sourceId: string;
    title: string | null;
    visibility: string;
  }>;
  nextCursor: string | null;
  hasMore: boolean;
}

export class ForumAppFacade {
  constructor(private readonly deps: ForumAppSdkDependencies) {}

  async listBoardTopics(boardId: string, params?: { cursor?: string; limit?: number; sort?: string }): Promise<TopicPage> {
    return this.deps.forum.topics.list({ boardId, ...params });
  }

  async createTopic(body: CreateTopicRequest): Promise<ForumTopic> {
    if (body.tagIds && body.tagIds.length > 0) {
      for (const tagId of body.tagIds) {
        await this.deps.drive.validateMediaReference(tagId);
      }
    }
    return this.deps.forum.topics.create(body);
  }

  async createReply(topicId: string, body: CreateReplyRequest): Promise<ForumReply> {
    const user = await this.deps.appbase.getCurrentUser();
    if (!user.userId) {
      throw new Error("Authentication required to create reply");
    }
    return this.deps.forum.topics.replies.create(topicId, body);
  }

  async retrieveTopic(topicId: string): Promise<ForumTopic> {
    return this.deps.forum.topics.retrieve(topicId);
  }

  async updateTopic(topicId: string, body: UpdateTopicRequest): Promise<ForumTopic> {
    return this.deps.forum.topics.update(topicId, body);
  }

  async deleteTopic(topicId: string): Promise<void> {
    return this.deps.forum.topics.delete(topicId);
  }

  async listReplies(topicId: string, params?: { cursor?: string; limit?: number }): Promise<ReplyPage> {
    return this.deps.forum.topics.replies.list(topicId, params);
  }

  async updateReply(replyId: string, body: UpdateReplyRequest): Promise<ForumReply> {
    return this.deps.forum.replies.update(replyId, body);
  }

  async deleteReply(replyId: string): Promise<void> {
    return this.deps.forum.replies.delete(replyId);
  }

  async listTopicRevisions(topicId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage> {
    return this.deps.forum.topics.revisions.list(topicId, params);
  }

  async listReplyRevisions(replyId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage> {
    return this.deps.forum.replies.revisions.list(replyId, params);
  }

  async acceptReply(topicId: string, replyId: string): Promise<ForumTopic> {
    return this.deps.forum.questions.acceptedReply.update(topicId, { replyId });
  }

  async clearAcceptedReply(topicId: string): Promise<void> {
    return this.deps.forum.questions.acceptedReply.delete(topicId);
  }

  async votePoll(pollId: string, optionIds: string[]): Promise<void> {
    return this.deps.forum.polls.votes.create(pollId, { optionIds });
  }

  async createReaction(targetType: string, targetId: string, reactionType: string): Promise<void> {
    return this.deps.forum.reactions.create({ targetType, targetId, reactionType });
  }

  async createVote(targetType: string, targetId: string, voteValue: number): Promise<void> {
    return this.deps.forum.votes.create({ targetType, targetId, voteValue });
  }

  async updateBookmark(targetType: string, targetId: string, note?: string): Promise<void> {
    return this.deps.forum.bookmarks.create({ targetType, targetId, note });
  }

  async updateReadState(topicId: string, lastReadReplyId?: string): Promise<void> {
    return this.deps.forum.readState.topics.update(topicId, { lastReadReplyId });
  }

  async createReport(body: CreateReportRequest): Promise<void> {
    return this.deps.forum.reports.create(body);
  }

  async listFeed(params?: { feedType?: string; cursor?: string; limit?: number }): Promise<FeedPage> {
    return this.deps.forum.feed.list(params);
  }

  async search(query: string, params?: { cursor?: string; limit?: number }): Promise<SearchResultPage> {
    return this.deps.forum.search.query({ q: query, ...params });
  }

  async listNodeTree(params?: { spaceId?: string; parentId?: string }): Promise<ForumNode[]> {
    return this.deps.forum.nodes.tree(params);
  }
}
