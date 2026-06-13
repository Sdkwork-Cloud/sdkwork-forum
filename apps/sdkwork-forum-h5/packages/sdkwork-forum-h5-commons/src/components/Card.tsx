import React, { ReactNode } from 'react'

interface CardProps {
  children: ReactNode
  style?: React.CSSProperties
  onClick?: () => void
}

export function Card({ children, style, onClick }: CardProps) {
  return (
    <div
      style={{
        background: '#fff',
        borderRadius: '8px',
        padding: '16px',
        boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
        cursor: onClick ? 'pointer' : undefined,
        transition: 'transform 0.2s, box-shadow 0.2s',
        ...style,
      }}
      onClick={onClick}
    >
      {children}
    </div>
  )
}
