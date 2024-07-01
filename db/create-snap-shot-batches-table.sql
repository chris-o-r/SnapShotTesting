DROP TABLE IF EXISTS snap_shots_batches;


CREATE TABLE snap_shots_batches (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(100) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  new_story_book_version VARCHAR(100) NOT NULL,
  old_story_book_version VARCHAR(100) NOT NULL
);


