import { createSyncStoragePersister } from "@tanstack/query-sync-storage-persister";
import { QueryClient as RQQueryClient } from "@tanstack/react-query";
import { PersistQueryClientProvider } from "@tanstack/react-query-persist-client";
import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import { PageTitleProvider } from "./context/pageTitleContext";
import { ErrorBoundary } from "./features/ErrorBoundary";
import { Layout } from "./Layouts/Layout";
import { NavigationProvider } from "./context/navigationContext";

export const QueryClient = new RQQueryClient({
  defaultOptions: {
    queries: {},
  },
});
const persister = createSyncStoragePersister({
  storage: window.sessionStorage,
});

export default function App() {

  return (
    <ErrorBoundary>
      <PageTitleProvider>
        <NavigationProvider>
        <PersistQueryClientProvider
          client={QueryClient}
          persistOptions={{ persister }}
        >
          <ToastContainer />

          <Layout />
        </PersistQueryClientProvider>
        </NavigationProvider>
      </PageTitleProvider>
    </ErrorBoundary>
  );
}
