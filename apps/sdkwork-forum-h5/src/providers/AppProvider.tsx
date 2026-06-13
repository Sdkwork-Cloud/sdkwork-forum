import React, { createContext, useContext, ReactNode, useMemo } from 'react'
import { createSdkClient, ForumSdkClient, TokenManager } from '@sdkwork/forum-h5-core'
import { getEnvironment, Environment } from '@sdkwork/forum-h5-core'

interface AppContextValue {
  sdkClient: ForumSdkClient
  environment: Environment
}

const AppContext = createContext<AppContextValue | null>(null)

export function AppProvider({ children }: { children: ReactNode }) {
  const value = useMemo<AppContextValue>(() => {
    const sdkClient = createSdkClient()
    
    // Wire TokenManager to SDK client
    sdkClient.http.interceptors.request.use((config) => {
      const token = TokenManager.getAuthToken()
      if (token) {
        config.headers.Authorization = `Bearer ${token}`
      }
      return config
    })

    sdkClient.http.interceptors.response.use(
      (response) => response,
      (error) => {
        if (error.response?.status === 401) {
          TokenManager.clearTokens()
          window.location.href = '/login'
        }
        return Promise.reject(error)
      }
    )

    return {
      sdkClient,
      environment: getEnvironment(),
    }
  }, [])

  return (
    <AppContext.Provider value={value}>
      {children}
    </AppContext.Provider>
  )
}

export function useAppContext(): AppContextValue {
  const context = useContext(AppContext)
  if (!context) {
    throw new Error('useAppContext must be used within AppProvider')
  }
  return context
}
