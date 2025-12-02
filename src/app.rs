use std::path::PathBuf;

/// Application state following the MVP architecture
#[derive(Debug, Clone)]
pub struct App {
    pub books: Vec<Book>,
    pub all_books: Vec<Book>, // Store all books for search recovery
    pub selected_book_index: usize,
    pub search_query: String,
    pub mode: AppMode,
    pub library_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,      // Normal browsing mode
    Search,      // Search mode
    Details,     // Details view mode
    DetailsFromSearch, // Details view accessed from search mode
    LibrarySelection, // Library selection mode
}

impl App {
    pub fn new(library_path: PathBuf) -> Self {
        App {
            books: Vec::new(),
            all_books: Vec::new(),
            selected_book_index: 0,
            search_query: String::new(),
            mode: AppMode::Normal,
            library_path,
        }
    }

    pub fn get_selected_book(&self) -> Option<&Book> {
        self.books.get(self.selected_book_index)
    }

    pub fn select_next(&mut self) {
        if self.selected_book_index < self.books.len().saturating_sub(1) {
            self.selected_book_index += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_book_index > 0 {
            self.selected_book_index -= 1;
        }
    }

    pub fn set_books(&mut self, books: Vec<Book>) {
        self.selected_book_index = 0;
        self.books = books;
    }
}

// Simplified book model for MVP
#[derive(Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub authors: Vec<String>,
    pub path: String,
    pub has_cover: bool,
    pub timestamp: String,
    pub format: String,
    pub filename: String,
    pub tags: Vec<String>,
}

impl Book {
    pub fn author_list(&self) -> String {
        self.authors.join(", ")
    }

    pub fn tag_list(&self) -> String {
        self.tags.join(", ")
    }

    pub fn display_title(&self) -> String {
        if self.title.chars().count() > 50 {
            let chars: Vec<char> = self.title.chars().collect();
            format!("{}...", chars.iter().take(47).collect::<String>())
        } else {
            self.title.clone()
        }
    }
}