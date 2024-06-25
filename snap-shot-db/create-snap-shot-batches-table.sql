DROP TABLE IF EXISTS snap_shots_batches;


CREATE TABLE snap_shots_batches (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


