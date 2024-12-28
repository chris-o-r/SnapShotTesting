import { useQuery, queryOptions, useIsFetching } from "@tanstack/react-query";
import { API_BASE_URL } from "../constants";
import { QUERY_KEYS } from "./constants";
import { operations } from "@/types/generated";

export type SnapShotHistoryResponse =
  operations["handle_get_snapshot_history"]["responses"]["200"]["content"]["application/json"]

export const useFetchSnapShotHistory = () => {
  return useQuery(fetchSnapShotHistoryQueryOptions());
};

const fetchSnapShotHistoryQueryOptions = () => {
  return queryOptions({
    queryKey: [QUERY_KEYS.SNAPSHOT_HISTORY_QUERY_KEY],
    queryFn: () => fetchSnapShotHistory(),
    gcTime: Infinity,
    enabled: true,
  });
};

export const useIsFetchingSnapShotHistory = () => {
  return useIsFetching({ queryKey: [QUERY_KEYS.SNAPSHOT_HISTORY_QUERY_KEY] });
};

export async function fetchSnapShotHistory(): Promise<SnapShotHistoryResponse> {
  const response = await fetch(`${API_BASE_URL}/snap-shots`);
  return response.json();
}
