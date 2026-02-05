# Phase 0 Validation Tests

This directory contains prototypes and validation tests for risky technical assumptions.

## Tests

### 1. Git Sync with SQLite Binary
**Goal:** Determine if syncing SQLite database files via Git is viable or if we need JSON export approach.

**Test Plan:**
1. Create two SQLite databases (simulating two devices)
2. Initialize Git repo
3. Commit database from device 1
4. Modify database on device 1, commit
5. Simulate device 2 pulling changes
6. Modify database on both devices (create conflict)
7. Attempt to resolve conflict

**Success Criteria:**
- ✅ Simple changes sync successfully
- ✅ Conflicts are detectable
- ✅ Conflicts can be resolved without data loss

**Fallback:** If conflicts are not resolvable, implement JSON export/import for sync.

---

### 2. Global Hotkey Performance
**Goal:** Validate that global hotkey can trigger overlay in < 200ms.

**Test Plan:**
1. Create minimal Tauri app with hotkey registration
2. Measure time from hotkey press to overlay display
3. Test across different window states (fullscreen, minimized, etc.)

**Success Criteria:**
- ✅ Hotkey latency < 200ms
- ✅ Works when other apps are fullscreen
- ✅ No conflicts with system hotkeys

---

### 3. NLP Parser Library Evaluation
**Goal:** Select NLP library for date/time parsing with 80%+ accuracy.

**Test Plan:**
1. Evaluate `chrono` crate (Rust)
2. Test on 50 sample inputs
3. Measure accuracy

**Sample Inputs:**
- "tomorrow at 2pm"
- "next Monday"
- "in 3 days"
- "Feb 15 at 9am"
- etc.

**Success Criteria:**
- ✅ Accuracy > 80%
- ✅ Library is actively maintained
- ✅ Easy to integrate with Tauri

---

## Results

Results will be documented here after testing.
