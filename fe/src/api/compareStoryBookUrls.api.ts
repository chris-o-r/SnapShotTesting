import {
  MutationFunction,
  queryOptions,
  useIsFetching,
  useMutation,
  useQuery,
} from "@tanstack/react-query";
import axios, { AxiosResponse } from "axios";
import { API_BASE_URL } from "../constants";
import { QueryClient } from "../App";
import { QUERY_KEYS } from "./constants";
import { Job } from "@/types/job";

export type CompareStoryBookUrlsRequest = {
  new: string;
  old: string;
};

export type CompareStoryBookUrlsResponse = Job;

export const useFetchCompareStoryBookUrls = (
  params: CompareStoryBookUrlsRequest,
  isEnabled: boolean = false
) => {
  return useQuery(fetchCompareStoryBookUrlsQueryOptions(params, isEnabled));
};

export const useMutateCompareStoryBookUrls = () => {
  return useMutation({
    mutationFn: compareStoryBookUrls,
    mutationKey: [QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY],
    onSuccess: async (data, params) => {
      await QueryClient.invalidateQueries({
        queryKey: [
          QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY,
          params.old,
          params.new,
        ],
      });

      QueryClient.setQueryData(
        [QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY, params.old, params.new],
        data
      );

      QueryClient.invalidateQueries({
        queryKey: [QUERY_KEYS.SNAPSHOT_HISTORY_QUERY_KEY],
      });
      return data;
    },
  });
};

const fetchCompareStoryBookUrlsQueryOptions = (
  params: CompareStoryBookUrlsRequest,
  isEnabled = true
) => {
  return queryOptions({
    queryKey: [
      QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY,
      params.old,
      params.new,
    ],
    queryFn: () => compareStoryBookUrls(params),

    gcTime: Infinity,
    // staleTime: 1000 * 60 * 60 * 24,
    enabled: isEnabled,
  });
};

export const useIsFetchingCompareStoryBookUrls = () => {
  return useIsFetching({
    queryKey: [QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY],
  });
};

const compareStoryBookUrls: MutationFunction<
  CompareStoryBookUrlsResponse,
  CompareStoryBookUrlsRequest
> = async (params: CompareStoryBookUrlsRequest) => {
  return axios
    .post<
      CompareStoryBookUrlsRequest,
      AxiosResponse<CompareStoryBookUrlsResponse>
    >(`${API_BASE_URL}/snap-shot`, params)
    .then((res) => res.data);
};
