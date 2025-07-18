export interface ApiResponse<T> {
  data?: T;
  error?: string;
  status: number;
}

export interface CookieSubmitRequest {
  cookie: string;
}

export interface VersionResponse {
  version: string;
}
