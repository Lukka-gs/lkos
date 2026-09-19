use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub schema_version: u32,
    pub always_on_top: bool,
    pub probe_count: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            always_on_top: true,
            probe_count: 0,
        }
    }
}

pub struct Store {
    path: PathBuf,
    pub current: Settings,
}

pub fn migrate(mut value: serde_json::Value) -> Result<Settings, String> {
    let version = value
        .get("schemaVersion")
        .and_then(|v| v.as_u64())
        .ok_or("Missing schemaVersion")?;
    match version {
        0 => {
            let object = value.as_object_mut().ok_or("Expected settings object")?;
            object.insert("schemaVersion".into(), 1.into());
            object.insert("probeCount".into(), 0.into());
        }
        1 => {}
        _ => {
            return Err(format!(
                "Unsupported settings schema {version}; original file preserved"
            ));
        }
    }
    serde_json::from_value(value).map_err(|e| e.to_string())
}

impl Store {
    pub fn open(path: PathBuf) -> Result<Self, String> {
        let current = match fs::read(&path) {
            Ok(bytes) => migrate(serde_json::from_slice(&bytes).map_err(|e| e.to_string())?)?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Settings::default(),
            Err(e) => return Err(e.to_string()),
        };
        let mut store = Self { path, current };
        store.save(store.current.clone())?;
        Ok(store)
    }

    /// Same-directory temp file, flush, then atomic replacement. Commit memory last.
    pub fn save(&mut self, next: Settings) -> Result<(), String> {
        if next.schema_version != 1 {
            return Err("Unsupported settings schema".into());
        }
        let parent = self.path.parent().ok_or("Missing settings directory")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        file.write_all(&serde_json::to_vec_pretty(&next).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        file.as_file().sync_all().map_err(|e| e.to_string())?;
        file.persist(&self.path).map_err(|e| e.to_string())?;
        self.current = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persists_changes_across_restart() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut store = Store::open(path.clone()).unwrap();
        let mut next = store.current.clone();
        next.probe_count = 7;
        next.always_on_top = false;
        store.save(next.clone()).unwrap();
        assert_eq!(Store::open(path).unwrap().current, next);
    }
    #[test]
    fn migrates_v0_and_preserves_user_preference() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::write(&path, r#"{"schemaVersion":0,"alwaysOnTop":false}"#).unwrap();
        let store = Store::open(path.clone()).unwrap();
        assert!(!store.current.always_on_top);
        assert_eq!(store.current.schema_version, 1);
        assert_eq!(Store::open(path).unwrap().current.probe_count, 0);
    }
    #[test]
    fn refuses_corrupt_future_and_invalid_settings_without_overwriting() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        for bytes in [
            "not json",
            r#"{"schemaVersion":99}"#,
            r#"{"schemaVersion":1,"alwaysOnTop":"yes","probeCount":0}"#,
            r#"{"schemaVersion":1,"alwaysOnTop":true,"probeCount":-1}"#,
        ] {
            fs::write(&path, bytes).unwrap();
            assert!(Store::open(path.clone()).is_err());
            assert_eq!(fs::read_to_string(&path).unwrap(), bytes);
        }
    }
    #[test]
    fn failed_write_does_not_commit_memory() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store {
            path: dir.path().to_path_buf(),
            current: Settings::default(),
        };
        let mut next = store.current.clone();
        next.probe_count = 8;
        assert!(store.save(next).is_err());
        assert_eq!(store.current.probe_count, 0);
    }
}
