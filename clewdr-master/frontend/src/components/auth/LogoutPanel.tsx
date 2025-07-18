import React from "react";
import { useTranslation } from "react-i18next";
import Button from "../common/Button";

interface LogoutPanelProps {
  onLogout: () => void;
}

const LogoutPanel: React.FC<LogoutPanelProps> = ({ onLogout }) => {
  const { t } = useTranslation();

  return (
    <div className="bg-gray-700 p-6 rounded-lg">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-medium text-white">
          {t("auth.authTitle")}
        </h3>
        <Button
          onClick={onLogout}
          variant="danger"
          className="py-2 px-4 text-sm"
        >
          {t("auth.logout")}
        </Button>
      </div>
      <p className="text-gray-300 text-sm mb-4">{t("auth.loggedInMessage")}</p>
    </div>
  );
};

export default LogoutPanel;
