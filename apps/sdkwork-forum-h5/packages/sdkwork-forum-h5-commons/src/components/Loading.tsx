import React from 'react'

export function Loading({ text = '加载中...' }: { text?: string }) {
  return (
    <div style={{ textAlign: 'center', padding: '40px', color: '#666' }}>
      <p>{text}</p>
    </div>
  )
}
