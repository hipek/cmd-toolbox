use std::path::PathBuf;

use crate::config;
use crate::parser::ProjectName;

pub fn build(config: &config::Config, name: &ProjectName) -> PathBuf {
    let now = chrono::Utc::now();
    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    PathBuf::from(&config.base_dir).join(format!("{}-{}-{}", year, month, &name.sanitized))
}

#[cfg(test)]
mod tests {
    use super::build;
    use crate::{config::Config, parser::ProjectName};
    use std::path::PathBuf;

    #[test]
    fn builds_path_with_current_year_month_and_sanitized_name() {
        let config = Config {
            base_dir: "/videos".into(),
            default_dirs: vec![],
        };
        let name = ProjectName {
            raw: "My Stream".into(),
            sanitized: "my-stream".into(),
        };
        let path = build(&config, &name);
        let prefix = chrono::Utc::now().format("%Y-%m-").to_string();

        assert_eq!(path, PathBuf::from(format!("/videos/{prefix}my-stream")));
    }
}
