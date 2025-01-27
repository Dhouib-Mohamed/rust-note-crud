CREATE TABLE IF NOT EXISTS notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(32) NOT NULL UNIQUE,
    content VARCHAR(255) NOT NULL,
    category VARCHAR(32),
    published BOOLEAN DEFAULT FALSE,
    createdat TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
