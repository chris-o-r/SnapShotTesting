import { Spin } from "antd";
type Props = {
  isLoading: boolean;
  children?: React.ReactNode;
  text?: string;
};
export default function Loadable({ text, isLoading, children }: Props) {
  if (isLoading)
    return (
      <div className="space-y-4 flex flex-col items-center h-[100vh] w-full justify-center">
        <Spin />
        {text && <p className="text-black">{text}</p>}
      </div>
    );

  if (children) {
    return children;
  }

  return null;
}
