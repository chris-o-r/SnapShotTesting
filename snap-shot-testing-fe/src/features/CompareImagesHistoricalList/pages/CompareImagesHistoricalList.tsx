import { Card, Layout, Space } from "antd";
import { useFetchSnapShotHistory } from "@/api/snapShotHistory.api";
import Loader from "@/components/Loader";
import { Link } from "react-router-dom";
import { useEffect } from "react";
import { toast } from "react-toastify";
import { Content, Header } from "antd/es/layout/layout";
import Sider from "antd/es/layout/Sider";
import Navigation from "@/components/Navigation";

export default function CompareImagesHistoricalList() {
  const {
    data: history,
    isError: isFetchHistoryError,
    isLoading,
  } = useFetchSnapShotHistory();

  if (isLoading) {
    return <Loader text="Fetching please wait..." />;
  }

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
          <div className="p-4">
            <Space>
              {history?.map((item) => (
                <Link id={item.id} to={`${item.id}`}>
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
            </Space>
          </div>
        </Content>
      </Layout>
    </>
  );
}
