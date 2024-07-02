import { API_BASE_URL } from "@/constants";
import { useQuery, queryOptions } from "@tanstack/react-query";
import { QUERY_KEYS } from "./constants";
import { SnapShotHistoryResponse } from "./snapShotHistory.api";

export const useFetchSnapShotHistoryItem = (id: string) => {
  return useQuery(fetchSnapShotHistoryItemQueryOptions(id));
};

const fetchSnapShotHistoryItemQueryOptions = (id: string) => {
  return queryOptions({
    queryKey: [QUERY_KEYS.SNAPSHOT_HISTORY_ITEM_QUERY_KEY, id],
    queryFn: () => fetchSnapShotHistoryItem(id),
    gcTime: Infinity,
    enabled: true,
  });
};

export async function fetchSnapShotHistoryItem(
  id: string
): Promise<SnapShotHistoryResponse[number]> {
  const response = await fetch(`${API_BASE_URL}/snap-shot/${id}`);
  return response.json();
}
