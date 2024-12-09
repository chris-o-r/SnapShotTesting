import Navigation from "@/components/Navigation"
import { Content, Header } from "antd/es/layout/layout"
import Sider from "antd/es/layout/Sider"
import { Routes } from "../Routes"
import { Layout as TheOtherLayout } from "antd";
import { usePageTitle } from "@/context/pageTitleContext";


export const Layout = () => {
    const {title} = usePageTitle()
    return (
        <TheOtherLayout hasSider>
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
        <TheOtherLayout>
        <Header style={{ padding: 0 }} className="flex items-center text-white justify-center text-2xl font-bold"><h1>{title}</h1></Header>

          <Content style={{ overflow: "initial", maxHeight: "100dvh" }}>
            <div className="overflow-y-auto h-full p-4">
              <Routes />
            </div>
          </Content>
        </TheOtherLayout>
      </TheOtherLayout>
    )
}