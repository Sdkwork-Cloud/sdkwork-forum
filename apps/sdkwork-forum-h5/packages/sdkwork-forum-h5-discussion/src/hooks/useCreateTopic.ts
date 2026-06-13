import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { forumService } from '../services/forumService'

interface CreateTopicData {
  title: string
  body: string
  board_id: number
  body_format: string
  topic_type: string
  visibility: string
}

export function useCreateTopic() {
  const navigate = useNavigate()
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  async function createTopic(data: CreateTopicData) {
    if (!data.title.trim() || !data.body.trim()) {
      setError('请填写标题和内容')
      return null
    }

    setSubmitting(true)
    setError(null)

    try {
      const topic = await forumService.createTopic(data)
      navigate(`/topic/${topic.id}`)
      return topic
    } catch (err) {
      setError('发布失败，请重试')
      return null
    } finally {
      setSubmitting(false)
    }
  }

  return { createTopic, submitting, error }
}
