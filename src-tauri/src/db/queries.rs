use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Row};
use uuid::Uuid;

use super::connection::Database;
use super::models::{DashboardData, Task, TaskStatus};

impl Database {
    /// Create a new task
    pub fn create_task(
        &self,
        title: String,
        status: TaskStatus,
        context: Option<String>,
        scheduled_for: Option<i64>,
        original_input: Option<String>,
        source: String,
        tags: Option<String>,
    ) -> Result<Task> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        self.conn().execute(
            "INSERT INTO tasks (id, title, status, context, scheduled_for, created_at, updated_at, original_input, source, tags, sync_version)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 1)",
            params![
                id,
                title,
                status.as_str(),
                context,
                scheduled_for,
                now,
                now,
                original_input,
                source,
                tags,
            ],
        )?;

        self.get_task(&id)
    }

    /// Get a task by ID
    pub fn get_task(&self, id: &str) -> Result<Task> {
        let task = self.conn().query_row(
            "SELECT id, title, status, context, scheduled_for, completed_at, created_at, updated_at, original_input, source, tags, sync_version
             FROM tasks WHERE id = ?1",
            params![id],
            |row| self.row_to_task(row),
        )?;

        Ok(task)
    }

    /// Get dashboard data (NOW task, NEXT tasks, WAITING tasks)
    pub fn get_dashboard_data(&self) -> Result<DashboardData> {
        // Get NOW task
        let now_task = self.conn().query_row(
            "SELECT id, title, status, context, scheduled_for, completed_at, created_at, updated_at, original_input, source, tags, sync_version
             FROM tasks WHERE status = 'now' LIMIT 1",
            [],
            |row| self.row_to_task(row),
        ).ok();

        // Get NEXT tasks (limit 10)
        let mut stmt = self.conn().prepare(
            "SELECT id, title, status, context, scheduled_for, completed_at, created_at, updated_at, original_input, source, tags, sync_version
             FROM tasks WHERE status = 'next' ORDER BY created_at DESC LIMIT 10"
        )?;

        let next_tasks = stmt
            .query_map([], |row| self.row_to_task(row))?
            .collect::<Result<Vec<_>, _>>()?;

        // Get WAITING tasks (limit 10)
        let mut stmt = self.conn().prepare(
            "SELECT id, title, status, context, scheduled_for, completed_at, created_at, updated_at, original_input, source, tags, sync_version
             FROM tasks WHERE status = 'waiting' ORDER BY created_at DESC LIMIT 10"
        )?;

        let waiting_tasks = stmt
            .query_map([], |row| self.row_to_task(row))?
            .collect::<Result<Vec<_>, _>>()?;

        // Calculate review due days
        let last_review: i64 = self
            .conn()
            .query_row(
                "SELECT value FROM settings WHERE key = 'last_review_date'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let review_frequency: i64 = self
            .conn()
            .query_row(
                "SELECT value FROM settings WHERE key = 'review_frequency_days'",
                [],
                |row| row.get::<_, String>(0).map(|s| s.parse().unwrap_or(7)),
            )
            .unwrap_or(7);

        let now = Utc::now().timestamp();
        let days_since_review = (now - last_review) / 86400;
        let review_due_in_days = (review_frequency - days_since_review) as i32;

        Ok(DashboardData {
            now_task,
            next_tasks,
            waiting_tasks,
            review_due_in_days,
        })
    }

    /// Update task status
    pub fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<()> {
        let completed_at = if status == TaskStatus::Done {
            Some(Utc::now().timestamp())
        } else {
            None
        };

        self.conn().execute(
            "UPDATE tasks SET status = ?1, completed_at = ?2, sync_version = sync_version + 1 WHERE id = ?3",
            params![status.as_str(), completed_at, id],
        )?;

        Ok(())
    }

    /// Delete a task
    pub fn delete_task(&self, id: &str) -> Result<()> {
        self.conn()
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Helper: Convert database row to Task
    fn row_to_task(&self, row: &Row) -> rusqlite::Result<Task> {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            status: TaskStatus::from_str(&row.get::<_, String>(2)?).unwrap_or(TaskStatus::Next),
            context: row.get(3)?,
            scheduled_for: row.get(4)?,
            completed_at: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            original_input: row.get(8)?,
            source: row.get(9)?,
            tags: row.get(10)?,
            sync_version: row.get(11)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn setup_test_db() -> Database {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        Database::new(db_path).unwrap()
    }

    #[test]
    fn test_create_and_get_task() {
        let db = setup_test_db();

        let task = db
            .create_task(
                "Test task".to_string(),
                TaskStatus::Next,
                None,
                None,
                None,
                "test".to_string(),
                None,
            )
            .unwrap();

        assert_eq!(task.title, "Test task");
        assert_eq!(task.status, TaskStatus::Next);

        let retrieved = db.get_task(&task.id).unwrap();
        assert_eq!(retrieved.id, task.id);
    }

    #[test]
    fn test_dashboard_data() {
        let db = setup_test_db();

        // Create NOW task
        db.create_task(
            "NOW task".to_string(),
            TaskStatus::Now,
            None,
            None,
            None,
            "test".to_string(),
            None,
        )
        .unwrap();

        // Create NEXT tasks
        db.create_task(
            "NEXT task 1".to_string(),
            TaskStatus::Next,
            None,
            None,
            None,
            "test".to_string(),
            None,
        )
        .unwrap();

        let dashboard = db.get_dashboard_data().unwrap();

        assert!(dashboard.now_task.is_some());
        assert_eq!(dashboard.now_task.unwrap().title, "NOW task");
        assert_eq!(dashboard.next_tasks.len(), 1);
    }

    #[test]
    fn test_one_now_task_enforcement() {
        let db = setup_test_db();

        // Create first NOW task
        let task1 = db
            .create_task(
                "NOW task 1".to_string(),
                TaskStatus::Now,
                None,
                None,
                None,
                "test".to_string(),
                None,
            )
            .unwrap();

        // Create second task as NEXT
        let task2 = db
            .create_task(
                "Task 2".to_string(),
                TaskStatus::Next,
                None,
                None,
                None,
                "test".to_string(),
                None,
            )
            .unwrap();

        // Update task2 to NOW (should demote task1 to NEXT)
        db.update_task_status(&task2.id, TaskStatus::Now).unwrap();

        // Verify task1 is now NEXT
        let task1_updated = db.get_task(&task1.id).unwrap();
        assert_eq!(task1_updated.status, TaskStatus::Next);

        // Verify task2 is NOW
        let task2_updated = db.get_task(&task2.id).unwrap();
        assert_eq!(task2_updated.status, TaskStatus::Now);
    }
}
