// `crossterm` provides cross-platform support for terminal handling, including events,
// execution control, and terminal state management.
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::fs;
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

fn read_file_preview(path: &str) -> String {
    const MAX_LINES: usize = 20; // Limit the preview to 10 lines
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let lines: Vec<_> = reader.lines()
                                    .take(MAX_LINES)
                                    .collect::<Result<_, _>>()
                                    .unwrap_or_else(|_| vec!["Error reading file".to_string()]);
            lines.join("\n")
        }
        Err(_) => "Cannot open file".to_string(),
    }
}

fn generate_file_info(path: &str) -> String {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(metadata) => {
            let size = metadata.len();
            let modified = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_date = modified.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards").as_secs();
            let modified_time = chrono::NaiveDateTime::from_timestamp(modified_date as i64, 0);
            format!("Name: {}\nSize: {} bytes\nModified: {}", path, size, modified_time)
        }
        Err(_) => "Cannot retrieve file info".to_string(),
    }
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
    let mut scroll: usize = 0; // Tracks the topmost item in the list view
    let display_count = 20; // Example fixed value, adjust based on your UI layout



    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
                // Split the right side into two vertically
            let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Upper right for file preview
                Constraint::Percentage(50), // Lower right for file info
            ])
            .split(chunks[1]);

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
                    .skip(scroll)
                    .take(display_count)
                    .enumerate()
                    .map(|(i, file)| {
                        // Adjust the index to be relative to the start of the displayed list
                        let display_index = i + scroll;
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
                        if display_index == selected {
                            item = item.style(Style::default().bg(Color::Blue))
                        } else if file.is_dir {
                            item = item.style(Style::default().fg(Color::Green))
                        }
                        item
                    })
                    .collect();
                

            let files_list =
                List::new(items).block(Block::default().borders(Borders::ALL).title(current_dir_name));

                let preview_content = if files[selected].is_dir {
                    "Directory selected - no preview available".to_string()
                } else {
                    read_file_preview(&files[selected].path)
                };
            
                let paragraph = tui::widgets::Paragraph::new(preview_content)
                    .block(Block::default().borders(Borders::ALL).title("Preview"))
                    .wrap(tui::widgets::Wrap { trim: true });
                    let file_info_content = generate_file_info(&files[selected].path);
                let file_info = tui::widgets::Paragraph::new(file_info_content)
                    .block(Block::default().borders(Borders::ALL).title("File Info"))
                    .wrap(tui::widgets::Wrap { trim: true });
            f.render_widget(files_list, chunks[0]);
            f.render_widget(paragraph, right_chunks[0]);
            f.render_widget(file_info, right_chunks[1]);
        })?;

        match event::read()? {
            CEvent::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Down => {
                    if selected < files.len() - 1 {
                        selected += 1;
                        // Ensure the selected item is always visible
                        if selected >= scroll + display_count {
                            scroll = selected - display_count + 1; // Adjust scroll to keep the selected item visible
                        }
                    }
                },
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                        if selected < scroll {
                            scroll = selected; // Scroll up when the selection moves above the current view
                        }
                    }
                },                
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
