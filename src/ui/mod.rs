use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

use crate::app::{App, AppMode, Book};
use crate::database::Database;
use crate::history::LibraryHistory;
use std::path::PathBuf;

pub mod components;
pub mod layout;
pub mod events;
pub mod selector;

use components::UIComponents;
use selector::LibrarySelector;

/// Main UI handler for the application
pub struct UI {
    components: UIComponents,
}

impl UI {
    pub fn new() -> Self {
        UI {
            components: UIComponents::new(),
        }
    }

    /// Show library selection UI and return selected library path
    pub async fn select_library(&mut self) -> Result<Option<PathBuf>> {
        // Initialize terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Discover libraries
        let mut selector = LibrarySelector::new();
        selector.discover_libraries().await?;

        if !selector.has_libraries() {
            // Show no libraries found message
            loop {
                terminal.draw(|f| {
                    self.components.render_no_libraries(f, f.size());
                })?;

                if event::poll(Duration::from_millis(250))? {
                    if let Event::Key(_) = event::read()? {
                        break;
                    }
                }
            }

            // Cleanup terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            return Ok(None);
        }

        let mut selected_index = 0;
        let mut in_search_mode = false;

        // Library selection loop
        loop {
            terminal.draw(|f| {
                // Check if we need to render filtered libraries or all libraries
                if in_search_mode {
                    // We need to modify the render function to support search mode indicator
                    self.render_library_selection_with_search(f, f.size(), &selector, selected_index, in_search_mode);
                } else {
                    self.components.render_library_selection(f, f.size(), &selector, selected_index);
                }
            })?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        // Handle search mode toggle
                        KeyCode::Char('/') if !in_search_mode => {
                            in_search_mode = true;
                            selector.set_search_query(String::new());
                            selected_index = 0; // Reset selection when entering search
                        }
                        KeyCode::Esc | KeyCode::Left => {
                            if in_search_mode {
                                // Exit search mode
                                in_search_mode = false;
                                selector.set_search_query(String::new());
                                selected_index = 0;
                            }
                        }
                        // Navigation keys (work in both modes)
                        KeyCode::Up | KeyCode::Char('k') => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if selected_index < selector.get_filtered_libraries().len().saturating_sub(1) {
                                selected_index += 1;
                            }
                        }
                        // Selection
                        KeyCode::Enter | KeyCode::Right => {
                            // Get the library from filtered results if in search mode, or from all libraries otherwise
                            let library = if in_search_mode {
                                selector.get_filtered_library(selected_index)
                            } else {
                                selector.get_library(selected_index)
                            };

                            if let Some(library) = library {
                                // Clone the path to avoid borrowing issues
                                let library_path = library.path.clone();
                                let library_name = Some(library.name.clone());

                                // Save to history with book count
                                if let Err(e) = selector.save_to_history(&library_path, library_name).await {
                                    eprintln!("Warning: Failed to save library to history: {}", e);
                                }

                                // Cleanup terminal
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;
                                return Ok(Some(library_path));
                            }
                        }
                        // Search input (only works in search mode)
                        KeyCode::Char(c) if in_search_mode => {
                            let mut current_query = selector.get_search_query().to_string();
                            current_query.push(c);
                            selector.set_search_query(current_query);
                            selected_index = 0; // Reset selection when search changes
                        }
                        KeyCode::Backspace if in_search_mode => {
                            let mut current_query = selector.get_search_query().to_string();
                            current_query.pop();
                            selector.set_search_query(current_query);
                            selected_index = 0; // Reset selection when search changes
                        }
                        // Quit
                        KeyCode::Char('q') if !in_search_mode => {
                            // Cleanup terminal
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;
                            return Ok(None);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Render library selection with search mode support
    fn render_library_selection_with_search(&self, frame: &mut Frame, area: ratatui::layout::Rect, selector: &LibrarySelector, selected_index: usize, in_search_mode: bool) {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            style::{Color, Style},
            text::{Line, Span},
            widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title bar
                Constraint::Min(0),      // Library list
                Constraint::Length(3),  // Status bar
            ])
            .split(area);

        // Render title bar with search indicator
        let title = if in_search_mode {
            format!("搜索: {} (按 ESC 退出搜索)", selector.get_search_query())
        } else {
            "选择 calibre 图书馆".to_string()
        };
        let title_widget = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(title_widget, chunks[0]);

        // Render library list (using filtered libraries if in search mode)
        let libraries = if in_search_mode {
            selector.get_filtered_libraries()
        } else {
            selector.get_libraries()
        };

        let items: Vec<ListItem> = libraries
            .iter()
            .enumerate()
            .map(|(i, lib)| {
                let style = if i == selected_index {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };

                let book_count = lib.book_count.unwrap_or(0);
                let mut content = if lib.from_history {
                    format!("⭐ {} - {} ({} 本书)",
                        lib.name,
                        lib.path.display(),
                        book_count
                    )
                } else {
                    format!("{} - {} ({} 本书)",
                        lib.name,
                        lib.path.display(),
                        book_count
                    )
                };

                // Add last used info for history libraries
                if let Some(last_used) = &lib.last_used {
                    content.push_str(&format!(" [上次使用: {}]", last_used));
                }

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("发现的图书馆"));

        let mut list_state = ListState::default();
        list_state.select(Some(selected_index));

        frame.render_stateful_widget(list, chunks[1], &mut list_state);

        // Render status bar with search controls
        let help_text = if in_search_mode {
            "输入搜索 | ↑↓/j/k 导航 | Enter 选择 | ESC 退出搜索 | q 退出"
        } else {
            "↑↓/j/k 导航 | Enter 选择 | / 搜索 | q 退出 | ⭐ = 历史记录中的库"
        };
        let status_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(status_widget, chunks[2]);
    }

    /// Run the main application loop
    /// Returns Some(new_library_path) if user wants to switch libraries, None if normal exit
    pub async fn run(&mut self, app: &mut App, database: &Database) -> Result<Option<PathBuf>> {
        // Initialize terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main event loop
        loop {
            // Check if we need to switch to library selection
            if app.mode == AppMode::LibrarySelection {
                // Cleanup terminal
                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;
                return Ok(Some(PathBuf::new())); // Signal to show library selector
            }

            // Render UI
            terminal.draw(|f| {
                self.render(f, app);
            })?;

            // Handle events
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match self.handle_key_event(key, app, database).await? {
                        Some(new_path) => {
                            // Cleanup terminal
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;
                            return Ok(Some(new_path));
                        },
                        None => {
                            // Continue or handle exit
                        }
                    }
                }
            }
        }

        // Cleanup terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(None)
    }

    /// Main render function
    fn render(&mut self, frame: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title bar
                Constraint::Min(0),      // Main content
                Constraint::Length(3),  // Status bar
            ])
            .split(frame.size());

        // Render title bar
        self.components.render_title_bar(frame, chunks[0], app);

        // Render main content
        match app.mode {
            AppMode::Normal | AppMode::Search => {
                self.components.render_book_list(frame, chunks[1], app);
            }
            AppMode::Details | AppMode::DetailsFromSearch => {
                self.components.render_book_details(frame, chunks[1], app);
            }
            AppMode::LibrarySelection => {
                // This should not happen in the main app, but just in case
                self.components.render_no_libraries(frame, chunks[1]);
            }
        }

        // Render status bar
        self.components.render_status_bar(frame, chunks[2], app);
    }

    /// Handle keyboard events
    /// Returns Some(new_library_path) if switching libraries, None for continue/exit
    async fn handle_key_event(&mut self, key: KeyEvent, app: &mut App, database: &Database) -> Result<Option<PathBuf>> {
        match app.mode {
            AppMode::Normal => {
                let continue_running = self.handle_normal_mode(key, app).await?;
                Ok(if continue_running && app.mode == AppMode::LibrarySelection {
                    // User wants to switch libraries
                    Some(PathBuf::new()) // Signal to show library selector
                } else {
                    None
                })
            },
            AppMode::Search => {
                let continue_running = self.handle_search_mode(key, app, database).await;
                Ok(if continue_running { None } else { Some(PathBuf::new()) })
            },
            AppMode::Details | AppMode::DetailsFromSearch => {
                let continue_running = self.handle_details_mode(key, app).await;
                Ok(if continue_running { None } else { Some(PathBuf::new()) })
            },
            AppMode::LibrarySelection => {
                // This shouldn't happen in the main app loop
                Ok(None)
            },
        }
    }

    async fn handle_normal_mode(&mut self, key: KeyEvent, app: &mut App) -> Result<bool> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.select_previous();
                Ok(true)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.select_next();
                Ok(true)
            }
            KeyCode::Enter | KeyCode::Right => {
                app.mode = AppMode::Details;
                Ok(true)
            }
            KeyCode::Char('/') => {
                app.mode = AppMode::Search;
                app.search_query.clear();
                Ok(true)
            }
            KeyCode::Esc | KeyCode::Left => {
                // Return to library selection
                app.mode = AppMode::LibrarySelection;
                Ok(true)
            }
            KeyCode::Char('q') => Ok(false), // Exit application
            _ => Ok(true),  // Ignore all other keys but don't exit
        }
    }

    async fn handle_search_mode(&mut self, key: KeyEvent, app: &mut App, database: &Database) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Left => {
                // Clear search, show all books, and exit search mode
                app.search_query.clear();
                app.books = app.all_books.clone();
                app.selected_book_index = 0;
                app.mode = AppMode::Normal;
                true
            }
            KeyCode::Enter | KeyCode::Right => {
                // Accept search and go directly to details view from search mode
                if !app.books.is_empty() {
                    app.mode = AppMode::DetailsFromSearch;
                } else {
                    app.mode = AppMode::Search;
                }
                true
            }
            KeyCode::Char(c) => {
                // Handle Ctrl+j and Ctrl+k for navigation
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if c == 'j' {
                        app.select_next();
                    } else if c == 'k' {
                        app.select_previous();
                    }
                } else {
                    app.search_query.push(c);
                    // Trigger real-time search
                    self.perform_realtime_search(app, database).await;
                }
                true
            }
            KeyCode::Backspace => {
                app.search_query.pop();
                // Trigger real-time search
                self.perform_realtime_search(app, database).await;
                true
            }
            KeyCode::Up => {
                app.select_previous();
                true
            }
            KeyCode::Down => {
                app.select_next();
                true
            }
            _ => true,  // Ignore other keys but don't exit
        }
    }

    /// Perform real-time search and update the book list
    async fn perform_realtime_search(&self, app: &mut App, database: &Database) {
        if app.search_query.is_empty() {
            // If search query is empty, show all books
            app.books = app.all_books.clone();
            app.selected_book_index = 0;
            return;
        }

        match database.search_books(&app.search_query).await {
            Ok(search_results) => {
                app.books = search_results;
                // Reset selection to first result
                app.selected_book_index = 0;
            }
            Err(_) => {
                // In real-time mode, we don't want to spam error messages
                // Just continue with current results if search fails
            }
        }
    }

    async fn handle_details_mode(&mut self, key: KeyEvent, app: &mut App) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Left => {
                // Return to search mode if we came from search, otherwise normal mode
                if app.mode == AppMode::DetailsFromSearch {
                    app.mode = AppMode::Search;
                } else {
                    app.mode = AppMode::Normal;
                }
                true
            }
            KeyCode::Enter | KeyCode::Right => {
                if let Some(book) = app.get_selected_book() {
                    self.open_book_file(&book, &app.library_path).await;
                }
                true
            }
            KeyCode::Char('q') => false, // Exit application
            _ => true,  // Ignore other keys but don't exit
        }
    }

    /// Open the book file using the system default application
    async fn open_book_file(&self, book: &Book, library_path: &PathBuf) {
        use std::process::Command;

        // Skip if we don't have file information
        if book.filename.is_empty() || book.format.is_empty() {
            eprintln!("❌ No file information available for book: {}", book.title);
            return;
        }

        // Construct the full path to the book file
        // calibre structure: library_path/book_folder/filename.format
        let book_filename = format!("{}.{}", book.filename, book.format.to_lowercase());
        let book_path = library_path.join(&book.path).join(&book_filename);

        if !book_path.exists() {
            eprintln!("❌ Book file not found: {}", book_path.display());
            return;
        }

        let result = if cfg!(target_os = "linux") {
            Command::new("xdg-open")
                .arg(book_path.to_str().unwrap_or(""))
                .spawn()
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg(book_path.to_str().unwrap_or(""))
                .spawn()
        } else if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg(&format!("/c start \"\" \"{}\"", book_path.display()))
                .spawn()
        } else {
            eprintln!("❌ Unsupported operating system for opening files");
            return;
        };

        match result {
            Ok(_) => {
                // Book opened successfully - silent operation
            }
            Err(e) => {
                eprintln!("❌ Failed to open book file: {}", e);
                eprintln!("💡 File path: {}", book_path.display());
            }
        }
    }
}