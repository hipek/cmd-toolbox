use std::path::Path;
use std::fs;

use crate::config;

pub fn create(config: &config::Config, path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| format!("Error: Could not create directory '{}': {}", path.display(), e))?;

    for dir in &config.default_dirs {
        let subdir = path.join(dir);
        fs::create_dir_all(&subdir).map_err(|e| format!("Error: Could not create '{}': {}", subdir.display(), e))?;
    }

    Ok(())
}
