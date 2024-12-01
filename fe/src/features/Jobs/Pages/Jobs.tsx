import { useFetchRunningJobs } from "@/api/jobs.api";
import Loadable from "@/components/Loader";
import Navigation from "@/components/Navigation";
import { Layout, Card, Progress } from "antd";
import Sider from "antd/es/layout/Sider";
import { Header, Content } from "antd/es/layout/layout";
import { Link } from "react-router-dom";

export default function Jobs() {
  const { data: jobs, isLoading } = useFetchRunningJobs();

  return (
    <>
      <Header style={{ display: "flex", color: "white", alignItems: "center" }}>
        <h1 className="text-2xl">Diff images Historical</h1>
      </Header>
      <Layout>
        <Sider>
          <Navigation />
        </Sider>
        <Content>
          <h2>Running Jobs</h2>
          <Loadable isLoading={isLoading}>
            <div className="p-4 h-max grid grid-flow-row grid-cols-4 gap-4">
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
                      <b>Created At:</b> {item.created_at}
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
        </Content>
      </Layout>
    </>
  );
}
