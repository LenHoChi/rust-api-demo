CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO users (name, email, password) VALUES
    ('John Smith', 'john.smith@gmail.com', '123456'),
    ('Emily Johnson', 'emily.johnson@gmail.com', '123456'),
    ('Michael Brown', 'michael.brown@gmail.com', '123456'),
    ('Sarah Davis', 'sarah.davis@gmail.com', '123456'),
    ('David Wilson', 'david.wilson@gmail.com', '123456');