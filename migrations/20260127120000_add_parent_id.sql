-- Add parent_id to link multiple exploded tasks to a single source recording
ALTER TABLE recordings ADD COLUMN parent_id UUID REFERENCES recordings(id) ON DELETE CASCADE;
