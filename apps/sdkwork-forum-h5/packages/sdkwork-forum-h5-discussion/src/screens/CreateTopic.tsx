import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Card, Button } from '@sdkwork/forum-h5-commons'
import { useCreateTopic } from '../hooks/useCreateTopic'

export function CreateTopic() {
  const navigate = useNavigate()
  const { createTopic, submitting, error } = useCreateTopic()
  const [formData, setFormData] = useState({
    title: '',
    body: '',
    board_id: 1,
    body_format: 'markdown',
    topic_type: 'discussion',
    visibility: 'public',
  })

  function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) {
    setFormData({ ...formData, [e.target.name]: e.target.value })
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    await createTopic(formData)
  }

  return (
    <div style={{ padding: '16px' }}>
      <Card>
        <h2 style={{ fontSize: '20px', fontWeight: 600, marginBottom: '20px' }}>发布新话题</h2>
        
        {error && (
          <div style={{ padding: '12px', background: '#fff2f0', border: '1px solid #ffccc7', borderRadius: '6px', marginBottom: '16px', color: '#ff4d4f' }}>
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <div style={{ marginBottom: '16px' }}>
            <label style={{ display: 'block', marginBottom: '6px', fontWeight: 500 }}>标题</label>
            <input
              type="text"
              name="title"
              value={formData.title}
              onChange={handleChange}
              placeholder="请输入话题标题"
              maxLength={240}
              style={{
                width: '100%',
                padding: '10px',
                border: '1px solid #e8e8e8',
                borderRadius: '6px',
                fontSize: '14px',
              }}
            />
          </div>

          <div style={{ marginBottom: '16px' }}>
            <label style={{ display: 'block', marginBottom: '6px', fontWeight: 500 }}>话题类型</label>
            <select
              name="topic_type"
              value={formData.topic_type}
              onChange={handleChange}
              style={{
                width: '100%',
                padding: '10px',
                border: '1px solid #e8e8e8',
                borderRadius: '6px',
                fontSize: '14px',
              }}
            >
              <option value="discussion">💬 讨论</option>
              <option value="question">❓ 问答</option>
              <option value="article">📝 文章</option>
            </select>
          </div>

          <div style={{ marginBottom: '16px' }}>
            <label style={{ display: 'block', marginBottom: '6px', fontWeight: 500 }}>所属版块</label>
            <select
              name="board_id"
              value={formData.board_id}
              onChange={handleChange}
              style={{
                width: '100%',
                padding: '10px',
                border: '1px solid #e8e8e8',
                borderRadius: '6px',
                fontSize: '14px',
              }}
            >
              <option value={1}>综合讨论</option>
              <option value={2}>API 设计</option>
              <option value={3}>SDK 集成</option>
            </select>
          </div>

          <div style={{ marginBottom: '16px' }}>
            <label style={{ display: 'block', marginBottom: '6px', fontWeight: 500 }}>内容</label>
            <textarea
              name="body"
              value={formData.body}
              onChange={handleChange}
              placeholder="请输入话题内容（支持 Markdown 格式）"
              style={{
                width: '100%',
                minHeight: '200px',
                padding: '10px',
                border: '1px solid #e8e8e8',
                borderRadius: '6px',
                fontSize: '14px',
                resize: 'vertical',
              }}
            />
          </div>

          <div style={{ marginBottom: '20px' }}>
            <label style={{ display: 'block', marginBottom: '6px', fontWeight: 500 }}>可见性</label>
            <select
              name="visibility"
              value={formData.visibility}
              onChange={handleChange}
              style={{
                width: '100%',
                padding: '10px',
                border: '1px solid #e8e8e8',
                borderRadius: '6px',
                fontSize: '14px',
              }}
            >
              <option value="public">🌐 公开</option>
              <option value="members">👥 仅会员</option>
              <option value="private">🔒 私密</option>
            </select>
          </div>

          <div style={{ display: 'flex', gap: '12px', justifyContent: 'flex-end' }}>
            <Button type="button" variant="secondary" onClick={() => navigate('/')}>
              取消
            </Button>
            <Button type="submit" disabled={submitting}>
              {submitting ? '发布中...' : '发布话题'}
            </Button>
          </div>
        </form>
      </Card>
    </div>
  )
}
