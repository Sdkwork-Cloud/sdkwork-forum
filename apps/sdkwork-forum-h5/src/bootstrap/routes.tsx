import React from 'react'
import { RouteObject } from 'react-router-dom'

const TopicList = React.lazy(() => import('@sdkwork/forum-h5-discussion').then(m => ({ default: m.TopicList })))
const TopicDetail = React.lazy(() => import('@sdkwork/forum-h5-discussion').then(m => ({ default: m.TopicDetail })))
const CreateTopic = React.lazy(() => import('@sdkwork/forum-h5-discussion').then(m => ({ default: m.CreateTopic })))

export const appRoutes: RouteObject[] = [
  {
    path: '/',
    element: <TopicList />,
  },
  {
    path: '/topic/:id',
    element: <TopicDetail />,
  },
  {
    path: '/create',
    element: <CreateTopic />,
  },
]
