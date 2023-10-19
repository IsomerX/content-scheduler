CREATE TABLE IF NOT EXISTS content (
    id TEXT PRIMARY KEY,
    date TIMESTAMP WITH TIME ZONE,
    content TEXT,
    linkedin TEXT,
    twitter TEXT,
    current BOOLEAN
);
