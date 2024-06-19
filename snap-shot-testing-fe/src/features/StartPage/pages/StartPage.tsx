import { Button, Form, Input, Spin } from "antd";
import { useForm, SubmitHandler, Controller } from "react-hook-form";
import {
  COMPARE_STORY_BOOK_URLS_QUERY_KEY,
  CompareStoryBookUrlsRequest,
  useMutateCompareStoryBookUrls,
} from "../api/compareStoryBookUrls";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Content } from "antd/es/layout/layout";
import { QueryClient } from "../../../App";
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
      queryKey: [COMPARE_STORY_BOOK_URLS_QUERY_KEY],
      exact: false,
    });
  });

  useEffect(() => {
    if (mutate.isSuccess) {
      const request: CompareStoryBookUrlsRequest = {
        new: getValues("newStoryBookUrl"),
        old: getValues("oldStoryBookUrl"),
      };

      const urlParams = new URLSearchParams(request).toString();
      navigate(`/compare?${urlParams.toString()}`);
    }
  }, [getValues, mutate.isSuccess, navigate]);

  const onSubmit: SubmitHandler<StartPageForm> = (data) => {
    mutate.mutate({
      new: data.newStoryBookUrl,
      old: data.oldStoryBookUrl,
    });
  };

  if (mutate.status === "pending")
    return (
      <div className="space-y-4 flex flex-col items-center h-[100vh] w-full justify-center">
        <Spin />
        <p className="text-black">Fetching please wait...</p>
      </div>
    );

  return (
    <Content className="space-y-4 flex flex-col items-center  justify-center">
      <h1 className="text-4xl font-sans font-bold text-black">
        Snap Shot Testing
      </h1>
      <p className="text-black">
        Please enter two story book urls to get started
      </p>

      <Form
        name="basic"
        // labelCol={{ span: 16 }}
        // wrapperCol={{ span: 32 }}
        // wrapperCol={{ span: 32 }}
        // style={{ maxWidth: 120 }}
        className="flex flex-col items-center  justify-center"
        initialValues={{ remember: true }}
        onSubmitCapture={(e) => {
          e.preventDefault();
          handleSubmit(onSubmit);
        }}
        autoComplete="off"
      >
        <Form.Item<StartPageForm>
          label="Old Story Book URL"
          name="oldStoryBookUrl"
          rules={[{ required: true, message: "Please input your username!" }]}
        >
          <Controller
            name="oldStoryBookUrl"
            defaultValue=""
            control={control}
            render={({ field }) => <Input className="min-w-48" {...field} />}
          />
        </Form.Item>
        <Form.Item<StartPageForm>
          label="New Story Book URL"
          name="newStoryBookUrl"
          rules={[{ required: true, message: "Please input your username!" }]}
        >
          <Controller
            name="newStoryBookUrl"
            defaultValue=""
            control={control}
            render={({ field }) => <Input className="min-w-48" {...field} />}
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
  );
};
