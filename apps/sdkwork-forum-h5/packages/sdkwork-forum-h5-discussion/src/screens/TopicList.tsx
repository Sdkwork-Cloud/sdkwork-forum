import React from 'react'
import { Link } from 'react-router-dom'
import { Loading, EmptyState, Card, Button, formatRelativeTime } from '@sdkwork/forum-h5-commons'
import { useTopics } from '../hooks/useTopics'

export function TopicList() {
  const { topics, loading, error } = useTopics()

  if (loading) return <Loading />
  if (error) return <EmptyState message={error} />

  return (
    <div style={{ padding: '16px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
        <h2 style={{ fontSize: '20px', fontWeight: 600 }}>最新话题</h2>
        <Link to="/create">
          <Button>发布新话题</Button>
        </Link>
      </div>

      {topics.length === 0 ? (
        <EmptyState
          message="暂无话题"
          action={
            <Link to="/create">
              <Button>发布第一个话题</Button>
            </Link>
          }
        />
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          {topics.map((topic) => (
            <Link key={topic.id} to={`/topic/${topic.id}`} style={{ textDecoration: 'none' }}>
              <Card>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', marginBottom: '8px' }}>
                  <h3 style={{ fontSize: '16px', fontWeight: 600, color: '#333', flex: 1 }}>
                    {topic.title}
                  </h3>
                  <span style={{
                    padding: '2px 8px',
                    borderRadius: '4px',
                    fontSize: '12px',
                    background: topic.topic_type === 'question' ? '#fff7e6' : '#e6f7ff',
                    color: topic.topic_type === 'question' ? '#faad14' : '#1890ff',
                    marginLeft: '8px',
                  }}>
                    {topic.topic_type === 'discussion' ? '讨论' : topic.topic_type === 'question' ? '问答' : '文章'}
                  </span>
                </div>
                <p style={{ color: '#666', fontSize: '14px', marginBottom: '12px', lineHeight: 1.5 }}>
                  {topic.body.substring(0, 100)}...
                </p>
                <div style={{ display: 'flex', gap: '16px', color: '#999', fontSize: '13px' }}>
                  <span>👁️ {topic.view_count}</span>
                  <span>💬 {topic.reply_count}</span>
                  <span>👍 {topic.vote_score}</span>
                  <span>🕐 {formatRelativeTime(topic.last_activity_at)}</span>
                </div>
              </Card>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
