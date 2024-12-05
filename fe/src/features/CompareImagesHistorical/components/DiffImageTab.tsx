import { SnapShotHistoryResponse } from "@/api/snapShotHistory.api";
import { API_BASE_URL } from "@/constants";
import { Card, Splitter } from "antd";
type Props = {
  diffImages: SnapShotHistoryResponse[number]["diff_image"];
};
export const DiffImageTab = ({ diffImages }: Props) => {
  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">Diff Images</h2>
      {diffImages.map((img) => {
        return (
          <Card
            key={img.diff}
            className="space-y-2"
            title={img.diff.split("/")[img.diff.split("/").length - 1]}
          >
            <Splitter style={{ boxShadow: "0 0 10px rgba(0, 0, 0, 0.1)" }}>
              <Splitter.Panel>
                <img alt="sds" src={`${API_BASE_URL}/${img.new}`} />
              </Splitter.Panel>
              <Splitter.Panel>
                <img alt="sds" src={`${API_BASE_URL}/${img.diff}`} />
              </Splitter.Panel>
              <Splitter.Panel>
                <img alt="sds" src={`${API_BASE_URL}/${img.old}`} />
              </Splitter.Panel>
            </Splitter>
          </Card>
        );
      })}
    </div>
  );
};
