import { API_BASE_URL } from "@/constants";
import { Card, Modal } from "antd";
import { useState } from "react";
type Props = {
  images: string[];
  title: string;
};
export const ImageTab = ({ images, title }: Props) => {
  const getImageTitle = (url: string) => {
    const urlSplit = url.split("/");
    return urlSplit[urlSplit.length - 1];
  };

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [currentImage, setCurrentImage] = useState<string | null>(null);

  const openDialog = (url: string) => {
    setIsModalOpen(true);
    setCurrentImage(url);
  };

  const closeDialog = () => {
    setCurrentImage(null);
    setIsModalOpen(false);
  };

  return (
    <>
      <div className="space-y-2">
        <h2 className="text-2xl font-bold">{title}</h2>
        <div className="grid grid-cols-4 grid-flow-row gap-3">
          {images.map((img) => {
            return (
              <Card
                className="cursor-pointer"
                title={getImageTitle(img)}
                onClick={() => openDialog(`${API_BASE_URL}/${img}`)}
              >
                <img alt="sds" src={`${API_BASE_URL}/${img}`} key={img} />
              </Card>
            );
          })}
        </div>
      </div>

      <Modal title="Basic Modal" open={isModalOpen}  onOk={() => closeDialog()}>
        <img alt="sds" src={currentImage!} key={currentImage} />
      </Modal>
    </>
  );
};
