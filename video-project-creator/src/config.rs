pub struct Config {
    pub base_dir: String,
    pub default_dirs: Vec<&'static str>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            base_dir: format!(
                "{}/{}",
                std::env::var("HOME").unwrap_or_default(),
                "MyVideos"
            ),
            default_dirs: vec![
                "01_project",
                "02_source",
                "03_assets",
                "04_music_sfx",
                "05_export",
            ],
        }
    }
}
