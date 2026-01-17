-- Create recordings table
DROP TABLE IF EXISTS recordings;

CREATE TABLE recordings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    transcription_text JSONB,
    transcription_status TEXT DEFAULT 'PENDING'
);

-- Index for date filtering
CREATE INDEX idx_recordings_created_at ON recordings(created_at);