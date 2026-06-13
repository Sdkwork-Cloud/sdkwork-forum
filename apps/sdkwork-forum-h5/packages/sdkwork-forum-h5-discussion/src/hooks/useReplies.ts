import { useState, useEffect } from 'react'
import { forumService, Reply } from '../services/forumService'

export function useReplies(topicId: number) {
  const [replies, setReplies] = useState<Reply[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadReplies()
  }, [topicId])

  async function loadReplies() {
    try {
      setLoading(true)
      setError(null)
      const data = await forumService.listReplies(topicId)
      setReplies(data)
    } catch (err) {
      setError('加载回复失败')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  async function addReply(body: string) {
    try {
      const reply = await forumService.createReply(topicId, {
        body,
        body_format: 'markdown',
      })
      setReplies([...replies, reply])
      return reply
    } catch (err) {
      console.error('回复失败:', err)
      throw err
    }
  }

  return { replies, loading, error, addReply, refresh: loadReplies }
}
