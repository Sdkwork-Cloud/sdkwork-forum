export class TokenManager {
  private static readonly AUTH_TOKEN_KEY = 'auth_token'
  private static readonly REFRESH_TOKEN_KEY = 'refresh_token'

  static getAuthToken(): string | null {
    return localStorage.getItem(this.AUTH_TOKEN_KEY)
  }

  static setAuthToken(token: string): void {
    localStorage.setItem(this.AUTH_TOKEN_KEY, token)
  }

  static getRefreshToken(): string | null {
    return localStorage.getItem(this.REFRESH_TOKEN_KEY)
  }

  static setRefreshToken(token: string): void {
    localStorage.setItem(this.REFRESH_TOKEN_KEY, token)
  }

  static clearTokens(): void {
    localStorage.removeItem(this.AUTH_TOKEN_KEY)
    localStorage.removeItem(this.REFRESH_TOKEN_KEY)
  }

  static isAuthenticated(): boolean {
    return !!this.getAuthToken()
  }
}
