import { API_BASE_URL } from "@/constants";
import { Card } from "antd";
type Props = {
  images: string[];
  title: string;
};
export const ImageTab = ({ images, title }: Props) => {
  const getImageTitle = (url: string) => {
    const urlSplit = url.split("/");
    return urlSplit[urlSplit.length - 1];
  };

  return (
    <div className="space-y-2">
      <h2 className="text-2xl font-bold">{title}</h2>
      <div className="grid grid-cols-4 grid-flow-row gap-3">
        {images.map((img) => {
          return (
            <Card title={getImageTitle(img)}>
              <img alt="sds" src={`${API_BASE_URL}/${img}`} key={img} />
            </Card>
          );
        })}
      </div>
    </div>
  );
};
