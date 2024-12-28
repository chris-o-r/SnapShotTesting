import { useCleaAllData } from "@/api/admin.api";
import { Button, Modal } from "antd";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";

export const AdminPage = () => {
  const { mutate, isError, isSuccess, isPending } = useCleaAllData();
  const [isModalOpen, setIsModalOpen] = useState(false);

  useEffect(() => {
    if (isSuccess) {
      toast.error("Cleared data succesfully");
      setIsModalOpen(false);
    }

    if (isError) {
      toast.error("Error clearing data");
    }
  }, [isError, isSuccess]);

  const handleConfirm = () => {
    mutate();
  };

  const handleCancel = () => {
    setIsModalOpen(false);
  };

  return (
    <>
      <Modal
        title="Clear all data"
        open={isModalOpen}
        onOk={handleConfirm}
        onCancel={handleCancel}
        loading={isPending  }
      >
        <p>Are you sure you want to clear all data</p>
      </Modal>
      <Button onClick={() => setIsModalOpen(true)}>Delete All Data</Button>
    </>
  );
};
