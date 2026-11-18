use std::fs;
use std::path::Path;

use crate::config;

pub fn create(config: &config::Config, path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| {
        format!(
            "Error: Could not create directory '{}': {}",
            path.display(),
            e
        )
    })?;

    for dir in &config.default_dirs {
        let subdir = path.join(dir);
        fs::create_dir_all(&subdir)
            .map_err(|e| format!("Error: Could not create '{}': {}", subdir.display(), e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::create;
    use crate::config::Config;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_path() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("cvp-create-{}-{nonce}", std::process::id()))
    }

    #[test]
    fn creates_project_and_configured_subdirectories() {
        let path = unique_path();
        let config = Config {
            base_dir: String::new(),
            default_dirs: vec!["01_project", "02_source", "03_assets"],
        };

        create(&config, &path).unwrap();

        assert!(path.is_dir());
        for dir in &config.default_dirs {
            assert!(path.join(dir).is_dir(), "missing directory {dir}");
        }
        fs::remove_dir_all(path).unwrap();
    }
}
