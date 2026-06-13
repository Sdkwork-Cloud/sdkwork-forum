import { useState, useEffect } from 'react'
import { forumService, Topic } from '../services/forumService'

export function useTopics() {
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadTopics()
  }, [])

  async function loadTopics() {
    try {
      setLoading(true)
      setError(null)
      const data = await forumService.listTopics()
      setTopics(data)
    } catch (err) {
      setError('加载话题失败')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  return { topics, loading, error, refresh: loadTopics }
}
