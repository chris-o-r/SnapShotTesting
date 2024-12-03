import { Card, Layout } from "antd";
import { useFetchSnapShotHistory } from "@/api/snapShotHistory.api";
import { Link } from "react-router-dom";
import { useEffect } from "react";
import { toast } from "react-toastify";
import { Content, Header } from "antd/es/layout/layout";
import Sider from "antd/es/layout/Sider";
import Navigation from "@/components/Navigation";
import Loadable from "@/components/Loader";
import {formatDateTime} from '@/utils/formatDateTime'
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
    return `https://${str}`
  }

  return (
    <>
      <Header style={{ display: "flex", color: "white", alignItems: "center" }}>
        <h1 className="text-2xl">Diff images Historical</h1>
      </Header>
      <Layout>
        <Sider>
          <Navigation />
        </Sider>
        <Content className="p-4 space-y-4">
          <h2 className="text-2xl font-semibold">Historical Jobs</h2>

          <Loadable isLoading={isLoading}>
            <div className="h-max grid grid-flow-row md:grid-cols-4 grid-cols-2 gap-4">
              {history?.map((item) => (
                <Link id={item.id} to={`${item.id}`} key={item.id}>
                  <Card key={item.id} title={item.name} hoverable>
                    <div className="flex flex-col space-y-1">
                      <a className="underline" href={formatUrl(item.old_story_book_version)}><b>Old URL</b></a>
                      <a className="underline" href={formatUrl(item.new_story_book_version)}><b>New URL</b></a>
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
        </Content>
      </Layout>
    </>
  );
}
