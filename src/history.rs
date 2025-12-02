use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Library usage history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryHistory {
    libraries: Vec<LibraryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryEntry {
    pub path: PathBuf,
    pub name: Option<String>,
    pub last_used: DateTime<Utc>,
    pub use_count: u32,
    pub book_count: Option<i32>,
}

impl LibraryHistory {
    /// Create new empty history
    pub fn new() -> Self {
        LibraryHistory {
            libraries: Vec::new(),
        }
    }

    /// Get the history file path in user's home directory
    pub fn get_history_file_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find user home directory"))?;

        let config_dir = home_dir.join(".config").join("tuilibre");
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory: {}", config_dir.display()))?;

        Ok(config_dir.join("libraries.json"))
    }

    /// Load history from file
    pub fn load() -> Result<Self> {
        let history_path = Self::get_history_file_path()?;

        if history_path.exists() {
            let content = fs::read_to_string(&history_path)
                .with_context(|| format!("Failed to read history file: {}", history_path.display()))?;

            let history: LibraryHistory = serde_json::from_str(&content)
                .with_context(|| "Failed to parse history file")?;

            // Clean up duplicate entries and sort by last used
            Ok(history.clean())
        } else {
            Ok(Self::new())
        }
    }

    /// Save history to file
    pub fn save(&self) -> Result<()> {
        let history_path = Self::get_history_file_path()?;

        let content = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize history")?;

        fs::write(&history_path, content)
            .with_context(|| format!("Failed to write history file: {}", history_path.display()))?;

        Ok(())
    }

    /// Add or update a library in history
    pub fn add_library(&mut self, path: &Path, name: Option<String>, book_count: Option<i32>) {
        let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if let Some(entry) = self.libraries.iter_mut().find(|e| e.path == path) {
            // Update existing entry
            entry.last_used = Utc::now();
            entry.use_count += 1;
            if name.is_some() {
                entry.name = name;
            }
            if book_count.is_some() {
                entry.book_count = book_count;
            }
        } else {
            // Add new entry
            let entry = LibraryEntry {
                path: path.clone(),
                name,
                last_used: Utc::now(),
                use_count: 1,
                book_count,
            };
            self.libraries.push(entry);
        }

        // Clean up and sort
        *self = self.clone().clean();
    }

    /// Remove duplicate entries and sort by last used (most recent first)
    fn clean(self) -> Self {
        let mut seen = HashSet::new();
        let mut unique_libraries: Vec<_> = self.libraries
            .into_iter()
            .filter(|entry| {
                let path_str = entry.path.display().to_string();
                seen.insert(path_str)
            })
            .collect();

        // Sort by last used (most recent first), then by use count
        unique_libraries.sort_by(|a, b| {
            b.last_used.cmp(&a.last_used)
                .then_with(|| b.use_count.cmp(&a.use_count))
        });

        // Limit to reasonable number (keep last 20)
        unique_libraries.truncate(20);

        LibraryHistory {
            libraries: unique_libraries,
        }
    }

    /// Get all libraries from history
    pub fn get_libraries(&self) -> &[LibraryEntry] {
        &self.libraries
    }

    /// Get a library by index
    pub fn get_library(&self, index: usize) -> Option<&LibraryEntry> {
        self.libraries.get(index)
    }

    /// Check if any libraries are in history
    pub fn has_libraries(&self) -> bool {
        !self.libraries.is_empty()
    }

    /// Remove a library from history
    pub fn remove_library(&mut self, index: usize) -> Result<()> {
        if index < self.libraries.len() {
            self.libraries.remove(index);
            self.save()?;
        }
        Ok(())
    }
}