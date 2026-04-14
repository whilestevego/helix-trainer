//! Persistent per-exercise completion progress.
//!
//! Stored as `<exercises_dir>/.progress.json`. Keyed by exercise id (the path
//! under `exercises/` without `.hxt`). Tracks first/last completion timestamps
//! and a count incremented on each not-passed → passed transition.
//!
//! All I/O is fail-soft: a missing or corrupt file yields an empty store
//! rather than crashing the TUI. Writes go through a temp file + rename for
//! atomicity.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const PROGRESS_FILE: &str = ".progress.json";
const CURRENT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExerciseProgress {
    pub first_completed_at: DateTime<Utc>,
    pub last_completed_at: DateTime<Utc>,
    pub completion_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgressData {
    version: u32,
    exercises: HashMap<String, ExerciseProgress>,
}

impl Default for ProgressData {
    fn default() -> Self {
        ProgressData {
            version: CURRENT_VERSION,
            exercises: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Progress {
    path: PathBuf,
    data: ProgressData,
}

impl Progress {
    /// Load progress from `<exercises_dir>/.progress.json`. Never fails:
    /// missing file → empty store; parse error → empty store + stderr warning.
    pub fn load(exercises_dir: &Path) -> Self {
        let path = exercises_dir.join(PROGRESS_FILE);
        let data = match fs::read_to_string(&path) {
            Ok(contents) => match serde_json::from_str::<ProgressData>(&contents) {
                Ok(parsed) => parsed,
                Err(e) => {
                    eprintln!(
                        "warning: could not parse {}: {} — starting with empty progress",
                        path.display(),
                        e
                    );
                    ProgressData::default()
                }
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => ProgressData::default(),
            Err(e) => {
                eprintln!(
                    "warning: could not read {}: {} — starting with empty progress",
                    path.display(),
                    e
                );
                ProgressData::default()
            }
        };
        Progress { path, data }
    }

    /// Atomic save: write to `<file>.tmp`, then rename. If the parent dir is
    /// missing we silently skip (the tests sometimes use a synthetic path).
    pub fn save(&self) -> io::Result<()> {
        let parent = match self.path.parent() {
            Some(p) => p,
            None => return Ok(()),
        };
        if !parent.exists() {
            return Ok(());
        }
        let tmp = self.path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(&self.data).map_err(io::Error::other)?;
        fs::write(&tmp, json)?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ExerciseProgress> {
        self.data.exercises.get(id)
    }

    /// Record that an exercise has just been passed. Inserts on first pass,
    /// otherwise bumps the count and updates `last_completed_at`.
    pub fn record_pass(&mut self, id: &str, now: DateTime<Utc>) {
        self.data
            .exercises
            .entry(id.to_string())
            .and_modify(|p| {
                p.last_completed_at = now;
                p.completion_count = p.completion_count.saturating_add(1);
            })
            .or_insert(ExerciseProgress {
                first_completed_at: now,
                last_completed_at: now,
                completion_count: 1,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use tempfile::tempdir;

    #[test]
    fn missing_file_yields_empty_store() {
        let dir = tempdir().unwrap();
        let progress = Progress::load(dir.path());
        assert!(progress.get("anything").is_none());
    }

    #[test]
    fn record_pass_inserts_and_increments() {
        let dir = tempdir().unwrap();
        let mut progress = Progress::load(dir.path());
        let t1 = Utc.with_ymd_and_hms(2026, 4, 13, 10, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2026, 4, 14, 11, 0, 0).unwrap();

        progress.record_pass("movement/m1", t1);
        let entry = progress.get("movement/m1").unwrap();
        assert_eq!(entry.completion_count, 1);
        assert_eq!(entry.first_completed_at, t1);
        assert_eq!(entry.last_completed_at, t1);

        progress.record_pass("movement/m1", t2);
        let entry = progress.get("movement/m1").unwrap();
        assert_eq!(entry.completion_count, 2);
        assert_eq!(entry.first_completed_at, t1, "first must not change");
        assert_eq!(entry.last_completed_at, t2);
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempdir().unwrap();
        let t = Utc.with_ymd_and_hms(2026, 4, 13, 10, 0, 0).unwrap();
        {
            let mut progress = Progress::load(dir.path());
            progress.record_pass("movement/m1", t);
            progress.record_pass("selection/s1", t);
            progress.save().unwrap();
        }
        let progress = Progress::load(dir.path());
        assert_eq!(progress.get("movement/m1").unwrap().completion_count, 1);
        assert_eq!(progress.get("selection/s1").unwrap().completion_count, 1);
    }

    #[test]
    fn corrupt_file_yields_empty_store() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join(PROGRESS_FILE), "{not valid json").unwrap();
        let progress = Progress::load(dir.path());
        assert!(progress.get("movement/m1").is_none());
    }

    #[test]
    fn save_with_missing_parent_is_noop() {
        // Path under a non-existent dir — save() should swallow rather than error.
        let bogus = PathBuf::from("/tmp/helixir-does-not-exist-xyz-9999");
        let progress = Progress::load(&bogus);
        // Returns Ok — silently skips because parent doesn't exist.
        assert!(progress.save().is_ok());
    }
}
