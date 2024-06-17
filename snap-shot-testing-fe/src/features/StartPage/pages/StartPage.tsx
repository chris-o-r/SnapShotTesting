import { Button, Form, Input, Spin } from "antd";
import { useForm, SubmitHandler, Controller } from "react-hook-form";
import {
  CompareStoryBookUrlsRequest,
  useFetchCompareStoryBookUrls,
} from "../api/compareStoryBookUrls";
import {} from "@tanstack/react-query";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
type StartPageForm = {
  oldStoryBookUrl: string;
  newStoryBookUrl: string;
};

export const StartPage = () => {
  const { control, handleSubmit, getValues, formState } =
    useForm<StartPageForm>({
      mode: "onChange",
    });

  const navigate = useNavigate();

  const { isFetching, isSuccess } = useFetchCompareStoryBookUrls(
    getValues("oldStoryBookUrl"),
    getValues("newStoryBookUrl"),
    formState.isSubmitSuccessful
  );

  useEffect(() => {
    if (isSuccess) {
      const request: CompareStoryBookUrlsRequest = {
        new: getValues("newStoryBookUrl"),
        old: getValues("oldStoryBookUrl"),
      };

      const urlParams = new URLSearchParams(request).toString();
      navigate(`/compare?${urlParams.toString()}`);
    }
  }, [isSuccess]);

  const onSubmit: SubmitHandler<StartPageForm> = (data) => {
    console.log(data);
  };

  if (isFetching)
    return (
      <div className="space-y-4 flex flex-col items-center h-[100vh] w-full justify-center">
        <Spin />
        <p>Fetching please wait...</p>
      </div>
    );

  return (
    <div className="space-y-4 flex flex-col items-center h-[100vh] w-full justify-center">
      <h1>Start Page</h1>
      <p>Please enter two story book urls to get started</p>

      <Form
        name="basic"
        labelCol={{ span: 8 }}
        wrapperCol={{ span: 16 }}
        style={{ maxWidth: 600 }}
        initialValues={{ remember: true }}
        onSubmitCapture={handleSubmit(onSubmit)}
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
            render={({ field }) => <Input {...field} />}
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
            render={({ field }) => <Input {...field} />}
          />
        </Form.Item>

        <Button
          type="primary"
          htmlType="submit"
          onClick={handleSubmit(onSubmit)}
        >
          Submit
        </Button>
      </Form>
    </div>
  );
};
