import { useFetchSnapShotHistoryItem } from "@/api/fetchSnapShotHistoryItem.api";
import Loadable from "@/components/Loader";
import { Layout, Tabs, TabsProps } from "antd";
import { Content, Header } from "antd/es/layout/layout";
import { useParams } from "react-router-dom";
import { DiffImageTab } from "../components/DiffImageTab";
import { ImageTab } from "../components/ImageTab";
import Sider from "antd/es/layout/Sider";
import Navigation from "@/components/Navigation";

export default function CompareImagesHistoricalPage() {
  const { historicalSnapShotId } = useParams();

  const { data: historicalSnapShotData, isLoading } =
    useFetchSnapShotHistoryItem(historicalSnapShotId ?? "");

  const items: TabsProps["items"] = [
    {
      key: "1",
      label: "Created",
      children: (
        <ImageTab
          title="Created"
          images={historicalSnapShotData?.created_image_paths ?? []}
        />
      ),
    },
    {
      key: "2",
      label: "Deleted",
      children: (
        <ImageTab
          title="Deleted"
          images={historicalSnapShotData?.deleted_image_paths ?? []}
        />
      ),
    },
    {
      key: "3",
      label: "Diff",
      children: (
        <DiffImageTab diffImages={historicalSnapShotData?.diff_image ?? []} />
      ),
    },
  ];

  return (
    <Loadable isLoading={isLoading}>
      <Header style={{ display: "flex", color: "white", alignItems: "center" }}>
        <h1 className="text-2xl">
          Comparing {historicalSnapShotData?.old_story_book_version} with{" "}
          {historicalSnapShotData?.new_story_book_version}
        </h1>
      </Header>
      <Layout>
        <Sider
          theme="dark"
          style={{
            maxHeight: "800px",
            maxWidth: "256px",
            minHeight: "800px",
            height: "100%",
            overflowY: "auto",
          }}
        >
          <Navigation />
        </Sider>
        <Content className="p-2 w-full h-full">
          <h1 className="text-4xl font-sans font-bold text-black">
            Snap Shot Testing
          </h1>
          <Tabs defaultActiveKey="1" items={items} />
        </Content>
      </Layout>
    </Loadable>
  );
}
