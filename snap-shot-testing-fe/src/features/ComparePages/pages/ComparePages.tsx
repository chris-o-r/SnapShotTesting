import { Layout, Menu, MenuProps, Spin } from "antd";

import { useState } from "react";
import {
  CompareStoryBookUrlsResponse,
  useFetchCompareStoryBookUrls,
} from "../../StartPage/api/compareStoryBookUrls";
import { API_BASE_URL } from "../../../constants";
import Sider from "antd/es/layout/Sider";
import { Content, Header } from "antd/es/layout/layout";
import getObjectEntries from "../../../utils/getObjectEntries";

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
    console.log("click ", e);
    if (e.keyPath.length === 1) {
      setCurrent(e.key);
    } else {
      setCurrentImage(e.key);
    }
  };

  const getMenuItems = (
    data: CompareStoryBookUrlsResponse
  ): MenuProps["items"] => {
    const getTitle = (key: string) => {
      if (key === "new_images_paths") {
        return "New Images";
      } else if (key === "old_images_paths") {
        return "Old Images";
      } else if (key === "diff_images_paths") {
        return "Changed";
      } else if (key === "created_images_paths") {
        return "Created";
      } else if (key === "deleted_images_paths") {
        return "Deleted";
      }
    };

    const getIsItemDiffImageObject = (
      item: unknown
    ): item is CompareStoryBookUrlsResponse["diff_images_paths"] => {
      if (item === null || item === undefined) return false;

      if (Array.isArray(item)) return false;

      if (typeof item !== "object") return false;

      if (
        "diff_images_paths" in item &&
        "created_images_paths" in item &&
        "deleted_images_paths" in item
      ) {
        return true;
      }

      return false;
    };

    return getObjectEntries(data).map(([key, value]) => {
      if (getIsItemDiffImageObject(value)) {
        return {
          key,
          label: "Diff Images",
          children: getObjectEntries(value).map(([key, value]) => {
            return {
              key: `${key}_diff`,
              label: getTitle(key),
              children: value.map((item) => {
                return {
                  key: item,
                  label: item,
                };
              }),
            };
          }),
        };
      } else {
        return {
          key,
          label: getTitle(key),
          children: value.map((item) => {
            return {
              key: item,
              label: item,
            };
          }),
        };
      }
    });
  };

  if (rest.isPending)
    return (
      <div className="space-y-4 flex flex-col items-center h-[100vh] w-full justify-center">
        <Spin />
        <p className="text-black">Fetching please wait...</p>
      </div>
    );

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
              selectedKeys={[current ?? ""]}
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
