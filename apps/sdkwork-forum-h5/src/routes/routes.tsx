import React from 'react'
import { Navigate } from 'react-router-dom'

const TopicList = React.lazy(() => import('@sdkwork/forum-h5-discussion').then(m => ({ default: m.TopicList })))
const TopicDetail = React.lazy(() => import('@sdkwork/forum-h5-discussion').then(m => ({ default: m.TopicDetail })))
const CreateTopic = React.lazy(() => import('@sdkwork/forum-h5-discussion').then(m => ({ default: m.CreateTopic })))

export const routes = [
  {
    path: '/',
    element: (
      <React.Suspense fallback={<div>Loading...</div>}>
        <TopicList />
      </React.Suspense>
    ),
  },
  {
    path: '/topic/:id',
    element: (
      <React.Suspense fallback={<div>Loading...</div>}>
        <TopicDetail />
      </React.Suspense>
    ),
  },
  {
    path: '/create',
    element: (
      <React.Suspense fallback={<div>Loading...</div>}>
        <CreateTopic />
      </React.Suspense>
    ),
  },
  {
    path: '*',
    element: <Navigate to="/" replace />,
  },
]
