use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub context: Option<String>,
    pub scheduled_for: Option<i64>,
    pub completed_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub original_input: Option<String>,
    pub source: String,
    pub tags: Option<String>,
    pub sync_version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Now,
    Next,
    Waiting,
    Someday,
    Done,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Now => "now",
            TaskStatus::Next => "next",
            TaskStatus::Waiting => "waiting",
            TaskStatus::Someday => "someday",
            TaskStatus::Done => "done",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "now" => Some(TaskStatus::Now),
            "next" => Some(TaskStatus::Next),
            "waiting" => Some(TaskStatus::Waiting),
            "someday" => Some(TaskStatus::Someday),
            "done" => Some(TaskStatus::Done),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    pub now_task: Option<Task>,
    pub next_tasks: Vec<Task>,
    pub waiting_tasks: Vec<Task>,
    pub review_due_in_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewLog {
    pub id: String,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub tasks_triaged: i32,
    pub tasks_completed: i32,
    pub tasks_deferred: i32,
    pub tasks_deleted: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncLog {
    pub id: String,
    pub timestamp: i64,
    pub operation: String,
    pub status: String,
    pub message: Option<String>,
}
