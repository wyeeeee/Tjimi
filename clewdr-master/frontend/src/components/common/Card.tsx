import React from "react";
import { CardProps } from "../../types/ui.types";

const Card: React.FC<CardProps> = ({ children, className = "" }) => {
  return (
    <div
      className={`bg-gray-800/50 rounded-xl shadow-xl p-6 border border-gray-700 backdrop-blur-sm ${className}`}
    >
      {children}
    </div>
  );
};

export default Card;
