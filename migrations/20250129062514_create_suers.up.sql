-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    hashed_pwd TEXT NOT NULL,
    privilege TEXT NOT NULL CHECK (privilege IN ('user', 'vip', 'mod', 'admin')) DEFAULT 'user',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);