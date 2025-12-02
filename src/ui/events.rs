use crossterm::event::{KeyCode, KeyEvent};

/// Event handling utilities for the TUI
pub struct EventHandler;

impl EventHandler {
    /// Check if a key event should be handled
    pub fn should_handle_key(key: KeyEvent) -> bool {
        matches!(
            key.code,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right |
            KeyCode::Enter | KeyCode::Esc | KeyCode::Backspace |
            KeyCode::Char(_) | KeyCode::Tab | KeyCode::BackTab
        )
    }

    /// Check if key is navigation
    pub fn is_navigation_key(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right)
    }

    /// Check if key is action
    pub fn is_action_key(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('\r'))
    }

    /// Check if key is quit
    pub fn is_quit_key(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('q') | KeyCode::Char('Q'))
    }

    /// Check if key is search mode trigger
    pub fn is_search_trigger(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('/'))
    }

    /// Check if key is back/escape
    pub fn is_back_key(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left)
    }

    /// Get character from key event
    pub fn get_char(key: KeyEvent) -> Option<char> {
        match key.code {
            KeyCode::Char(c) => Some(c),
            _ => None,
        }
    }
}