use anyhow::Result;
use std::path::{Path, PathBuf};
use sqlx::SqlitePool;
use crate::history::LibraryHistory;

/// Library selection functionality
pub struct LibrarySelector {
    known_libraries: Vec<LibraryInfo>,
    history: LibraryHistory,
    search_query: String,
    filtered_libraries: Vec<LibraryInfo>,
}

#[derive(Debug, Clone)]
pub struct LibraryInfo {
    pub path: PathBuf,
    pub name: String,
    pub book_count: Option<i32>,
    pub from_history: bool,
    pub last_used: Option<String>, // Formatted last used time
}

impl LibrarySelector {
    pub fn new() -> Self {
        LibrarySelector {
            known_libraries: Vec::new(),
            history: LibraryHistory::load().unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load library history: {}", e);
                LibraryHistory::new()
            }),
            search_query: String::new(),
            filtered_libraries: Vec::new(),
        }
    }

    /// Discover calibre libraries on the system
    pub async fn discover_libraries(&mut self) -> Result<()> {
        self.known_libraries.clear();

        // First, add libraries from history (with recently used first)
        self.add_history_libraries();

        // Then discover new libraries from common locations
        let search_paths = self.get_common_search_paths();

        for search_path in search_paths {
            if search_path.exists() {
                self.search_directory(&search_path).await?;
            }
        }

        // Update filtered libraries with current search query
        self.update_filtered_libraries();

        Ok(())
    }

    /// Get common search paths for calibre libraries
    fn get_common_search_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Current directory
        paths.push(PathBuf::from("."));

        // User home directory and its Documents folder
        if let Some(home) = dirs::home_dir() {
            paths.push(home.clone());
            paths.push(home.join("Documents"));
            paths.push(home.join("Calibre Libraries"));
            paths.push(home.join("Books"));
            paths.push(home.join("Library"));
        }

        // Common system paths
        if cfg!(target_os = "linux") {
            paths.push(PathBuf::from("/home"));
            paths.push(PathBuf::from("/media"));
            paths.push(PathBuf::from("/mnt"));
        } else if cfg!(target_os = "macos") {
            paths.push(PathBuf::from("/Users"));
            paths.push(PathBuf::from("/Volumes"));
        } else if cfg!(target_os = "windows") {
            // On Windows, we'll check common drives
            for drive in ['C', 'D', 'E', 'F'] {
                let drive_path = PathBuf::from(format!("{}:/", drive));
                if drive_path.exists() {
                    paths.push(drive_path);
                }
            }
        }

        paths
    }

    /// Add libraries from history
    fn add_history_libraries(&mut self) {
        let mut existing_paths = std::collections::HashSet::new();

        for entry in self.history.get_libraries() {
            if entry.path.exists() {
                let db_path = entry.path.join("metadata.db");
                if db_path.exists() {
                    let library_info = LibraryInfo {
                        path: entry.path.clone(),
                        name: entry.name.clone()
                            .unwrap_or_else(|| {
                                entry.path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string()
                            }),
                        book_count: entry.book_count,
                        from_history: true,
                        last_used: Some(
                            entry.last_used.format("%Y-%m-%d %H:%M").to_string()
                        ),
                    };
                    self.known_libraries.push(library_info);
                    existing_paths.insert(entry.path.clone());
                }
            }
        }
    }

    /// Search a directory for calibre libraries
    async fn search_directory(&mut self, base_path: &Path) -> Result<()> {
        // Get paths already in history to avoid duplicates
        let history_paths: std::collections::HashSet<_> = self.known_libraries
            .iter()
            .filter(|lib| lib.from_history)
            .map(|lib| lib.path.canonicalize().unwrap_or_else(|_| lib.path.clone()))
            .collect();

        if let Ok(entries) = std::fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Skip if already in history
                    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());
                    if history_paths.contains(&canonical_path) {
                        continue;
                    }

                    let db_path = path.join("metadata.db");
                    if db_path.exists() {
                        let book_count = self.get_book_count(&path).await.ok();
                        let library_info = LibraryInfo {
                            path: path.clone(),
                            name: path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or(&path.display().to_string())
                                .to_string(),
                            book_count,
                            from_history: false,
                            last_used: None,
                        };
                        self.known_libraries.push(library_info);
                    }
                }
            }
        }
        Ok(())
    }

    /// Get the number of books in a library
    async fn get_book_count(&self, library_path: &Path) -> Result<i32> {
        let db_path = library_path.join("metadata.db");
        if !db_path.exists() {
            return Ok(0);
        }

        let connection_string = format!("sqlite:{}", db_path.display());
        let pool = sqlx::SqlitePool::connect(&connection_string).await?;

        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM books")
            .fetch_one(&pool)
            .await?;

        pool.close().await;
        Ok(count)
    }

    /// Get the list of discovered libraries
    pub fn get_libraries(&self) -> &[LibraryInfo] {
        &self.known_libraries
    }

    /// Get library by index
    pub fn get_library(&self, index: usize) -> Option<&LibraryInfo> {
        self.known_libraries.get(index)
    }

    /// Check if any libraries were found
    pub fn has_libraries(&self) -> bool {
        !self.known_libraries.is_empty()
    }

    /// Save library to history when user selects it
    pub async fn save_to_history(&mut self, library_path: &Path, library_name: Option<String>) -> Result<()> {
        let book_count = self.get_book_count(library_path).await.ok();
        self.history.add_library(library_path, library_name, book_count);
        self.history.save()?;
        Ok(())
    }

    /// Set search query and update filtered libraries
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query.clone();
        self.update_filtered_libraries();
    }

    /// Get current search query
    pub fn get_search_query(&self) -> &str {
        &self.search_query
    }

    /// Update filtered libraries based on search query
    fn update_filtered_libraries(&mut self) {
        if self.search_query.is_empty() {
            // If no search query, show all libraries
            self.filtered_libraries = self.known_libraries.clone();
        } else {
            // Filter libraries based on search query
            let search_term = self.search_query.to_lowercase();
            self.filtered_libraries = self.known_libraries
                .iter()
                .filter(|lib| {
                    let name_matches = lib.name.to_lowercase().contains(&search_term);
                    let path_matches = lib.path.display().to_string().to_lowercase().contains(&search_term);
                    name_matches || path_matches
                })
                .cloned()
                .collect();
        }
    }

    /// Get the filtered libraries (for display)
    pub fn get_filtered_libraries(&self) -> &[LibraryInfo] {
        &self.filtered_libraries
    }

    /// Get a filtered library by index
    pub fn get_filtered_library(&self, index: usize) -> Option<&LibraryInfo> {
        self.filtered_libraries.get(index)
    }

    /// Check if any filtered libraries exist
    pub fn has_filtered_libraries(&self) -> bool {
        !self.filtered_libraries.is_empty()
    }
}