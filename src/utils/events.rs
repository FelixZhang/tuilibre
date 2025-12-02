use crossterm::event::{Event, KeyEvent, KeyCode};

/// Utility functions for event handling
pub struct EventUtils;

impl EventUtils {
    /// Convert key event to string representation for debugging
    pub fn key_to_string(key: &KeyEvent) -> String {
        match key.code {
            KeyCode::Char(c) => format!("Char('{}')", c),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Escape".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::BackTab => "BackTab".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            KeyCode::Null => "Null".to_string(),
            KeyCode::CapsLock => "CapsLock".to_string(),
            KeyCode::ScrollLock => "ScrollLock".to_string(),
            KeyCode::NumLock => "NumLock".to_string(),
            KeyCode::PrintScreen => "PrintScreen".to_string(),
            KeyCode::Pause => "Pause".to_string(),
            KeyCode::Menu => "Menu".to_string(),
            KeyCode::KeypadBegin => "KeypadBegin".to_string(),
            KeyCode::Media(_) => "Media".to_string(),
            KeyCode::Modifier(_) => "Modifier".to_string(),
            KeyCode::Insert => "Insert".to_string(),
        }
    }

    /// Check if event is a key event
    pub fn is_key_event(event: &Event) -> bool {
        matches!(event, Event::Key(_))
    }

    /// Extract key event from event
    pub fn get_key_event(event: &Event) -> Option<&KeyEvent> {
        match event {
            Event::Key(key) => Some(key),
            _ => None,
        }
    }
}