use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use walkdir::WalkDir;

fn list_directory_contents(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let path = ".";
    let files = list_directory_contents(path);
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = files
                .iter()
                .enumerate()
                .map(|(i, file)| {
                    let item = ListItem::new(Spans::from(vec![Span::raw(file)]));
                    if i == selected {
                        item.style(Style::default().bg(Color::Blue))
                    } else {
                        item
                    }
                })
                .collect();

            let files_list =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Files"));
            f.render_widget(files_list, chunks[0]);
        })?;

        match event::read()? {
            CEvent::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Down => {
                    selected = (selected + 1) % files.len();
                }
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    } else {
                        selected = files.len() - 1; // Cycle to the last item
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
