import { useFetchSnapShotHistory } from "@/api/snapShotHistory.api";
import Loadable from "@/components/Loader";
import { formatDateTime } from "@/utils/formatDateTime";
import { Card } from "antd";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { toast } from "react-toastify";
export default function CompareImagesHistoricalList() {
  const navigate = useNavigate();

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

  return (
    <>
      <Loadable isLoading={isLoading}>
        <div className="h-max grid grid-flow-row md:grid-cols-4 grid-cols-2 gap-4">
          {history?.map((item) => (
            <Card
              key={item.id}
              title={item.name}
              hoverable
              onClick={() => navigate(item.id)}
            >
              <div className="flex flex-col space-y-1">
                <a
                  className="underline"
                  target="_blank"
                  onClick={(e) => e.stopPropagation()}
                  href={item.old_story_book_version}
                >
                  <b>Old URL</b>
                </a>
                <a
                  className="underline"
                  target="_blank"
                  onClick={(e) => e.stopPropagation()}
                  href={item.new_story_book_version}
                >
                  <b>New URL</b>
                </a>
                <span>
                  <b>Created At: </b>
                  {formatDateTime(new Date(item.created_at))}
                </span>
                <span>
                  <b>Total Added:</b>
                  {item.created_image_paths.length}
                </span>
                <span>
                  <b>Total Deleted:</b>
                  {item.deleted_image_paths.length}
                </span>
                <span>
                  <b>Total Changed:</b>
                  {item.deleted_image_paths.length}
                </span>
              </div>
            </Card>
          ))}
        </div>
      </Loadable>
    </>
  );
}
