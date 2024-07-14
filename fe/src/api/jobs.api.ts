import { useQuery, queryOptions } from "@tanstack/react-query";
import { API_BASE_URL } from "../constants";
import { QUERY_KEYS } from "./constants";
import { Job } from "@/types/job";

export const useFetchRunningJobs = () => {
  return useQuery(fetchRunningJobsQueryOptions());
};

const fetchRunningJobsQueryOptions = () => {
  return queryOptions({
    queryKey: [QUERY_KEYS.JOBS_QUERY_KEY],
    queryFn: () => fetchAllRunningJobs(),
    staleTime: 1000 * 10,
    enabled: true,
  });
};

export async function fetchAllRunningJobs(): Promise<Job[]> {
  const response = await fetch(`${API_BASE_URL}/jobs`);
  return response.json();
}
