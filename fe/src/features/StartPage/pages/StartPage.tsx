import { Button, Form, Input, Layout } from "antd";
import { useForm, SubmitHandler, Controller } from "react-hook-form";
import { useMutateCompareStoryBookUrls } from "@/api/compareStoryBookUrls.api";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Content, Header } from "antd/es/layout/layout";
import { QueryClient } from "@/App";
import Sider from "antd/es/layout/Sider";
import { QUERY_KEYS } from "@/api/constants";
import { toast } from "react-toastify";
import Navigation from "@/components/Navigation";
import Loadable from "@/components/Loader";

type StartPageForm = {
  oldStoryBookUrl: string;
  newStoryBookUrl: string;
};

export const StartPage = () => {
  const { control, handleSubmit, getValues } = useForm<StartPageForm>({
    mode: "onChange",
  });
  const navigate = useNavigate();

  const mutate = useMutateCompareStoryBookUrls();

  useEffect(() => {
    QueryClient.cancelQueries({
      queryKey: [QUERY_KEYS.COMPARE_STORY_BOOK_URLS_QUERY_KEY],
      exact: false,
    });
  });

  useEffect(() => {
    if (mutate.isError) toast.error("Error while fetching data");
  }, [getValues, mutate.isSuccess, mutate.isError, navigate]);

  const onSubmit: SubmitHandler<StartPageForm> = (data) => {
    mutate.mutate({
      new: data.newStoryBookUrl,
      old: data.oldStoryBookUrl,
    });
  };

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
        <Loadable isLoading={mutate.status === "pending"}>
          <Content className="space-y-4 flex flex-col items-center  justify-center">
            <h1 className="text-4xl font-sans font-bold text-black">
              Snap Shot Testing
            </h1>
            <p className="text-black">
              Please enter two story book urls to get started
            </p>

            <Form
              name="basic"
              className="flex flex-col items-center  justify-center"
              initialValues={{ remember: true }}
              onSubmitCapture={(e) => {
                e.preventDefault();
                handleSubmit(onSubmit);
              }}
              autoComplete="on"
            >
              <Form.Item<StartPageForm>
                label="Old Story Book URL"
                name="oldStoryBookUrl"
                rules={[
                  { required: true, message: "Please input your username!" },
                ]}
              >
                <Controller
                  name="oldStoryBookUrl"
                  defaultValue=""
                  control={control}
                  render={({ field }) => (
                    <Input className="min-w-48" {...field} />
                  )}
                />
              </Form.Item>
              <Form.Item<StartPageForm>
                label="New Story Book URL"
                name="newStoryBookUrl"
                rules={[
                  { required: true, message: "Please input your username!" },
                ]}
              >
                <Controller
                  name="newStoryBookUrl"
                  defaultValue=""
                  control={control}
                  render={({ field }) => (
                    <Input className="min-w-48" {...field} />
                  )}
                />
              </Form.Item>
              <Form.Item>
                <Button
                  type="primary"
                  htmlType="submit"
                  onClick={handleSubmit(onSubmit)}
                >
                  Submit
                </Button>
              </Form.Item>
            </Form>
          </Content>
        </Loadable>
      </Layout>
    </>
  );
};
