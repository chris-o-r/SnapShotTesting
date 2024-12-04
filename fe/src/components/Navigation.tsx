import {
  DeliveredProcedureOutlined,
  HistoryOutlined,
  HomeOutlined,
  SettingOutlined,
} from "@ant-design/icons";
import { Menu } from "antd";
import { MenuItemType } from "antd/es/menu/interface";
import { useMemo } from "react";
import { matchPath, useNavigate } from "react-router-dom";

type MenuItem = MenuItemType & { href: string };

export default function Navigation() {
  const navigate = useNavigate();
  const menuItems: MenuItem[] = [
    {
      label: "Home",
      href: "/",
      key: "1",
      icon: <HomeOutlined />,
      onClick: () => navigate("/"),
    },
    {
      label: "Historical",
      key: "2",
      href: "/compare/historical",
      icon: <HistoryOutlined />,
      onClick: () => navigate("/compare/historical"),
    },
    {
      label: "Jobs",
      key: "3",
      href: "/jobs",
      icon: <DeliveredProcedureOutlined />,
      onClick: () => navigate("/jobs"),
    },
    {
      label: "Admin",
      key: "4",
      href: "/admin",
      icon: <SettingOutlined />,
      onClick: () => navigate("/admin"),
    },
  ];
  const pathName = window.location.pathname;

  const currentMenuItemKey = useMemo(() => {
    const currentMenuItem = menuItems.find((item) =>
      matchPath(pathName ?? "", item.href)
    );

    return currentMenuItem?.key;
  }, [pathName]);

  return (
    <Menu
      theme="dark"
      items={menuItems}
      style={{ height: "100%" }}
      defaultSelectedKeys={["1"]}
      defaultOpenKeys={["diff_images_paths"]}
      mode="inline"
      selectedKeys={[currentMenuItemKey?.toString() ?? ""]}
    />
  );
}
