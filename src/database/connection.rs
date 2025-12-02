use anyhow::Result;
use sqlx::{SqlitePool, Row};
use std::path::Path;

use crate::app::Book;

/// Database connection manager for calibre libraries
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(library_path: &Path) -> Result<Self> {
        let db_path = library_path.join("metadata.db");
        let connection_string = format!("sqlite:{}", db_path.display());

        let pool = SqlitePool::connect(&connection_string).await?;
        Ok(Database { pool })
    }

    /// Load all books from the library (MVP simplified version)
    pub async fn load_books(&self) -> Result<Vec<Book>> {
        let rows = sqlx::query(r#"
            SELECT
                b.id,
                b.title,
                b.path,
                b.has_cover,
                b.timestamp,
                COALESCE(d.format, '') as format,
                COALESCE(d.name, '') as filename,
                GROUP_CONCAT(a.name, ', ') as authors,
                GROUP_CONCAT(t.name, ', ') as tags
            FROM books b
            LEFT JOIN books_authors_link bal ON b.id = bal.book
            LEFT JOIN authors a ON bal.author = a.id
            LEFT JOIN data d ON b.id = d.book
            LEFT JOIN books_tags_link btl ON b.id = btl.book
            LEFT JOIN tags t ON btl.tag = t.id
            GROUP BY b.id
            ORDER BY b.sort
        "#)
        .fetch_all(&self.pool)
        .await?;

        let mut books = Vec::new();
        for row in rows {
            let authors: String = row.get("authors");
            let author_list = if authors.is_empty() {
                vec!["Unknown".to_string()]
            } else {
                authors.split(", ").map(|s| s.to_string()).collect()
            };

            let tags: String = row.get("tags");
            let tag_list = if tags.is_empty() {
                vec![]
            } else {
                tags.split(", ").map(|s| s.to_string()).collect()
            };

            books.push(Book {
                id: row.get("id"),
                title: row.get("title"),
                authors: author_list,
                path: row.get("path"),
                has_cover: row.get("has_cover"),
                timestamp: row.get("timestamp"),
                format: row.get("format"),
                filename: row.get("filename"),
                tags: tag_list,
            });
        }

        Ok(books)
    }

    /// Simple search functionality
    pub async fn search_books(&self, query: &str) -> Result<Vec<Book>> {
        let search_term = format!("%{}%", query);

        let rows = sqlx::query(r#"
            SELECT
                b.id,
                b.title,
                b.path,
                b.has_cover,
                b.timestamp,
                COALESCE(d.format, '') as format,
                COALESCE(d.name, '') as filename,
                GROUP_CONCAT(a.name, ', ') as authors,
                GROUP_CONCAT(t.name, ', ') as tags
            FROM books b
            LEFT JOIN books_authors_link bal ON b.id = bal.book
            LEFT JOIN authors a ON bal.author = a.id
            LEFT JOIN data d ON b.id = d.book
            LEFT JOIN books_tags_link btl ON b.id = btl.book
            LEFT JOIN tags t ON btl.tag = t.id
            WHERE b.title LIKE ? OR a.name LIKE ? OR t.name LIKE ? OR b.path LIKE ?
            GROUP BY b.id
            ORDER BY b.sort
            LIMIT 100
        "#)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await?;

        // Convert rows to books (same logic as load_books)
        let mut books = Vec::new();
        for row in rows {
            let authors: String = row.get("authors");
            let author_list = if authors.is_empty() {
                vec!["Unknown".to_string()]
            } else {
                authors.split(", ").map(|s| s.to_string()).collect()
            };

            let tags: String = row.get("tags");
            let tag_list = if tags.is_empty() {
                vec![]
            } else {
                tags.split(", ").map(|s| s.to_string()).collect()
            };

            books.push(Book {
                id: row.get("id"),
                title: row.get("title"),
                authors: author_list,
                path: row.get("path"),
                has_cover: row.get("has_cover"),
                timestamp: row.get("timestamp"),
                format: row.get("format"),
                filename: row.get("filename"),
                tags: tag_list,
            });
        }

        Ok(books)
    }
}