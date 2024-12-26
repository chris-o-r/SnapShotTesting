DROP TABLE IF EXISTS snap_shots_batches;


CREATE TABLE snap_shots_batches (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  new_story_book_version VARCHAR(255) NOT NULL,
  old_story_book_version VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS snap_shots;

CREATE TABLE snap_shots (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  batch_id UUID NOT NULL,
  name VARCHAR(255) NOT NULL,
  path VARCHAR(255) NOT NULL,
  width DOUBLE PRECISION NOT NULL,
  height DOUBLE PRECISION NOT NULL,
  snap_shot_type VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);