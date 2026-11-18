use crate::twitch::Video;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

pub fn select(videos: &[Video]) -> Result<Option<&Video>, String> {
    if videos.is_empty() {
        return Ok(None);
    }

    let items: Vec<ListItem> = videos
        .iter()
        .map(|video| {
            let title = video.title.lines().collect::<Vec<_>>().join(" ");
            ListItem::new(Line::from(format!(
                "{title}  [{}]  {}",
                video.duration, video.created_at
            )))
        })
        .collect();
    let mut state = ListState::default().with_selected(Some(0));

    let selected =
        ratatui::run(|terminal| run_selector(terminal, &items, videos.len(), &mut state))
            .map_err(|e| format!("video selector failed: {e}"))?;
    Ok(selected.map(|index| &videos[index]))
}

fn run_selector(
    terminal: &mut DefaultTerminal,
    items: &[ListItem],
    item_count: usize,
    state: &mut ListState,
) -> io::Result<Option<usize>> {
    loop {
        terminal.draw(|frame| render(frame, items, item_count, state))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        match key.code {
            KeyCode::Up => {
                let selected = state.selected().unwrap_or(0);
                state.select(Some(selected.saturating_sub(1)));
            }
            KeyCode::Down => {
                let selected = state.selected().unwrap_or(0);
                state.select(Some((selected + 1).min(item_count - 1)));
            }
            KeyCode::Enter => return Ok(state.selected()),
            KeyCode::Esc | KeyCode::Char('q') => return Ok(None),
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, items: &[ListItem], item_count: usize, state: &mut ListState) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("Select Twitch video • ↑/↓ move • Enter select • q cancel"),
        areas[0],
    );

    let list = List::new(items.iter().cloned())
        .block(Block::default().borders(Borders::ALL).title(" Videos "))
        .highlight_symbol("› ")
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    frame.render_stateful_widget(list, areas[1], state);

    let selected = state.selected().map_or(0, |index| index + 1);
    frame.render_widget(
        Paragraph::new(format!("Video {selected} of {item_count}")),
        areas[2],
    );
}
