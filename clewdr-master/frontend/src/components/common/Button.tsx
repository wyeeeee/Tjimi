import React from "react";
import { ButtonProps } from "../../types/ui.types";

const Button: React.FC<ButtonProps> = ({
  type = "button",
  onClick,
  disabled = false,
  isLoading = false,
  variant = "primary",
  className = "",
  children,
}) => {
  // Base classes
  const baseClasses =
    "py-3 px-4 rounded-md text-white font-medium transition-all duration-200";

  // Variant-specific classes
  const variantClasses = {
    primary: "bg-purple-600 hover:bg-purple-500 focus:ring-purple-500",
    secondary: "bg-gray-600 hover:bg-gray-500 focus:ring-gray-500",
    danger: "bg-red-600 hover:bg-red-500 focus:ring-red-500",
    success: "bg-green-600 hover:bg-green-500 focus:ring-green-500",
  };

  // Disabled/loading classes
  const stateClasses =
    disabled || isLoading
      ? "opacity-70 cursor-not-allowed"
      : "focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 shadow-md hover:shadow-lg";

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled || isLoading}
      className={`${baseClasses} ${variantClasses[variant]} ${stateClasses} ${className}`}
    >
      {isLoading ? (
        <div className="flex items-center justify-center">
          <svg
            className="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            ></circle>
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          {isLoading && typeof children === "string" ? children : "Loading..."}
        </div>
      ) : (
        children
      )}
    </button>
  );
};

export default Button;
