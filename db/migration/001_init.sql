CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO users (name, email, password) VALUES
    ('Nguyen Van An', 'an@gmail.com', '123456'),
    ('Tran Thi Binh', 'binh@gmail.com', '123456'),
    ('Le Van Cuong', 'cuong@gmail.com', '123456'),
    ('Pham Thi Dung', 'dung@gmail.com', '123456'),
    ('Hoang Van Em', 'em@gmail.com', '123456');