export interface Environment {
  apiBaseUrl: string
  environment: 'development' | 'test' | 'staging' | 'production'
  appId: string
  features: {
    enableNotifications: boolean
    enableSearch: boolean
    enableModeration: boolean
  }
}

const defaultEnvironment: Environment = {
  apiBaseUrl: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/app/v3/api',
  environment: (import.meta.env.VITE_ENVIRONMENT as Environment['environment']) || 'development',
  appId: 'sdkwork-forum-h5',
  features: {
    enableNotifications: true,
    enableSearch: true,
    enableModeration: false,
  },
}

export function getEnvironment(): Environment {
  return defaultEnvironment
}
