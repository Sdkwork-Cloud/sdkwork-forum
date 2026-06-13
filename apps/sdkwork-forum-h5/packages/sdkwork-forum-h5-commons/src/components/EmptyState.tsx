import React from 'react'

interface EmptyStateProps {
  message?: string
  action?: React.ReactNode
}

export function EmptyState({ message = '暂无数据', action }: EmptyStateProps) {
  return (
    <div style={{ textAlign: 'center', padding: '60px 20px', color: '#666' }}>
      <p style={{ fontSize: '16px', marginBottom: '16px' }}>{message}</p>
      {action}
    </div>
  )
}
