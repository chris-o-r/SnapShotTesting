import { useNavigate } from "react-router-dom";
import { SnapShotHistoryResponse } from "../../../api/snapShotHistory.api";
import { ItemType, MenuItemType } from "antd/es/menu/interface";

export function getMenuItemsHistoricalPage(
  data: SnapShotHistoryResponse[number],
  navigate: ReturnType<typeof useNavigate>
): ItemType<MenuItemType>[] {
  return [
    {
      label: "Home",
      key: "1",
      onClick: () => navigate("/"),
    },
    {
      label: "Diff",
      key: "diff_images_paths",
      children: [
        {
          label: "Created",
          key: "diff_images_paths_created",
          children: data.created_image_paths.map((item) => {
            return {
              key: item,
              label: item,
            };
          }),
        },
        {
          label: "Deleted",
          key: "diff_images_paths_deleted",
          children: data.deleted_image_paths.map((item) => {
            return {
              key: item,
              label: item,
            };
          }),
        },
        {
          label: "Changed",
          key: "diff_images_paths_diff",
          children: data.diff_image.map((item) => {
            return {
              key: item.diff,
              label: item.diff,
            };
          }),
        },
      ],
    }
    
  ];
}

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
