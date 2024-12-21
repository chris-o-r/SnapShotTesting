import { API_BASE_URL } from "@/constants";
import { Card, Image } from "antd";
type Props = {
  images: {
    height: number;
    width: number;
    path: string;
  }[];
  title: string;
};
export const ImageTab = ({ images, title }: Props) => {
  const getImageTitle = (url: string) => {
    const urlSplit = url.split("/");
    return urlSplit[urlSplit.length - 1];
  };



  return (
    <>
      <div className="space-y-2">
        <h2 className="text-2xl font-bold">{title}</h2>
        <div className="grid grid-cols-4 grid-flow-row gap-3">
          {images.map(({ path }) => {
            return (
              <Card
                hoverable
                className="cursor-pointer w-full"
                title={getImageTitle(path)}
              >
                <Image
                  alt="sds"
                  src={`${API_BASE_URL}/${path}`}
                  key={path}
                />
              </Card>
            );
          })}
        </div>
      </div>
    </>
  );
};
