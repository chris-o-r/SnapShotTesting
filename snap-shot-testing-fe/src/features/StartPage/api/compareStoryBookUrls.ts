import { queryOptions, useIsFetching, useQuery } from "@tanstack/react-query";
import axios from "axios";
import { API_BASE_URL } from "../../../constants";

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
  oldStoryBookUrl: string,
  newStoryBookUrl: string,
  isEnabled: boolean
) => {
  return useQuery(
    fetchCompareStoryBookUrlsQueryOptions(
      oldStoryBookUrl,
      newStoryBookUrl,
      isEnabled
    )
  );
};

const fetchCompareStoryBookUrlsQueryOptions = (
  oldStoryBookUrl: string,
  newStoryBookUrl: string,
  isEnabled: boolean
) => {
  return queryOptions({
    queryKey: [
      COMPARE_STORY_BOOK_URLS_QUERY_KEY,
      oldStoryBookUrl,
      newStoryBookUrl,
    ],
    queryFn: () => compareStoryBookUrls(oldStoryBookUrl, newStoryBookUrl),
    staleTime: 1000 * 60 * 60 * 24,
    enabled: isEnabled,
  });
};

export const useIsFetchingCompareStoryBookUrls = () => {
  return useIsFetching({ queryKey: [COMPARE_STORY_BOOK_URLS_QUERY_KEY] });
};

const compareStoryBookUrls = async (
  oldStoryBookUrl: string,
  newStoryBookUrl: string
) => {
  const request: CompareStoryBookUrlsRequest = {
    new: newStoryBookUrl,
    old: oldStoryBookUrl,
  };

  const urlParams = new URLSearchParams(request).toString();

  return axios.get<CompareStoryBookUrlsResponse>(
    `${API_BASE_URL}/snap-shot?${urlParams}`
  );
};
