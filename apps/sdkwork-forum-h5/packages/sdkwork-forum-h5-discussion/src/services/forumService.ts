import { getSdkClient } from '@sdkwork/forum-h5-core'

export interface Topic {
  id: number
  uuid: string
  title: string
  body: string
  body_format: string
  topic_type: string
  moderation_status: string
  visibility: string
  board_id: number
  author_user_id: number
  reply_count: number
  view_count: number
  vote_score: number
  created_at: string
  updated_at: string
  last_activity_at: string
}

export interface Reply {
  id: number
  uuid: string
  topic_id: number
  body: string
  body_format: string
  reply_no: number
  author_user_id: number
  moderation_status: string
  vote_score: number
  created_at: string
}

export interface Board {
  id: number
  uuid: string
  name: string
  description: string
  slug: string
  node_type: string
  topic_count: number
  reply_count: number
}

class ForumService {
  async listTopics(): Promise<Topic[]> {
    const client = getSdkClient()
    const response = await client.http.get('/forum/topics')
    return response.data.data.items || []
  }

  async getTopic(id: number): Promise<Topic> {
    const client = getSdkClient()
    const response = await client.http.get(`/forum/topics/${id}`)
    return response.data.data
  }

  async createTopic(data: {
    title: string
    body: string
    board_id: number
    body_format?: string
    topic_type?: string
    visibility?: string
  }): Promise<Topic> {
    const client = getSdkClient()
    const response = await client.http.post('/forum/topics', data)
    return response.data.data
  }

  async listReplies(topicId: number): Promise<Reply[]> {
    const client = getSdkClient()
    const response = await client.http.get(`/forum/topics/${topicId}/replies`)
    return response.data.data.items || []
  }

  async createReply(topicId: number, data: {
    body: string
    body_format?: string
    parent_reply_id?: number
  }): Promise<Reply> {
    const client = getSdkClient()
    const response = await client.http.post(`/forum/topics/${topicId}/replies`, data)
    return response.data.data
  }

  async voteTopic(topicId: number): Promise<void> {
    const client = getSdkClient()
    await client.http.post(`/forum/topics/${topicId}/vote`)
  }

  async bookmarkTopic(topicId: number): Promise<void> {
    const client = getSdkClient()
    await client.http.post(`/forum/topics/${topicId}/bookmark`)
  }

  async listBoards(): Promise<Board[]> {
    const client = getSdkClient()
    const response = await client.http.get('/forum/boards')
    return response.data.data.items || []
  }
}

export const forumService = new ForumService()
