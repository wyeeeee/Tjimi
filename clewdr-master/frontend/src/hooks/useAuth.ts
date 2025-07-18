// frontend/src/hooks/useAuth.ts
import { useState, useEffect } from "react";
import { validateAuthToken } from "../api";
import { maskToken } from "../utils/formatters";

export const useAuth = (onAuthenticated?: (status: boolean) => void) => {
  const [authToken, setAuthToken] = useState("");
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const [savedToken, setSavedToken] = useState("");

  // Check for existing token on mount
  useEffect(() => {
    const storedToken = localStorage.getItem("authToken");
    if (storedToken) {
      setSavedToken(maskToken(storedToken));
      checkToken(storedToken);
    }
  }, []);

  const checkToken = async (token: string) => {
    setIsLoading(true);
    setError("");

    try {
      // Use the centralized API function instead of direct fetch
      const isValid = await validateAuthToken(token);

      if (isValid) {
        setIsAuthenticated(true);
        if (onAuthenticated) {
          onAuthenticated(true);
        }
        return true;
      } else {
        localStorage.removeItem("authToken");
        setSavedToken("");
        setIsAuthenticated(false);
        setError("Invalid token. Please try again.");
        if (onAuthenticated) {
          onAuthenticated(false);
        }
        return false;
      }
    } catch (err) {
      setIsAuthenticated(false);
      setError(err instanceof Error ? err.message : "Unknown error");
      if (onAuthenticated) {
        onAuthenticated(false);
      }
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (token: string) => {
    if (!token.trim()) {
      setError("Please enter an auth token");
      return;
    }

    setIsLoading(true);
    setError("");

    try {
      // Use the centralized API function instead of direct fetch
      const isValid = await validateAuthToken(token);

      if (isValid) {
        // Save token to localStorage immediately upon successful validation
        localStorage.setItem("authToken", token);
        setSavedToken(maskToken(token));
        setAuthToken("");
        setIsAuthenticated(true);
        if (onAuthenticated) {
          onAuthenticated(true);
        }
      } else {
        setIsAuthenticated(false);
        setError("Invalid token. Please try again.");
        if (onAuthenticated) {
          onAuthenticated(false);
        }
      }
    } catch (err) {
      setIsAuthenticated(false);
      setError(err instanceof Error ? err.message : "Unknown error");
      if (onAuthenticated) {
        onAuthenticated(false);
      }
    } finally {
      setIsLoading(false);
    }
  };

  const logout = () => {
    localStorage.removeItem("authToken");
    setSavedToken("");
    setIsAuthenticated(false);
    if (onAuthenticated) {
      onAuthenticated(false);
    }
  };

  return {
    authToken,
    setAuthToken,
    isAuthenticated,
    isLoading,
    error,
    savedToken,
    login,
    logout,
  };
};
