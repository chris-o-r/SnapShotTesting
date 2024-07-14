import { Card, Layout } from "antd";
import { useFetchSnapShotHistory } from "@/api/snapShotHistory.api";
import { Link } from "react-router-dom";
import { useEffect } from "react";
import { toast } from "react-toastify";
import { Content, Header } from "antd/es/layout/layout";
import Sider from "antd/es/layout/Sider";
import Navigation from "@/components/Navigation";
import Loadable from "@/components/Loader";

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
          <Loadable isLoading={isLoading}>
            <div className="grid grid-cols-4 gap-4">
              {history?.map((item) => (
                <Link id={item.id} to={`${item.id}`} key={item.id}>
                  <Card key={item.id} title={item.name} hoverable>
                    <p>
                      <b>URL Old:</b> {item.old_story_book_version}
                    </p>
                    <p>
                      <b>URL New:</b>
                      {item.new_story_book_version}
                    </p>
                    <p>
                      <b>Created At:</b>
                      {item.created_at}
                    </p>
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
