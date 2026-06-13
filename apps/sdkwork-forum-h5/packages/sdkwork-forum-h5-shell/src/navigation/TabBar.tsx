import React from 'react'

interface Tab {
  key: string
  label: string
}

interface TabBarProps {
  tabs: Tab[]
  activeKey: string
  onChange: (key: string) => void
}

export function TabBar({ tabs, activeKey, onChange }: TabBarProps) {
  return (
    <div style={{
      display: 'flex',
      background: '#fff',
      borderBottom: '1px solid #e8e8e8',
      padding: '0 16px',
    }}>
      {tabs.map((tab) => (
        <button
          key={tab.key}
          onClick={() => onChange(tab.key)}
          style={{
            flex: 1,
            padding: '12px 0',
            background: 'none',
            border: 'none',
            borderBottom: activeKey === tab.key ? '2px solid #1890ff' : '2px solid transparent',
            color: activeKey === tab.key ? '#1890ff' : '#666',
            fontWeight: activeKey === tab.key ? 600 : 400,
            cursor: 'pointer',
            transition: 'all 0.2s',
          }}
        >
          {tab.label}
        </button>
      ))}
    </div>
  )
}
