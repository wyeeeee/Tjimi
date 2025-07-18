import React, { ReactNode } from "react";

interface ConfigSectionProps {
  title: string;
  description?: string;
  children: ReactNode;
}

const ConfigSection: React.FC<ConfigSectionProps> = ({
  title,
  description,
  children,
}) => {
  return (
    <div className="bg-gray-700/60 p-4 rounded-lg">
      <h4 className="text-md font-medium text-cyan-300 mb-3">{title}</h4>
      {description && (
        <p className="text-xs text-gray-400 mb-3">{description}</p>
      )}
      <div className="space-y-4">{children}</div>
    </div>
  );
};

export default ConfigSection;
