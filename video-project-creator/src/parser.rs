#[derive(Debug)]
pub struct ProjectName {
    #[allow(dead_code)]
    pub raw: String,
    pub sanitized: String,
}

pub fn parse(args: &[String]) -> Result<ProjectName, String> {
    let mut args = args.iter().skip(1).peekable();

    if args.peek().map_or(false, |&a| a == "-h" || a == "--help") {
        print_help();
        std::process::exit(0);
    }

    let name = args.next().ok_or("Error: Missing project name.\n\nUsage: cvp <project-name>")?;

    let sanitized = sanitize(name);
    if sanitized.is_empty() {
        return Err("Error: Project name is empty after sanitization.".into());
    }

    Ok(ProjectName {
        raw: name.clone(),
        sanitized,
    })
}

fn sanitize(input: &str) -> String {
    input
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
        .to_lowercase()
        .trim_matches(|c| c == '-')
        .to_string()
}

fn print_help() {
    println!("Usage: cvp <project-name>");
    println!();
    println!("Creates a new timestamped project directory under ~/MyVideos.");
    println!("e.g. ~/MyVideos/2026-10-my-cool-project/");
    println!();
    println!("Subdirectories created:");
    println!("  01_project");
    println!("  02_source");
    println!("  03_assets");
    println!("  04_music_sfx");
    println!("  05_export");
}
