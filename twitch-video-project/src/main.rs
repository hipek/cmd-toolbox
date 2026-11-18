mod cli;
mod project;
mod selector;
mod twitch;

use std::process::Command;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let options = cli::parse(std::env::args().skip(1).collect())?;
    let login = match options.login.or_else(|| std::env::var("TWITCH_LOGIN").ok()) {
        Some(login) if !login.trim().is_empty() => login.trim().to_owned(),
        _ => {
            eprint!("Twitch channel login: ");
            let mut login = String::new();
            std::io::stdin()
                .read_line(&mut login)
                .map_err(|e| format!("could not read Twitch login: {e}"))?;
            let login = login.trim().to_owned();
            if login.is_empty() {
                return Err("Twitch login cannot be empty".into());
            }
            login
        }
    };

    let videos = twitch::list_videos(
        &login,
        options.client_id.as_deref(),
        options.client_secret.as_deref(),
    )?;
    if videos.is_empty() {
        return Err(format!("no Twitch videos found for '{login}'"));
    }
    let selected = selector::select(&videos)?;
    let Some(video) = selected else {
        return Ok(());
    };

    let project_path = project::create(&options.project_name)?;
    let filename = format!("{}.mp4", project::slug(&options.project_name));
    let output_path = project_path.join("02_source").join(filename);
    let status = Command::new("yt-dlp")
        .arg("--no-playlist")
        .arg("--merge-output-format")
        .arg("mp4")
        .arg("--output")
        .arg(&output_path)
        .arg(format!("https://www.twitch.tv/videos/{}", video.id))
        .status()
        .map_err(|e| {
            format!("could not run yt-dlp: {e}. Install yt-dlp and ensure it is on PATH")
        })?;
    if !status.success() {
        return Err(format!("yt-dlp exited with status {status}"));
    }

    let transcript_path = project_path.join("03_assets").join(format!(
        "{}-transcript.txt",
        project::slug(&options.project_name)
    ));
    let transcript = Command::new("wh")
        .arg("transcribe")
        .arg(&output_path)
        .arg("--raw")
        .output()
        .map_err(|e| {
            format!(
                "video downloaded to {}, but could not run wh: {e}",
                output_path.display()
            )
        })?;
    if !transcript.status.success() {
        return Err(format!(
            "video downloaded to {}, but wh transcribe failed: {}",
            output_path.display(),
            String::from_utf8_lossy(&transcript.stderr).trim()
        ));
    }
    std::fs::write(&transcript_path, transcript.stdout).map_err(|e| {
        format!(
            "video downloaded, but could not save transcript to {}: {e}",
            transcript_path.display()
        )
    })?;

    println!(
        "Downloaded '{}' into {}\nSaved transcript to {}",
        video.title,
        output_path.display(),
        transcript_path.display()
    );
    Ok(())
}
