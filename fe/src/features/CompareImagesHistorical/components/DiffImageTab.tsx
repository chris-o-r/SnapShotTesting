import { SnapShotHistoryResponse } from "@/api/snapShotHistory.api";
import { API_BASE_URL } from "@/constants";
import { Button, Card, Image, Splitter } from "antd";
import { useState } from "react";
type Props = {
  diffImages: SnapShotHistoryResponse[number]["diff_image"];
};
export const DiffImageTab = ({ diffImages }: Props) => {
  const [isVisible, setIsLcsVisible] = useState<string | null>(null);

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">Diff Images</h2>
      {diffImages.map(({ color_diff, lcs_diff, new: newImage, old }) => {
        const lcs_path = `${API_BASE_URL}/${lcs_diff.path}`;

        return (
          <>
            <Card
              key={color_diff.path}
              className="space-y-2"
              title={color_diff.name}
              extra={
                <Button
                  onClick={() => setIsLcsVisible(lcs_diff.path)}
                  type="link"
                >
                  LCS Diff
                </Button>
              }
            >
              <Splitter style={{ boxShadow: "0 0 10px rgba(0, 0, 0, 0.1)" }}>
                <Splitter.Panel>
                  <Image
                    alt={`new-${newImage.path}`}
                    src={`${API_BASE_URL}/${newImage.path}`}
                  />
                </Splitter.Panel>
                <Splitter.Panel>
                  <Image
                    alt={`diff-${color_diff.path}`}
                    src={`${API_BASE_URL}/${color_diff.path}`}
                  />
                </Splitter.Panel>
                <Splitter.Panel>
                  <Image
                    alt={`new-${old.path}`}
                    src={`${API_BASE_URL}/${old.path}`}
                  />
                </Splitter.Panel>
              </Splitter>
            </Card>

            <Image
              width={lcs_diff.width}
              style={{ display: "none" }}
              src={lcs_path}
              preview={{
                visible: isVisible === lcs_diff.path,
                src: lcs_path,
                onVisibleChange: (value) => {
                  setIsLcsVisible((update) =>
                    value === false ? null : update
                  );
                },
              }}
            />
          </>
        );
      })}
    </div>
  );
};
