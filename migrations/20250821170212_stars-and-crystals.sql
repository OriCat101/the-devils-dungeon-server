--this WILL cause inconsistencies, use Count instead
-- https://www.w3schools.com/sql/sql_count.asp
-- SELECT COUNT(*) FROM level_stars WHERE 
--
-- ALTER TABLE levels
-- ADD COLUMN stars INTEGER DEFAULT 0;

ALTER TABLE levels
ADD COLUMN total_crystals INTEGER DEFAULT 0;

