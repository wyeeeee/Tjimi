import { Toaster } from "react-hot-toast";

const ToastProvider = () => {
  return (
    <Toaster
      position="top-right"
      toastOptions={{
        duration: 3000,
        style: {
          background: "#2D3748",
          color: "#fff",
          border: "1px solid #4A5568",
        },
        success: {
          iconTheme: {
            primary: "#38B2AC",
            secondary: "#fff",
          },
        },
        error: {
          iconTheme: {
            primary: "#F56565",
            secondary: "#fff",
          },
        },
      }}
    />
  );
};

export default ToastProvider;
