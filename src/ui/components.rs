use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{App, AppMode};
use crate::ui::selector::{LibrarySelector, LibraryInfo};

/// UI component renderer
pub struct UIComponents;

impl UIComponents {
    pub fn new() -> Self {
        UIComponents
    }

    /// Render title bar
    pub fn render_title_bar(&self, frame: &mut Frame, area: Rect, app: &App) {
        let title = if app.mode == AppMode::Search {
            format!("Search: {}", app.search_query)
        } else {
            format!("tuilibre - {} books", app.books.len())
        };

        let title_widget = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(title_widget, area);
    }

    /// Render book list
    pub fn render_book_list(&mut self, frame: &mut Frame, area: Rect, app: &App) {
        let items: Vec<ListItem> = app.books
            .iter()
            .enumerate()
            .map(|(i, book)| {
                let style = if i == app.selected_book_index {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };

                let path_display = if book.path.chars().count() > 30 {
                    let chars: Vec<char> = book.path.chars().collect();
                    format!("...{}", chars.iter().skip(chars.len().saturating_sub(27)).collect::<String>())
                } else {
                    book.path.clone()
                };

                let content = format!("{} - {} [{}]",
                    book.display_title(),
                    book.author_list(),
                    path_display
                );

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Books"));

        let mut list_state = ListState::default();
        list_state.select(Some(app.selected_book_index));

        frame.render_stateful_widget(list, area, &mut list_state);
    }

    /// Render book details
    pub fn render_book_details(&self, frame: &mut Frame, area: Rect, app: &App) {
        if let Some(book) = app.get_selected_book() {
            let mut details = vec![
                Line::from(vec![
                    Span::styled("Title: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&book.title),
                ]),
                Line::from(vec![
                    Span::styled("Authors: ", Style::default().fg(Color::Yellow)),
                    Span::raw(book.author_list()),
                ]),
            ];

            // Add tags if available
            if !book.tags.is_empty() {
                details.push(Line::from(vec![
                    Span::styled("Tags: ", Style::default().fg(Color::Yellow)),
                    Span::raw(book.tag_list()),
                ]));
            }

            details.extend(vec![
                Line::from(vec![
                    Span::styled("Path: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&book.path),
                ]),
                Line::from(vec![
                    Span::styled("Cover: ", Style::default().fg(Color::Yellow)),
                    Span::raw(if book.has_cover { "Yes" } else { "No" }),
                ]),
                Line::from(vec![
                    Span::styled("Added: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&book.timestamp),
                ]),
            ]);

            let details_widget = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title("Book Details"));

            frame.render_widget(details_widget, area);
        }
    }

    /// Render status bar
    pub fn render_status_bar(&self, frame: &mut Frame, area: Rect, app: &App) {
        let help_text = match app.mode {
            AppMode::Normal => "↑↓ Navigate | Enter Details | / Search | ESC Library | q Quit",
            AppMode::Search => "ESC Back | Enter Select | q Quit",
            AppMode::Details => "ESC Back | Enter Open | q Quit",
            AppMode::DetailsFromSearch => "ESC Back to Search | Enter Open | q Quit",
            AppMode::LibrarySelection => "↑↓ Select | Enter Open | q Quit",
        };

        let status_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(status_widget, area);
    }

    /// Render library selection screen
    pub fn render_library_selection(&self, frame: &mut Frame, area: Rect, selector: &LibrarySelector, selected_index: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title bar
                Constraint::Min(0),      // Library list
                Constraint::Length(3),  // Status bar
            ])
            .split(area);

        // Render title bar with search query
        let title = if selector.get_search_query().is_empty() {
            "选择 calibre 图书馆".to_string()
        } else {
            format!("搜索: {}", selector.get_search_query())
        };
        let title_widget = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(title_widget, chunks[0]);

        // Render library list
        let items: Vec<ListItem> = selector.get_filtered_libraries()
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

        // Render status bar
        let help_text = "↑↓ 选择 | Enter 确认 | q 退出 | ⭐ = 历史记录中的库";
        let status_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(status_widget, chunks[2]);
    }

    /// Render no libraries found message
    pub fn render_no_libraries(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title bar
                Constraint::Min(0),      // Message
                Constraint::Length(3),  // Status bar
            ])
            .split(area);

        // Render title bar
        let title = "未找到 calibre 图书馆";
        let title_widget = Paragraph::new(title)
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(title_widget, chunks[0]);

        // Render message
        let message = vec![
            Line::from("❌ 未在任何常见位置找到 calibre 图书馆"),
            Line::from(""),
            Line::from("💡 请手动指定图书馆路径："),
            Line::from("   tuilibre /path/to/your/calibre/library"),
            Line::from(""),
            Line::from("🔍 搜索位置："),
            Line::from("   /home"),
            Line::from("   /Users"),
            Line::from("   /win/cloud/hecloud/library"),
            Line::from("   当前目录"),
        ];

        let message_widget = Paragraph::new(message)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(message_widget, chunks[1]);

        // Render status bar
        let help_text = "按任意键退出";
        let status_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(status_widget, chunks[2]);
    }
}