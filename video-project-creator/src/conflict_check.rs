use std::path::Path;

#[derive(Debug)]
pub struct ConflictError {
    pub path: String,
}

impl std::fmt::Display for ConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Project directory already exists:\n  {}", self.path)
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
