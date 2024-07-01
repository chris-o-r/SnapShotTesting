import { Layout, Menu, MenuProps } from "antd";
import { useFetchSnapShotHistory } from "@/api/snapShotHistory.api";
import { Content, Header } from "antd/es/layout/layout";
import Sider from "antd/es/layout/Sider";
import { useParams } from "react-router-dom";
import { useState } from "react";
import { API_BASE_URL } from "@/constants";
import { getMenuItemsHistoricalPage } from "../utils/getMenuItemsHistoricalPage";
import Loader from "@/components/Loader";

export default function CompareImagesHistoricalPage() {
  const [selectedImage, setSelectedImage] = useState<string | null>(null);
  const [current, setCurrent] = useState<string | null>(null);

  const { data: historicalSnapShotData, isLoading } = useFetchSnapShotHistory();
  let { historicalSnapShotId } = useParams();
  console.log(historicalSnapShotData);

  const currentHistoricalSnapShot = historicalSnapShotData?.find(
    (item) => item.id === historicalSnapShotId
  );

  const onClick: MenuProps["onClick"] = (e) => {
    if (e.keyPath.length === 1) {
      setCurrent(e.key);
    } else {
      setSelectedImage(e.key);
    }
  };
  if (isLoading) {
    return <Loader text="Fetching please wait..." />;
  }

  return (
    <>
      <Header style={{ display: "flex", color: "white", alignItems: "center" }}>
        <h1 className="text-2xl">
          Comparing {currentHistoricalSnapShot?.old_story_book_version} with{" "}
          {currentHistoricalSnapShot?.new_story_book_version}
        </h1>
      </Header>
      <Content>
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
            {currentHistoricalSnapShot && (
              <Menu
                theme="dark"
                items={getMenuItemsHistoricalPage(currentHistoricalSnapShot)}
                style={{ height: "100%" }}
                defaultSelectedKeys={["1"]}
                defaultOpenKeys={["diff_images_paths"]}
                mode="inline"
                selectedKeys={[current ?? "", selectedImage ?? ""]}
                onClick={onClick}
              />
            )}
          </Sider>
          <Content>
            {selectedImage && (
              <img alt="sds" src={`${API_BASE_URL}/${selectedImage}`} />
            )}
          </Content>
        </Layout>
      </Content>
    </>
  );
}
