import { useQuery, queryOptions, useIsFetching } from "@tanstack/react-query";
import { API_BASE_URL } from "../constants";
import { QUERY_KEYS } from "./constants";

export type SnapShotHistoryResponse = {
  id: string;
  name: string;
  created_at: string;
  new_story_book_version: string;
  old_story_book_version: string;
  new_images_paths: string[];
  old_images_paths: string[];
  diff_images_paths: {
    created_images_paths: string[];
    deleted_images_paths: string[];
    diff_images_paths: string[];
  };
}[];

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

export async  function fetchSnapShotHistory(): Promise<SnapShotHistoryResponse> {
  const response = await fetch(`${API_BASE_URL}/snap-shot`);
  return response.json();
}
