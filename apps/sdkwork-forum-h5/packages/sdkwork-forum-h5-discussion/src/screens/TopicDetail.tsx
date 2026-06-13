import React, { useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { Loading, EmptyState, Card, Button, formatDate } from '@sdkwork/forum-h5-commons'
import { useTopic } from '../hooks/useTopic'
import { useReplies } from '../hooks/useReplies'

export function TopicDetail() {
  const { id } = useParams<{ id: string }>()
  const topicId = Number(id)
  const { topic, loading: topicLoading, error: topicError, vote, bookmark } = useTopic(topicId)
  const { replies, loading: repliesLoading, addReply } = useReplies(topicId)
  const [replyText, setReplyText] = useState('')
  const [submitting, setSubmitting] = useState(false)

  if (topicLoading) return <Loading />
  if (topicError || !topic) return <EmptyState message={topicError || '话题不存在'} />

  async function handleReply() {
    if (!replyText.trim()) return
    setSubmitting(true)
    try {
      await addReply(replyText)
      setReplyText('')
    } catch (err) {
      alert('回复失败')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div style={{ padding: '16px' }}>
      <Card style={{ marginBottom: '16px' }}>
        <h1 style={{ fontSize: '22px', fontWeight: 700, marginBottom: '12px' }}>{topic.title}</h1>
        <div style={{ display: 'flex', gap: '16px', color: '#666', fontSize: '13px', marginBottom: '16px' }}>
          <span>作者: 用户{topic.author_user_id}</span>
          <span>{formatDate(topic.created_at)}</span>
          <span>👁️ {topic.view_count}</span>
          <span>👍 {topic.vote_score}</span>
        </div>
        <div style={{ fontSize: '15px', lineHeight: 1.8, color: '#333', marginBottom: '20px' }}>
          {topic.body.split('\n').map((line, i) => (
            <p key={i} style={{ marginBottom: '12px' }}>{line}</p>
          ))}
        </div>
        <div style={{ display: 'flex', gap: '12px', paddingTop: '16px', borderTop: '1px solid #e8e8e8' }}>
          <Button onClick={vote}>👍 点赞</Button>
          <Button variant="secondary" onClick={bookmark}>⭐ 收藏</Button>
          <Link to="/" style={{ marginLeft: 'auto' }}>
            <Button variant="secondary">返回列表</Button>
          </Link>
        </div>
      </Card>

      <div>
        <h3 style={{ fontSize: '18px', fontWeight: 600, marginBottom: '16px', paddingBottom: '8px', borderBottom: '2px solid #1890ff' }}>
          回复 ({replies.length})
        </h3>

        {repliesLoading ? (
          <Loading text="加载回复中..." />
        ) : replies.length === 0 ? (
          <EmptyState message="暂无回复" />
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            {replies.map((reply) => (
              <Card key={reply.id}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                  <span style={{ fontWeight: 600, color: '#1890ff' }}>用户{reply.author_user_id}</span>
                  <span style={{ color: '#999', fontSize: '13px' }}>{formatDate(reply.created_at)}</span>
                </div>
                <p style={{ fontSize: '14px', lineHeight: 1.6, color: '#333' }}>{reply.body}</p>
                <div style={{ display: 'flex', gap: '12px', marginTop: '12px', paddingTop: '8px', borderTop: '1px solid #f0f0f0' }}>
                  <button style={{ background: 'none', border: 'none', color: '#666', cursor: 'pointer', fontSize: '13px' }}>
                    👍 {reply.vote_score}
                  </button>
                  <button style={{ background: 'none', border: 'none', color: '#666', cursor: 'pointer', fontSize: '13px' }}>
                    💬 回复
                  </button>
                </div>
              </Card>
            ))}
          </div>
        )}

        <Card style={{ marginTop: '16px' }}>
          <h4 style={{ marginBottom: '12px' }}>发表回复</h4>
          <textarea
            value={replyText}
            onChange={(e) => setReplyText(e.target.value)}
            placeholder="写下你的回复..."
            style={{
              width: '100%',
              minHeight: '120px',
              padding: '12px',
              border: '1px solid #e8e8e8',
              borderRadius: '6px',
              fontSize: '14px',
              resize: 'vertical',
              marginBottom: '12px',
            }}
          />
          <Button onClick={handleReply} disabled={submitting}>
            {submitting ? '提交中...' : '提交回复'}
          </Button>
        </Card>
      </div>
    </div>
  )
}
