import axios, { AxiosInstance } from 'axios'
import { getEnvironment } from '../config/environment'

export interface ForumSdkClient {
  http: AxiosInstance
}

let sdkClient: ForumSdkClient | null = null

export function createSdkClient(): ForumSdkClient {
  const env = getEnvironment()
  
  const http = axios.create({
    baseURL: env.apiBaseUrl,
    headers: {
      'Content-Type': 'application/json',
    },
  })

  return { http }
}

export function getSdkClient(): ForumSdkClient {
  if (!sdkClient) {
    sdkClient = createSdkClient()
  }
  return sdkClient
}
