import { useCleaAllData } from "@/api/admin.api";
import Navigation from "@/components/Navigation";
import { Button, Layout } from "antd";
import { Content, Header } from "antd/es/layout/layout";
import Sider from "antd/es/layout/Sider";

export const AdminPage = () => {
  const mutate = useCleaAllData();
  return (
    <>
      <Header style={{ display: "flex", color: "white", alignItems: "center" }}>
        <h1 className="text-2xl">Diff images</h1>
      </Header>
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
          <Navigation />
        </Sider>
        <Content className="p-4 space-y-4">
          <Button onClick={() => mutate.mutate()} >Delete All Data</Button>
        </Content>
      </Layout>
    </>
  );
};
