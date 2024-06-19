import { Route, Routes } from "react-router-dom";
import { NotFound } from "./features/NotFound/pages/NotFound";
import { StartPage } from "./features/StartPage/pages/StartPage";
import { ErrorBoundary } from "./features/ErrorBoundary";
import { QueryClient as RQQueryClient } from "@tanstack/react-query";
import { PersistQueryClientProvider } from "@tanstack/react-query-persist-client";
import { createSyncStoragePersister } from "@tanstack/query-sync-storage-persister";
import { ComparePages } from "./features/ComparePages/pages/ComparePages";
import { Layout } from "antd";

export const QueryClient = new RQQueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 60 * 24,
    },
  },
});
const persister = createSyncStoragePersister({
  storage: window.localStorage,
});

export default function App() {
  return (
    <ErrorBoundary>
      <PersistQueryClientProvider
        client={QueryClient}
        persistOptions={{ persister }}
      >
        <Layout style={{ minHeight: "100vh" }}>
          <Routes>
            <Route path="/">
              <Route index element={<StartPage />} />
              <Route path="/compare" element={<ComparePages />} />

              {/* Using path="*"" means "match anything", so this route
            acts like a catch-all for URLs that we don't have explicit
            routes for. */}
              <Route path="*" element={<NotFound />} />
            </Route>
          </Routes>
        </Layout>
      </PersistQueryClientProvider>
    </ErrorBoundary>
  );
}
