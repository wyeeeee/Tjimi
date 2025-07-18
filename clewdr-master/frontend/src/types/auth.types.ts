export interface AuthStatus {
  type: "idle" | "success" | "error";
  message: string;
}

export interface AuthProps {
  onAuthenticated?: (status: boolean) => void;
}
