--this WILL cause inconsistencies, use Count instead
-- https://www.w3schools.com/sql/sql_count.asp
-- SELECT COUNT(*) FROM level_stars WHERE 
--
-- ALTER TABLE levels
-- ADD COLUMN stars INTEGER DEFAULT 0;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name='levels'
          AND column_name='total_crystals'
    ) THEN
        ALTER TABLE levels ADD COLUMN total_crystals INTEGER DEFAULT 0;
    END IF;
END
$$;
