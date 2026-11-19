use std::path::Path;

#[derive(Debug)]
pub struct ConflictError {
    pub path: String,
}

impl std::fmt::Display for ConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: Project directory already exists:\n  {}",
            self.path
        )
    }
}

pub fn check(path: &Path) -> Result<(), ConflictError> {
    if path.exists() {
        Err(ConflictError {
            path: path.to_string_lossy().to_string(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::check;
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
        std::env::temp_dir().join(format!("cvp-conflict-{}-{nonce}", std::process::id()))
    }

    #[test]
    fn accepts_path_that_does_not_exist() {
        let path = unique_path();
        assert!(check(&path).is_ok());
    }

    #[test]
    fn rejects_existing_path() {
        let path = unique_path();
        fs::create_dir_all(&path).unwrap();
        let error = check(&path).unwrap_err();
        assert_eq!(error.path, path.to_string_lossy());
        fs::remove_dir_all(path).unwrap();
    }
}
