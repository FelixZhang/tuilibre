use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Block,
};

/// Layout management utilities for the TUI
pub struct LayoutManager;

impl LayoutManager {
    /// Create the main application layout
    pub fn create_main_layout(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title bar
                Constraint::Min(0),      // Main content
                Constraint::Length(3),  // Status bar
            ])
            .split(area)
            .to_vec()
    }

    /// Create content layout for book list view
    pub fn create_book_list_layout(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(100),  // Book list (full width for MVP)
            ])
            .split(area)
            .to_vec()
    }

    /// Create content layout for book details view
    pub fn create_book_details_layout(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),  // Details (full height for MVP)
            ])
            .split(area)
            .to_vec()
    }

    /// Create a standard block with borders
    pub fn create_bordered_block(title: &str) -> Block<'_> {
        Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(title)
    }
}