import { useFetchRunningJobs } from "@/api/jobs.api";
import Loadable from "@/components/Loader";
import { formatDateTime } from "@/utils/formatDateTime";
import { Card, Progress } from "antd";
import { Link } from "react-router-dom";

export default function Jobs() {
  const { data: jobs, isLoading } = useFetchRunningJobs();

  return (
    <Loadable isLoading={isLoading}>
      <div className="h-max grid grid-flow-row md:grid-cols-4 grid-cols-2 gap-4">
        {jobs?.map((item) => (
          <Link id={item.id} to={`${item.id}`} key={item.id}>
            <Card key={item.id} title={item.id} hoverable>
              <p>
                <b>ID</b> {item.id}
              </p>
              <p>
                <b>Status:</b> {item.status}
              </p>
              <p>
                <b>Created At:</b> {formatDateTime(new Date(item.created_at))}
              </p>
              <p>
                <b>Updated At:</b> {item.updated_at}
              </p>
              <Progress percent={item.progress * 100} size="small" />
            </Card>
          </Link>
        ))}
      </div>
    </Loadable>
  );
}
