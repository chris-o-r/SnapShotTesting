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
          children: data.diff_images_paths.created_images_paths.map((item) => {
            return {
              key: item,
              label: item,
            };
          }),
        },
        {
          label: "Deleted",
          key: "diff_images_paths_deleted",
          children: data.diff_images_paths.deleted_images_paths.map((item) => {
            return {
              key: item,
              label: item,
            };
          }),
        },
        {
          label: "Changed",
          key: "diff_images_paths_diff",
          children: data.diff_images_paths.diff_images_paths.map((item) => {
            return {
              key: item,
              label: item,
            };
          }),
        },
      ],
    },

    {
      label: getTitle("new_images_paths"),
      key: "new_images_paths",
      children: data.new_images_paths.map((item) => {
        return {
          key: item,
          label: item,
        };
      }),
    },
    {
      label: getTitle("old_images_paths"),
      key: "old_images_paths",
      children: data.old_images_paths.map((item) => {
        return {
          key: item,
          label: item,
        };
      }),
    },
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
