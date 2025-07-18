import React from "react";
import { useTranslation } from "react-i18next";

const LanguageSelector: React.FC = () => {
  const { i18n } = useTranslation();

  const changeLanguage = (lng: string) => {
    i18n.changeLanguage(lng);
  };

  return (
    <div className="inline-flex items-center space-x-2">
      <button
        onClick={() => changeLanguage("en")}
        className={`p-1 rounded-md text-xs ${
          i18n.language === "en" || i18n.language.startsWith("en-")
            ? "bg-cyan-700 text-white"
            : "bg-gray-700 text-gray-300 hover:bg-gray-600"
        }`}
        aria-label="Switch to English"
      >
        EN
      </button>
      <button
        onClick={() => changeLanguage("zh")}
        className={`p-1 rounded-md text-xs ${
          i18n.language === "zh" || i18n.language.startsWith("zh-")
            ? "bg-cyan-700 text-white"
            : "bg-gray-700 text-gray-300 hover:bg-gray-600"
        }`}
        aria-label="Switch to Chinese"
      >
        中文
      </button>
    </div>
  );
};

export default LanguageSelector;
