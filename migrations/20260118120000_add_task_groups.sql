-- Create task_groups table
CREATE TABLE task_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    ordering INT NOT NULL
);

-- Seed default groups
INSERT INTO task_groups (name, description, ordering) VALUES
('Day Part 1', 'Delegation: Can I delegate this task?', 1),
('Day Part 2', 'Implementation: Doing/Implementing the task', 2),
('Day Part 3', 'Recurring: Recurring tasks, emails, etc.', 3);

-- Add group_id to recordings table
ALTER TABLE recordings
ADD COLUMN group_id UUID REFERENCES task_groups(id);

-- Create index for group_id
CREATE INDEX idx_recordings_group_id ON recordings(group_id);
