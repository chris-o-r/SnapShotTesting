DROP TABLE IF EXISTS snap_shots;

CREATE TABLE snap_shots (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  batch_id UUID NOT NULL,
  name VARCHAR(255) NOT NULL,
  path VARCHAR(255) NOT NULL,
  snap_shot_type VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX snap_shots_batch_id_idx ON snap_shots (batch_id);
