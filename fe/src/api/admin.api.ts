import { API_BASE_URL } from "@/constants";
import { useMutation } from "@tanstack/react-query";
import { QUERY_KEYS } from "./constants";
import { QueryClient } from "../App";

export const useCleaAllData = () => {
  return useMutation({
    mutationFn: clearAllDataRequest,
    mutationKey: [QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY],
    onSuccess: async (data) => {
      QueryClient.invalidateQueries({
        queryKey: Object.values(QUERY_KEYS),
      });
      return data;
    },
  });
};

export async function clearAllDataRequest(): Promise<void> {
  const response = await fetch(`${API_BASE_URL}/admin/clean-up`);
  return response.json();
}
