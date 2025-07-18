import React from "react";
import { FormInputProps } from "../../types/ui.types";

const FormInput: React.FC<FormInputProps> = ({
  id,
  name,
  type = "text",
  value,
  onChange,
  label,
  placeholder,
  disabled = false,
  className = "",
  isTextarea = false,
  rows = 3,
  error,
  onClear,
}) => {
  const inputClasses = `w-full bg-gray-700 border border-gray-600 rounded-md focus:ring-2 focus:ring-cyan-500 focus:border-cyan-500 text-sm text-gray-200 transition-all duration-200 placeholder-gray-400 ${
    isTextarea ? "min-h-32" : ""
  } ${className} ${error ? "border-red-500" : ""}`;

  return (
    <div className="space-y-2">
      {label && (
        <label
          htmlFor={id}
          className="block text-sm font-medium text-gray-300 mb-1"
        >
          {label}
        </label>
      )}
      <div className="relative">
        {isTextarea ? (
          <textarea
            id={id}
            name={name}
            value={value}
            onChange={onChange}
            placeholder={placeholder}
            className={`p-4 ${inputClasses}`}
            disabled={disabled}
            rows={rows}
          />
        ) : (
          <input
            id={id}
            name={name}
            type={type}
            value={value}
            onChange={onChange}
            placeholder={placeholder}
            className={`p-4 ${inputClasses}`}
            disabled={disabled}
          />
        )}
        {value && onClear && (
          <button
            type="button"
            className="absolute top-3 right-2 text-gray-400 hover:text-gray-200"
            onClick={onClear}
            disabled={disabled}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-5 w-5"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fillRule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                clipRule="evenodd"
              />
            </svg>
          </button>
        )}
      </div>
      {error && <p className="text-red-500 text-xs mt-1">{error}</p>}
    </div>
  );
};

export default FormInput;
