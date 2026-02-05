-- TaskFlow Database Schema
-- Version: 1
-- Created: 2026-02-05

PRAGMA foreign_keys = ON;

-- Tasks table
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'next' CHECK(status IN ('now', 'next', 'waiting', 'someday', 'done')),
    context TEXT,
    scheduled_for INTEGER,
    completed_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    original_input TEXT,
    source TEXT NOT NULL,
    tags TEXT,
    sync_version INTEGER NOT NULL DEFAULT 1
);

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Weekly review log
CREATE TABLE IF NOT EXISTS review_log (
    id TEXT PRIMARY KEY,
    started_at INTEGER NOT NULL,
    completed_at INTEGER,
    tasks_triaged INTEGER DEFAULT 0,
    tasks_completed INTEGER DEFAULT 0,
    tasks_deferred INTEGER DEFAULT 0,
    tasks_deleted INTEGER DEFAULT 0
);

-- Sync log
CREATE TABLE IF NOT EXISTS sync_log (
    id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    operation TEXT NOT NULL,
    status TEXT NOT NULL,
    message TEXT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_scheduled ON tasks(scheduled_for);
CREATE INDEX IF NOT EXISTS idx_tasks_created ON tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_sync_log_timestamp ON sync_log(timestamp);

-- Trigger: Enforce one NOW task rule
CREATE TRIGGER IF NOT EXISTS enforce_one_now_task
BEFORE UPDATE OF status ON tasks
WHEN NEW.status = 'now' AND OLD.status != 'now'
BEGIN
    UPDATE tasks SET status = 'next' WHERE status = 'now' AND id != NEW.id;
END;

-- Trigger: Auto-update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_task_timestamp
AFTER UPDATE ON tasks
FOR EACH ROW
BEGIN
    UPDATE tasks SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- Insert default settings
INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
    ('last_review_date', '0', strftime('%s', 'now')),
    ('review_frequency_days', '7', strftime('%s', 'now')),
    ('git_sync_enabled', 'false', strftime('%s', 'now')),
    ('git_sync_interval_minutes', '5', strftime('%s', 'now'));
