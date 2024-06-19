import {
  MutationFunction,
  queryOptions,
  useIsFetching,
  useMutation,
  useQuery,
} from "@tanstack/react-query";
import axios, { AxiosResponse } from "axios";
import { API_BASE_URL } from "../../../constants";
import { QueryClient } from "../../../App";

export const COMPARE_STORY_BOOK_URLS_QUERY_KEY = "compare_story_book_urls";

export type CompareStoryBookUrlsRequest = {
  new: string;
  old: string;
};
export type CompareStoryBookUrlsResponse = {
  new_images_paths: string[];
  old_images_paths: string[];
  diff_images_paths: {
    diff_images_paths: string[];
    created_images_paths: string[];
    deleted_images_paths: string[];
  };
};

export const useFetchCompareStoryBookUrls = (
  params: CompareStoryBookUrlsRequest,
  isEnabled: boolean = false
) => {
  return useQuery(fetchCompareStoryBookUrlsQueryOptions(params, isEnabled));
};

export const useMutateCompareStoryBookUrls = () => {
  return useMutation({
    mutationFn: compareStoryBookUrls,
    mutationKey: [COMPARE_STORY_BOOK_URLS_QUERY_KEY],
    onSuccess: async (data, params) => {
      await QueryClient.invalidateQueries({
        queryKey: [COMPARE_STORY_BOOK_URLS_QUERY_KEY, params.old, params.new],
      });

      QueryClient.setQueryData(
        [COMPARE_STORY_BOOK_URLS_QUERY_KEY, params.old, params.new],
        data
      );

      return data;
    },
  });
};

const fetchCompareStoryBookUrlsQueryOptions = (
  params: CompareStoryBookUrlsRequest,
  isEnabled = true
) => {
  return queryOptions({
    queryKey: [COMPARE_STORY_BOOK_URLS_QUERY_KEY, params.old, params.new],
    queryFn: () => compareStoryBookUrls(params),

    gcTime: Infinity,
    // staleTime: 1000 * 60 * 60 * 24,
    enabled: isEnabled,
  });
};

export const useIsFetchingCompareStoryBookUrls = () => {
  return useIsFetching({ queryKey: [COMPARE_STORY_BOOK_URLS_QUERY_KEY] });
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
