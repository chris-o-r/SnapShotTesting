import { Route, Routes as TheOtherRoutes } from "react-router-dom";
import { AdminPage } from "./features/Admin/Page/AdminPage";
import CompareImagesHistoricalPage from "./features/CompareImagesHistorical/pages/CompareImagesHistoricalPage";
import CompareImagesHistoricalList from "./features/CompareImagesHistoricalList/pages/CompareImagesHistoricalList";
import { NotFound } from "./features/NotFound/pages/NotFound";
import { StartPage } from "./features/StartPage/pages/StartPage";

export const Routes = () => {
  return (
    <TheOtherRoutes>
      <Route path="/">
        <Route index element={<StartPage />} />
        <Route path="/admin" element={<AdminPage />} />
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
    </TheOtherRoutes>
  );
};
