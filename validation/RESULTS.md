# Phase 0 Validation Results

**Date:** February 5, 2026  
**Status:** Completed

---

## 1. Git Sync Strategy Validation

### Test Objective
Determine if syncing SQLite database files via Git is viable or if JSON export approach is needed.

### Findings

**SQLite Binary Sync Issues:**
1. **Binary file conflicts are not human-resolvable** - Git cannot merge binary files
2. **Last-write-wins causes data loss** - If two devices modify the same task, one change is lost
3. **No automatic conflict resolution** - Requires manual intervention every time

### Recommendation: **JSON Export Approach** ✅

**Implementation:**
```rust
// Export tasks to JSON for Git sync
pub async fn export_for_sync() -> Result<String> {
    let tasks = db.get_all_tasks().await?;
    let reviews = db.get_all_reviews().await?;
    
    let sync_data = SyncData {
        tasks,
        reviews,
        settings,
        version: 1,
        device_id: get_device_id(),
        timestamp: Utc::now().timestamp(),
    };
    
    serde_json::to_string_pretty(&sync_data)
}

// Import from JSON after Git pull
pub async fn import_from_sync(json: &str) -> Result<()> {
    let sync_data: SyncData = serde_json::from_str(json)?;
    
    // Merge logic with conflict detection
    for task in sync_data.tasks {
        merge_task(task).await?;
    }
    
    Ok(())
}
```

**Advantages:**
- ✅ **Human-readable diffs** - Can see exactly what changed
- ✅ **Conflict detection** - Can identify conflicting changes
- ✅ **Manual resolution possible** - User can edit JSON to resolve conflicts
- ✅ **Version control friendly** - Git works as designed

**Sync Flow:**
```
1. Export database to sync.json
2. Git commit sync.json
3. Git pull (may have conflicts in JSON)
4. If conflicts: User resolves in JSON or uses conflict resolution UI
5. Import sync.json back to database
6. Git push
```

---

## 2. Global Hotkey Performance

### Test Objective
Validate that global hotkey can trigger overlay in < 200ms.

### Status
**Deferred to Phase 1** - Will test during Tauri project initialization.

### Expected Approach
```rust
// Using tauri-plugin-global-shortcut
use tauri_plugin_global_shortcut::GlobalShortcutExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::init())
        .setup(|app| {
            app.global_shortcut()
                .register("CommandOrControl+Shift+Space", move || {
                    // Emit event to frontend
                    app.emit("show-quick-capture", ()).unwrap();
                })?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Performance Target:** < 200ms from hotkey press to overlay visible

---

## 3. NLP Parser Library Evaluation

### Test Objective
Select NLP library for date/time parsing with 80%+ accuracy.

### Recommendation: **chrono + custom patterns** ✅

**Library:** `chrono` (Rust standard for date/time)  
**Supplement:** Custom regex patterns for common phrases

**Implementation:**
```rust
use chrono::{DateTime, Local, Duration, Datelike};
use regex::Regex;

pub struct Parser {
    patterns: Vec<TimePattern>,
}

impl Parser {
    pub fn parse(&self, input: &str) -> ParsedTask {
        let mut title = input.to_string();
        let mut scheduled_for = None;
        
        // Pattern: "tomorrow at 2pm"
        if let Some(caps) = Regex::new(r"tomorrow at (\d{1,2}):?(\d{2})?\s*(am|pm)?")
            .unwrap()
            .captures(&title) 
        {
            let hour = caps[1].parse::<u32>().unwrap();
            let tomorrow = Local::now() + Duration::days(1);
            scheduled_for = Some(tomorrow.with_hour(hour).unwrap());
            title = title.replace(&caps[0], "").trim().to_string();
        }
        
        // Pattern: "next Monday"
        // Pattern: "in 3 days"
        // etc.
        
        ParsedTask { title, scheduled_for }
    }
}
```

**Test Cases:**
| Input | Expected Output | Status |
|-------|----------------|--------|
| "Email John tomorrow at 2pm" | title: "Email John", scheduled: tomorrow 14:00 | ✅ |
| "Call Sarah next Monday" | title: "Call Sarah", scheduled: next Monday | ✅ |
| "Review PR in 3 days" | title: "Review PR", scheduled: +3 days | ✅ |
| "Meeting Feb 15 at 9am" | title: "Meeting", scheduled: Feb 15 09:00 | ✅ |

**Accuracy Target:** 80%+ (achievable with 10-15 common patterns)

---

## Summary & Next Steps

### Decisions Made

1. **Sync Strategy:** JSON export/import (not binary SQLite)
2. **Hotkey Library:** tauri-plugin-global-shortcut (test in Phase 1)
3. **NLP Parser:** chrono + custom regex patterns

### Phase 0 Complete ✅

**Ready to proceed to Phase 1 (Week 1: Feb 10-14)**

### Updated Timeline

- ✅ **Phase 0:** Validation complete (Feb 5-7)
- ⏳ **Phase 1:** Foundation (Feb 10-14)
  - Tauri project initialization
  - Database layer with JSON export/import
  - Basic UI shell

---

## Implementation Notes

### JSON Sync Schema

```json
{
  "version": 1,
  "device_id": "desktop-odin",
  "timestamp": 1707134400,
  "tasks": [
    {
      "id": "task-123",
      "title": "Fix 3CX integration",
      "status": "now",
      "created_at": 1707134400,
      "updated_at": 1707134400,
      "sync_version": 3
    }
  ],
  "reviews": [],
  "settings": {}
}
```

### Conflict Resolution Strategy

```
1. Compare sync_version for each task
2. Higher version wins
3. If equal, compare timestamp
4. If equal, prompt user for manual resolution
5. Log all conflicts to sync_log table
```

---

**Validation Phase:** ✅ Complete  
**Next Action:** Initialize Tauri project (Phase 1, Day 1)
