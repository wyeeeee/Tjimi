import React from "react";
import { Tab } from "../../types/ui.types";

interface TabNavigationProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  className?: string;
}
const TabNavigation: React.FC<TabNavigationProps> = ({
  tabs,
  activeTab,
  onTabChange,
  className = "",
}) => {
  const getTabStyle = (tab: Tab) => {
    // Use explicit classes for each color instead of dynamic string interpolation
    // This ensures the classes are preserved by TailwindCSS purge process
    if (activeTab === tab.id) {
      switch (tab.color) {
        case "cyan":
          return "text-cyan-400 border-b-2 border-cyan-400";
        case "green":
          return "text-green-400 border-b-2 border-green-400";
        case "purple":
          return "text-purple-400 border-b-2 border-purple-400";
        case "blue":
          return "text-blue-400 border-b-2 border-blue-400";
        case "amber":
          return "text-amber-400 border-b-2 border-amber-400";
        default:
          return "text-cyan-400 border-b-2 border-cyan-400";
      }
    }
    return "text-gray-400 hover:text-gray-300";
  };

  return (
    <div className={`flex border-b border-gray-700 ${className}`}>
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onTabChange(tab.id)}
          className={`flex-1 py-2 font-medium text-sm transition-colors duration-200 ${getTabStyle(
            tab,
          )}`}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
};

export default TabNavigation;
