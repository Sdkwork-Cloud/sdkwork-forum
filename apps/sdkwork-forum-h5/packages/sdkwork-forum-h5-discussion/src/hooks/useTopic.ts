import { useState, useEffect } from 'react'
import { forumService, Topic } from '../services/forumService'

export function useTopic(id: number) {
  const [topic, setTopic] = useState<Topic | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadTopic()
  }, [id])

  async function loadTopic() {
    try {
      setLoading(true)
      setError(null)
      const data = await forumService.getTopic(id)
      setTopic(data)
    } catch (err) {
      setError('加载话题详情失败')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  async function vote() {
    try {
      await forumService.voteTopic(id)
      if (topic) {
        setTopic({ ...topic, vote_score: topic.vote_score + 1 })
      }
    } catch (err) {
      console.error('投票失败:', err)
    }
  }

  async function bookmark() {
    try {
      await forumService.bookmarkTopic(id)
      alert('已收藏')
    } catch (err) {
      console.error('收藏失败:', err)
    }
  }

  return { topic, loading, error, vote, bookmark, refresh: loadTopic }
}
