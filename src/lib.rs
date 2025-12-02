//! tuilibre - A TUI tool for managing calibre digital libraries
//!
//! This library provides the core functionality for the tuilibre application,
//! including database access, UI components, and application state management.

pub mod app;
pub mod database;
pub mod ui;
pub mod utils;
pub mod history;

pub use app::{App, Book};
pub use database::Database;
pub use ui::UI;