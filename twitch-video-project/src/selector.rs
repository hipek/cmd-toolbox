use crate::twitch::Video;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

pub fn select(videos: &[Video]) -> Result<Option<&Video>, String> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| format!("could not enable terminal input: {e}"))?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)
        .map_err(|e| format!("terminal setup failed: {e}"))?;
    let mut selected = 0usize;
    let result = loop {
        render(&mut stdout, videos, selected)?;
        match event::read().map_err(|e| format!("could not read keyboard input: {e}"))? {
            Event::Key(key) => match key.code {
                KeyCode::Up => selected = selected.saturating_sub(1),
                KeyCode::Down => selected = (selected + 1).min(videos.len() - 1),
                KeyCode::Enter => break Ok(Some(&videos[selected])),
                KeyCode::Esc | KeyCode::Char('q') => break Ok(None),
                _ => {}
            },
            _ => {}
        }
    };
    let cleanup = execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)
        .and_then(|_| terminal::disable_raw_mode());
    cleanup.map_err(|e| format!("terminal cleanup failed: {e}"))?;
    result
}

fn render(stdout: &mut io::Stdout, videos: &[Video], selected: usize) -> Result<(), String> {
    let (_, rows) = terminal::size().unwrap_or((80, 24));
    let visible_count = (rows as usize).saturating_sub(4).max(1);
    let start = selected
        .saturating_sub(visible_count - 1)
        .min(videos.len().saturating_sub(visible_count));
    let end = (start + visible_count).min(videos.len());
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Print("Select Twitch video (↑/↓, Enter, q to cancel)\n\n")
    )
    .map_err(|e| format!("could not render selector: {e}"))?;
    for (index, video) in videos.iter().enumerate().take(end).skip(start) {
        if index == selected {
            queue!(stdout, SetAttribute(Attribute::Reverse)).map_err(|e| e.to_string())?;
        }
        queue!(
            stdout,
            Print(format!(
                "{}  {}  [{}]  {}\n",
                if index == selected { ">" } else { " " },
                video.title,
                video.duration,
                video.created_at
            ))
        )
        .map_err(|e| e.to_string())?;
        if index == selected {
            queue!(stdout, SetAttribute(Attribute::Reset)).map_err(|e| e.to_string())?;
        }
    }
    queue!(
        stdout,
        Print(format!(
            "\nShowing {}–{} of {}",
            start + 1,
            end,
            videos.len()
        ))
    )
    .map_err(|e| format!("could not render selector status: {e}"))?;
    stdout
        .flush()
        .map_err(|e| format!("could not flush terminal: {e}"))
}
