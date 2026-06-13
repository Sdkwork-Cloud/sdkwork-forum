import { createSdkClient, getSdkClient, type ForumSdkClient } from '@sdkwork/forum-h5-core'

export type { ForumSdkClient }

export function initSdkClient(): ForumSdkClient {
  return createSdkClient()
}

export { getSdkClient }
