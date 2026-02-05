use anyhow::Result;
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Initialize database connection and run migrations
    pub fn new(db_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        let mut db = Database { conn };
        db.run_migrations()?;

        Ok(db)
    }

    /// Run database migrations
    fn run_migrations(&mut self) -> Result<()> {
        // Create migrations table if it doesn't exist
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                applied_at INTEGER NOT NULL
            )",
            [],
        )?;

        // Check current version
        let current_version: i32 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Migration 1: Initial schema
        if current_version < 1 {
            let migration_sql = include_str!("../../migrations/001_initial_schema.sql");
            self.conn.execute_batch(migration_sql)?;

            self.conn.execute(
                "INSERT INTO migrations (version, applied_at) VALUES (?1, strftime('%s', 'now'))",
                params![1],
            )?;

            println!("Applied migration 001_initial_schema.sql");
        }

        Ok(())
    }

    /// Get database connection reference
    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_initialization() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = Database::new(db_path).unwrap();

        // Verify tables exist
        let table_count: i32 = db.conn()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('tasks', 'settings', 'review_log', 'sync_log')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_count, 4);
    }

    #[test]
    fn test_migrations_applied() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = Database::new(db_path).unwrap();

        // Verify migration was applied
        let version: i32 = db
            .conn()
            .query_row("SELECT MAX(version) FROM migrations", [], |row| row.get(0))
            .unwrap();

        assert_eq!(version, 1);
    }
}
