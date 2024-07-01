import { Spin } from "antd";
type Props = {
  text: string;
};
export default function Loader({ text }: Props) {
  return (
    <div className="space-y-4 flex flex-col items-center h-[100vh] w-full justify-center">
      <Spin />
      <p className="text-black">{text}</p>
    </div>
  );
}
