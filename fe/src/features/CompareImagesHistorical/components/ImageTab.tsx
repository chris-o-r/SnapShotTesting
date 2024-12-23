import { API_BASE_URL } from "@/constants";
import { components } from "@/types/generated";
import { Card, Image } from "antd";

type ImageFromResponse = components["schemas"]["SnapShotBatchImage"];

type Props = {
  images: ImageFromResponse[];
  title: string;
};

export const ImageTab = ({ images, title }: Props) => {
  return (
    <div className="space-y-2">
      <h2 className="text-2xl font-bold">{title}</h2>
      <div className="grid grid-cols-4 grid-flow-row gap-3">
        {images.map(({ path, name }) => {
          return (
            <Card hoverable className="cursor-pointer w-full" title={name}>
              <Image alt="sds" src={`${API_BASE_URL}/${path}`} key={path} />
            </Card>
          );
        })}
      </div>
    </div>
  );
};
