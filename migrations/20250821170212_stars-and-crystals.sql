ALTER TABLE levels
ADD COLUMN stars INTEGER DEFAULT 0;

ALTER TABLE levels
ADD COLUMN total_crystals INTEGER DEFAULT 0;

-- Create join table to track which users starred which levels
CREATE TABLE IF NOT EXISTS level_stars (
    user_id UUID NOT NULL,
    level_id UUID NOT NULL,
    starred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, level_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (level_id) REFERENCES levels(id) ON DELETE CASCADE
);
