import React from "react";

interface ConfigCheckboxProps {
  name: string;
  checked: boolean;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  label: string;
}

const ConfigCheckbox: React.FC<ConfigCheckboxProps> = ({
  name,
  checked,
  onChange,
  label,
}) => {
  return (
    <label className="flex items-center space-x-2 cursor-pointer">
      <input
        type="checkbox"
        name={name}
        checked={checked}
        onChange={onChange}
        className="w-4 h-4 bg-gray-800 border-gray-600 rounded text-cyan-500 focus:ring-cyan-500 focus:ring-opacity-25"
      />
      <span className="text-sm text-gray-300">{label}</span>
    </label>
  );
};

export default ConfigCheckbox;
