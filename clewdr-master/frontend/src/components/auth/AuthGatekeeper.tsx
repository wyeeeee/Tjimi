// frontend/src/components/auth/AuthGatekeeper.tsx
import React, { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import Button from "../common/Button";
import FormInput from "../common/FormInput";
import StatusMessage from "../common/StatusMessage";
import { useAuth } from "../../hooks/useAuth";

interface AuthGatekeeperProps {
  onAuthenticated?: (status: boolean) => void;
}

const AuthGatekeeper: React.FC<AuthGatekeeperProps> = ({ onAuthenticated }) => {
  const { t } = useTranslation();
  const {
    authToken,
    setAuthToken,
    isLoading,
    error,
    savedToken,
    login,
    logout,
  } = useAuth(onAuthenticated);

  const [statusMessage, setStatusMessage] = useState({
    type: "info" as "success" | "error" | "warning" | "info",
    message: "",
  });

  // Update status message when error changes
  useEffect(() => {
    if (error) {
      setStatusMessage({
        type: "error",
        message: error,
      });
    }
  }, [error]);

  // Show persistent success message after login
  // (we don't clear it automatically)
  const handleLoginSuccess = () => {
    setStatusMessage({
      type: "success",
      message: t("auth.success"),
    });
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setStatusMessage({ type: "info", message: "" });

    if (!authToken.trim()) {
      setStatusMessage({
        type: "warning",
        message: t("auth.enterToken"),
      });
      return;
    }

    try {
      await login(authToken);
      handleLoginSuccess();
    } catch {
      // Error is already handled in the useAuth hook
      // and will be displayed via the useEffect
    }
  };

  const handleClearToken = () => {
    logout();
    setStatusMessage({
      type: "info",
      message: t("auth.tokenCleared"),
    });
  };

  return (
    <div>
      <form onSubmit={handleSubmit} className="space-y-6">
        <FormInput
          id="authToken"
          name="authToken"
          type="password"
          value={authToken}
          onChange={(e) => setAuthToken(e.target.value)}
          label={t("auth.token")}
          placeholder={t("auth.tokenPlaceholder")}
          disabled={isLoading}
          onClear={() => setAuthToken("")}
        />

        {savedToken && (
          <div className="flex items-center justify-between mt-2">
            <p className="text-xs text-gray-400">
              {t("auth.previousToken")}{" "}
              <span className="font-mono">{savedToken}</span>
            </p>
            <button
              type="button"
              onClick={handleClearToken}
              className="text-xs text-red-400 hover:text-red-300"
              disabled={isLoading}
            >
              {t("auth.clear")}
            </button>
          </div>
        )}

        {statusMessage.message && (
          <StatusMessage
            type={statusMessage.type}
            message={statusMessage.message}
          />
        )}

        <Button
          type="submit"
          isLoading={isLoading}
          disabled={isLoading}
          className="w-full"
          variant="primary"
        >
          {isLoading ? t("auth.verifying") : t("auth.submitButton")}
        </Button>
      </form>
    </div>
  );
};

export default AuthGatekeeper;
