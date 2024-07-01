import { Layout } from "antd";
import { Content } from "antd/es/layout/layout";
import React from "react";

type Props = {
  children: React.ReactNode;
  fallback?: React.ReactNode;
};

type State = {
  error?: Error;
  hasError: boolean;
  path: string;
  dateTime: Date;
};
export class ErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      dateTime: new Date(),
      path: window.location.pathname,
    };
  }

  static getDerivedStateFromError(error: Error) {
    console.error(error);
    return {
      hasError: true,
      error,
      dateTime: new Date(),
      path: window.location.pathname,
    };
  }

  render() {
    const { children } = this.props;
    const { hasError } = this.state;

    if (hasError) {
      return (
        <Layout>
          <Content className="space-y-4">
            {!import.meta.env.DEV ? (
              <div className="bg-black p-4">
                <p className="!text-white">
                  <span className="!text-red-500">Error:</span>{" "}
                  {this.state.error?.message}
                </p>
                <p className="!text-white">
                  <span className="!text-red-500">Stack:</span>{" "}
                  {this.state.error?.stack}
                </p>
                <p className="!text-white">
                  <span className="!text-red-500">Time:</span>{" "}
                  {this.state.dateTime.toJSON()}
                </p>
                <p className="!text-white">
                  <span className="!text-red-500">Path:</span> {this.state.path}
                </p>
              </div>
            ) : (
              <></>
            )}
          </Content>
        </Layout>
      );
    }

    return children;
  }
}
