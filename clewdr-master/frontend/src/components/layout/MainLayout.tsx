import React, { ReactNode } from "react";
import Header from "./Header";
import Footer from "./Footer";
import ToastProvider from "../common/ToastProvider";

interface MainLayoutProps {
  children: ReactNode;
  version: string;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children, version }) => {
  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white">
      <ToastProvider />
      <div className="w-full px-4 sm:px-6 md:px-8 py-10 mx-auto max-w-full sm:max-w-4xl lg:max-w-5xl xl:max-w-6xl">
        <Header version={version} />
        <main>{children}</main>
        <Footer />
      </div>
    </div>
  );
};

export default MainLayout;
