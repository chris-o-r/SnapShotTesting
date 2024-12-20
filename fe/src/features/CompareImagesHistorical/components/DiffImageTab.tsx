import { SnapShotHistoryResponse } from "@/api/snapShotHistory.api";
import { API_BASE_URL } from "@/constants";
import { Card, Image, Splitter } from "antd";
type Props = {
  diffImages: SnapShotHistoryResponse[number]["diff_image"];
};
export const DiffImageTab = ({ diffImages }: Props) => {
  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">Diff Images</h2>
      {diffImages.map(({diff, new: newImage, old}) => {
        return (
          <Card
            key={diff.path}
            className="space-y-2"
            title={diff.path.split("/")[diff.path.split("/").length - 1]}
          >
            <Splitter style={{ boxShadow: "0 0 10px rgba(0, 0, 0, 0.1)" }}>
              <Splitter.Panel>
                <Image width={newImage.width} alt="sds" src={`${API_BASE_URL}/${newImage.path}`} />
              </Splitter.Panel>
              <Splitter.Panel>
                <Image width={diff.width} alt="sds" src={`${API_BASE_URL}/${diff.path}`} />
              </Splitter.Panel>
              <Splitter.Panel>
                <Image width={newImage.width} alt="sds" src={`${API_BASE_URL}/${old.path}`} />
              </Splitter.Panel>
            </Splitter>
          </Card>
        );
      })}
    </div>
  );
};
