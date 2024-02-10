// `crossterm` provides cross-platform support for terminal handling, including events,
// execution control, and terminal state management.
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::path::{Path, PathBuf};
// `tui` is a library for building Text User Interfaces (TUIs)
use tui::{
    backend::CrosstermBackend, // Connects `tui` with `crossterm` for terminal backend operations.
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
// for recursively traversing filesystem
use walkdir::WalkDir;

struct FileSystemEntry {
    path: String,
    is_dir: bool,
}

fn list_directory_contents(path: &str) -> Vec<FileSystemEntry> {
    WalkDir::new(path)
        .min_depth(1) // Start at depth 1 to skip the root directory itself.
        .max_depth(1) // Limit traversal to the immediate contents of the directory, not going deeper.
        .into_iter()
        .filter_map(Result::ok) // Filter out any errors
        // .filter(|e| e.file_type().is_file()) 
        .map(|e| FileSystemEntry {
            path: e.path().display().to_string(),
            is_dir: e.file_type().is_dir(),
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let path = ".";
    let initial_path = String::from(".");
    let mut current_path = initial_path.clone();
    let mut files = list_directory_contents(path);
    let mut selected = 0;


    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

                // Use the current directory name or the initial path as the title
                let current_dir_name = Path::new(&current_path)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| {
                        Path::new(&current_path)
                            .components()
                            .last()
                            .map(|c| c.as_os_str().to_string_lossy().into_owned())
                            .unwrap_or_else(|| "Directory".into())
                    });

                    let items: Vec<ListItem> = files
                    .iter()
                    .enumerate()
                    .map(|(i, file)| {
                        // Extract just the file name or directory name for display, instead of the full path.
                        let file_name = Path::new(&file.path)
                            .file_name() // Extracts the last component of the path as a file name
                            .unwrap_or_else(|| std::ffi::OsStr::new("Unknown")) // Fallback in case of an error
                            .to_string_lossy(); // Converts the file name to a string
                
                        let display_text = if file.is_dir { format!("{}/", file_name) } else { file_name.into_owned() };
                
                        // Create a Span from the adjusted display text.
                        let content = Spans::from(vec![Span::raw(display_text)]);
                        // Create a ListItem with the content, applying style based on selection or directory status.
                        let mut item = ListItem::new(content);
                        if i == selected {
                            item = item.style(Style::default().bg(Color::Blue))
                        } else if file.is_dir {
                            item = item.style(Style::default().fg(Color::Green))
                        }
                        item
                    })
                    .collect();
                

            let files_list =
                List::new(items).block(Block::default().borders(Borders::ALL).title(current_dir_name));
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
                KeyCode::Enter => {
                    if files[selected].is_dir {
                        // Logic to display contents of the selected directory
                        let new_path = format!("{}/{}", current_path, files[selected].path.trim_start_matches("./"));
                        current_path = new_path;
                        files = list_directory_contents(&current_path);
                        selected = 0; // Reset selection in the new directory
                    }
                }
                KeyCode::Backspace => {
                    // First, handle the result of canonicalize() to get the canonical path
                    if let Ok(canonical_path) = Path::new(&current_path).canonicalize() {
                        // Then, check if the parent of the canonical path exists
                        if let Some(parent_path) = canonical_path.parent() {
                            // Convert the parent path to a String
                            current_path = parent_path.to_string_lossy().into_owned();
                            // Refresh the directory listing based on the new current path
                            files = list_directory_contents(&current_path);
                            selected = 0; // Reset the selection index
                        }
                    }
                },                            
                _ => {}
            },
            _ => {}
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
