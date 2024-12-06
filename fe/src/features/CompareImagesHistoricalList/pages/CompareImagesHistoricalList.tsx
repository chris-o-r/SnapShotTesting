import { useFetchSnapShotHistory } from "@/api/snapShotHistory.api";
import Loadable from "@/components/Loader";
import { formatDateTime } from "@/utils/formatDateTime";
import { Card } from "antd";
import { useEffect } from "react";
import { Link } from "react-router-dom";
import { toast } from "react-toastify";
export default function CompareImagesHistoricalList() {
  const {
    data: history,
    isError: isFetchHistoryError,
    isLoading,
  } = useFetchSnapShotHistory();

  useEffect(() => {
    if (isFetchHistoryError) {
      toast.error("Error while fetching data");
    }
  }, [isFetchHistoryError]);

  const formatUrl = (str: string) => {
    return `https://${str}`;
  };

  return (
    <>
      <Loadable isLoading={isLoading}>
        <div className="h-max grid grid-flow-row md:grid-cols-4 grid-cols-2 gap-4">
          {history?.map((item) => (
            <Link id={item.id} to={`${item.id}`} key={item.id}>
              <Card key={item.id} title={item.name} hoverable>
                <div className="flex flex-col space-y-1">
                  <a
                    className="underline"
                    href={formatUrl(item.old_story_book_version)}
                  >
                    <b>Old URL</b>
                  </a>
                  <a
                    className="underline"
                    href={formatUrl(item.new_story_book_version)}
                  >
                    <b>New URL</b>
                  </a>
                  <span>
                    <b>Created At: </b>
                    {formatDateTime(new Date(item.created_at))}
                  </span>
                </div>
              </Card>
            </Link>
          ))}
        </div>
      </Loadable>
    </>
  );
}
