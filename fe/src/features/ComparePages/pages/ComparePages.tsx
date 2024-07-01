import { Layout, Menu, MenuProps } from "antd";

import { useState } from "react";
import { useFetchCompareStoryBookUrls } from "../../../api/compareStoryBookUrls.api";
import { API_BASE_URL } from "../../../constants";
import Sider from "antd/es/layout/Sider";
import { Content, Header } from "antd/es/layout/layout";
import { getMenuItems } from "../utils/getMenuItem";
import Loader from "../../../components/Loader";

export const ComparePages = () => {
  const [current, setCurrent] = useState<string | null>(null);
  const [currentImage, setCurrentImage] = useState<string | null>(null);

  const params = new URLSearchParams(window.location.href.split("?")[1]);
  const oldUrl = params.get("old");
  const newUrl = params.get("new");

  const { ...rest } = useFetchCompareStoryBookUrls({
    old: oldUrl!,
    new: newUrl!,
  });

  const onClick: MenuProps["onClick"] = (e) => {
    if (e.keyPath.length === 1) {
      setCurrent(e.key);
    } else {
      setCurrentImage(e.key);
    }
  };

  if (rest.isPending) return <Loader text="Fetching please wait..." />;

  return (
    <>
      <Header style={{ display: "flex", color: "white", alignItems: "center" }}>
        <h1 className="text-2xl">
          Comparing {oldUrl} with {newUrl}
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
            <Menu
              theme="dark"
              items={getMenuItems(rest.data!)}
              style={{ height: "100%" }}
              defaultSelectedKeys={["1"]}
              defaultOpenKeys={["diff_images_paths"]}
              mode="inline"
              selectedKeys={[current ?? "", currentImage ?? ""]}
              onClick={onClick}
            />
          </Sider>
          <Content>
            {currentImage && (
              <img alt="sds" src={`${API_BASE_URL}/${currentImage}`} />
            )}
          </Content>
        </Layout>
      </Content>
    </>
  );
};
