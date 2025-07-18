// frontend/src/components/common/ErrorBoundary.tsx
import { Component, ErrorInfo, ReactNode } from "react";
import { withTranslation, WithTranslation } from "react-i18next";

interface ErrorBoundaryProps extends WithTranslation {
  children: ReactNode;
  fallback?: ReactNode;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
    };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return {
      hasError: true,
      error,
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    console.error("Error caught by ErrorBoundary:", error, errorInfo);
  }

  handleRetry = (): void => {
    this.setState({
      hasError: false,
      error: null,
    });
  };

  render(): ReactNode {
    const { t } = this.props;

    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div className="p-6 bg-red-900/20 border border-red-600 rounded-lg text-white">
          <h2 className="text-xl font-bold mb-2">
            {t("common.errorBoundary.title")}
          </h2>
          <p className="mb-4">{t("common.errorBoundary.message")}</p>
          {this.state.error && (
            <div className="bg-red-950 p-3 rounded mb-4 font-mono text-sm overflow-auto max-h-32">
              {this.state.error.toString()}
            </div>
          )}
          <button
            onClick={this.handleRetry}
            className="bg-red-600 hover:bg-red-500 text-white py-2 px-4 rounded transition-colors"
          >
            {t("common.errorBoundary.retry")}
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

export default withTranslation()(ErrorBoundary);
