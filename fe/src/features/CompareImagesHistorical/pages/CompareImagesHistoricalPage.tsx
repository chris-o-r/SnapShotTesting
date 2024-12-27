import { useFetchSnapShotHistoryItem } from "@/api/fetchSnapShotHistoryItem.api";
import Loadable from "@/components/Loader";
import { Tabs, TabsProps } from "antd";
import { useParams } from "react-router-dom";
import { DiffImageTab } from "../components/DiffImageTab";
import { ImageTab } from "../components/ImageTab";
import { menuItems, useNavigation } from "@/context/navigationContext";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useEffect } from "react";

export default function CompareImagesHistoricalPage() {
  const { historicalSnapShotId } = useParams();
  const { setNavigationItems } = useNavigation();

 useEffect(() => {
   setNavigationItems([
     {
       label: "Back",
       href: "/compare/historical",
       key: "1",
       icon: <ArrowLeftOutlined />,
     },
   ]);
   return () => {
     setNavigationItems(menuItems);
   };
 }, []);

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
      <div className="">
        <Tabs defaultActiveKey="1" items={items} />
      </div>
    </Loadable>
  );
}
