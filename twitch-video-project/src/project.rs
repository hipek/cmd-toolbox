use std::path::PathBuf;
use std::process::Command;

pub fn create(name: &str) -> Result<PathBuf, String> {
    let output = Command::new("cvp")
        .arg(name)
        .output()
        .map_err(|e| format!("could not run cvp: {e}. Install cvp and ensure it is on PATH"))?;
    if !output.status.success() {
        return Err(format!(
            "cvp failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let path = stdout
        .lines()
        .find_map(|line| line.strip_prefix("✓ Created project: "))
        .ok_or_else(|| format!("could not read project path from cvp output:\n{stdout}"))?;
    let path = PathBuf::from(path.trim());
    if !path.join("02_source").is_dir() {
        return Err(format!(
            "cvp created project without expected source directory: {}",
            path.display()
        ));
    }
    Ok(path)
}

pub fn slug(input: &str) -> String {
    let mut result = String::new();
    let mut dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            result.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !result.is_empty() && !dash {
            result.push('-');
            dash = true;
        }
    }
    result.trim_matches('-').to_owned()
}

#[cfg(test)]
mod tests {
    use super::slug;

    #[test]
    fn slug_lowercases_and_replaces_separators() {
        assert_eq!(slug("My Stream: Day 1!"), "my-stream-day-1");
    }

    #[test]
    fn slug_preserves_underscores() {
        assert_eq!(slug("Project_Name"), "project_name");
    }

    #[test]
    fn slug_discards_non_ascii_characters() {
        assert_eq!(slug("Tęst video"), "t-st-video");
    }

    #[test]
    fn slug_returns_empty_for_non_slug_characters() {
        assert_eq!(slug("!!!"), "");
    }
}
