use std::path::PathBuf;

use crate::config;
use crate::parser::ProjectName;

pub fn build(config: &config::Config, name: &ProjectName) -> PathBuf {
    let now = chrono::Utc::now();
    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    PathBuf::from(&config.base_dir)
        .join(format!("{}-{}-{}", year, month, &name.sanitized))
}
