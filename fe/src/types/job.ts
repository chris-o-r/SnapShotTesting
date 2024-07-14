export type Job = {
  created_at: string;
  id: string;
  snap_shot_batch_id: string | null;
  status: string;
  updated_at: string;
  progress: number;
};
