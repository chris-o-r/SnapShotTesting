import React, { createContext, useContext, useState } from "react";
import { MenuItemType } from "antd/es/menu/interface";
import { HistoryOutlined, HomeOutlined, SettingOutlined } from "@ant-design/icons";

export type MenuItem = MenuItemType & { href: string };

export const NavigationContext = createContext<{navigationItems: MenuItem[], setNavigationItems: React.Dispatch<React.SetStateAction<MenuItem[]>>}>(null!);

export const menuItems: MenuItem[] = [
  {
    label: "Home",
    href: "/",
    key: "1",
    icon: <HomeOutlined />,
  },
  {
    label: "Historical",
    key: "2",
    href: "/compare/historical",
    icon: <HistoryOutlined />,
  },
  {
    label: "Admin",
    key: "4",
    href: "/admin",
    icon: <SettingOutlined />,
  },
];

type Props = {
  children: React.ReactElement;
};
export const NavigationProvider = ({ children }: Props) => {
  const [navigationItems, setNavigationItems] = useState<MenuItem[]>(menuItems);

  return (
    <NavigationContext.Provider
      value={{
        navigationItems,
        setNavigationItems,
      }}
    >
      {children}
    </NavigationContext.Provider>
  );
};

export const useNavigation = () => {
  return useContext(NavigationContext);
};
