import { useCleaAllData } from "@/api/admin.api";
import { Button } from "antd";

export const AdminPage = () => {
  const mutate = useCleaAllData();
  return <Button onClick={() => mutate.mutate()}>Delete All Data</Button>;
};
