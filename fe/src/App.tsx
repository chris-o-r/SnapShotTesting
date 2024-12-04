import { Route, Routes } from "react-router-dom";
import { NotFound } from "./features/NotFound/pages/NotFound";
import { StartPage } from "./features/StartPage/pages/StartPage";
import { ErrorBoundary } from "./features/ErrorBoundary";
import { QueryClient as RQQueryClient } from "@tanstack/react-query";
import { PersistQueryClientProvider } from "@tanstack/react-query-persist-client";
import { createSyncStoragePersister } from "@tanstack/query-sync-storage-persister";
import { Layout } from "antd";
import CompareImagesHistoricalPage from "./features/CompareImagesHistorical/pages/CompareImagesHistoricalPage";
import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import CompareImagesHistoricalList from "./features/CompareImagesHistoricalList/pages/CompareImagesHistoricalList";
import Jobs from "./features/Jobs/Pages/Jobs";
import { AdminPage } from "./features/Admin/Page/AdminPage";

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
      <PersistQueryClientProvider
        client={QueryClient}
        persistOptions={{ persister }}
      >
        <ToastContainer />
        <Layout style={{ minHeight: "100vh" }}>
          <Routes>
            <Route path="/">
              <Route index element={<StartPage />} />
              <Route path="/admin" element={<AdminPage />} />
              <Route path="/jobs" element={<Jobs />} />
              <Route
                path="/compare/historical"
                element={<CompareImagesHistoricalList />}
              />
              <Route
                path="/compare/historical/:historicalSnapShotId"
                element={<CompareImagesHistoricalPage />}
              />
              <Route path="*" element={<NotFound />} />
            </Route>
          </Routes>
        </Layout>
      </PersistQueryClientProvider>
    </ErrorBoundary>
  );
}
