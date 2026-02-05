-- Test schema for Git sync validation
-- Simplified version of main schema

PRAGMA foreign_keys = ON;

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'next',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Insert test data
INSERT INTO tasks (id, title, status, created_at, updated_at) VALUES
    ('task-1', 'Test task 1', 'next', 1707134400, 1707134400),
    ('task-2', 'Test task 2', 'now', 1707134400, 1707134400),
    ('task-3', 'Test task 3', 'waiting', 1707134400, 1707134400);
