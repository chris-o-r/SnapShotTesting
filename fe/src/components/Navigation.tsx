import { useNavigation } from "@/context/navigationContext";
import { Menu } from "antd";
import { useMemo } from "react";
import { matchPath, useNavigate } from "react-router-dom";

export default function Navigation() {
  const navigate = useNavigate();
  const { navigationItems} = useNavigation()


  const pathName = window.location.pathname;

  const currentMenuItemKey = useMemo(() => {
    const currentMenuItem = navigationItems.find((item) =>
      matchPath(pathName ?? "", item.href)
    );

    return currentMenuItem?.key;
  }, [pathName]);


  return (
    <Menu
      theme="dark"
      items={navigationItems.map(item =>  {
       return item = {
          ...item,
          onClick: () => navigate(item.href)
        }
      })}
      style={{ height: "100%" }}
      defaultSelectedKeys={["1"]}
      defaultOpenKeys={["diff_images_paths"]}
      mode="inline"
      selectedKeys={[currentMenuItemKey?.toString() ?? ""]}
    />
  );
}
