use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

mod app;
mod database;
mod ui;
mod utils;
mod history;

use app::App;
use database::Database;
use ui::UI;
use history::LibraryHistory;

#[derive(Parser)]
#[command(name = "tuilibre")]
#[command(about = "A TUI tool for managing calibre digital libraries")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Path to the calibre library directory (contains metadata.db)
    /// Can be provided as: tuilibre /path/to/library OR tuilibre --library /path/to/library
    #[arg(short, long, default_value = ".")]
    library: PathBuf,

    /// (Deprecated) Positional argument for library path - kept for compatibility
    /// Use --library or provide the path directly instead
    #[arg()]
    library_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Use positional argument if provided, otherwise use the --library argument
    let mut library_path = if args.library_path.is_some() {
        args.library_path.unwrap()
    } else {
        args.library
    };

    // Check if library path exists and has metadata.db
    let mut library_valid = library_path.exists();
    if library_valid {
        let db_path = library_path.join("metadata.db");
        library_valid = db_path.exists();
    }

    // If no valid library provided, show library selection UI
    if !library_valid {
        println!("🔍 未指定有效的 calibre 图书馆，正在搜索已知的图书馆...");

        // Initialize UI for library selection
        let mut ui = UI::new();

        if let Some(selected_path) = ui.select_library().await? {
            library_path = selected_path;
            println!("✅ 选择了图书馆: {}", library_path.display());
        } else {
            eprintln!("❌ 未选择图书馆，退出程序。");
            eprintln!("\n💡 手动指定图书馆路径:");
            eprintln!("   tuilibre /path/to/calibre/library");
            eprintln!("   tuilibre --library /path/to/calibre/library");
            eprintln!("\n🔍 搜索位置:");
            eprintln!("   当前目录");
            eprintln!("   用户主目录 ~/Documents, ~/Calibre Libraries");
            eprintln!("   系统常用目录 (Linux: /home, macOS: /Users, Windows: C:/ D:/ 等)");
            std::process::exit(1);
        }
    }

    // Double-check that the selected library is valid
    let db_path = library_path.join("metadata.db");
    if !db_path.exists() {
        eprintln!("❌ Error: No calibre database found at: {}", db_path.display());
        eprintln!("💡 Make sure the directory contains a calibre library with metadata.db");
        std::process::exit(1);
    }

    // Initialize database connection with better error handling
    let database = Database::new(&library_path)
        .await
        .with_context(|| format!("Failed to connect to calibre database at: {}", db_path.display()))?;

    // Save this library to history (for direct path usage)
    if let Err(e) = save_library_to_history(&library_path, &database).await {
        eprintln!("Warning: Failed to save library to history: {}", e);
    }

    // Load initial books
    let books = database.load_books().await
        .with_context(|| "Failed to load books from database")?;

    if books.is_empty() {
        eprintln!("⚠️  Warning: No books found in this calibre library.");
        eprintln!("💡 The database appears to be empty.");
        std::process::exit(0);
    }

    println!("📚 Loaded {} books from calibre library", books.len());

    // Initialize application state
    let all_books = books.clone();
    let mut app = App {
        books,
        all_books,
        selected_book_index: 0,
        search_query: String::new(),
        mode: app::AppMode::Normal,
        library_path,
    };

    // Initialize UI
    let mut ui = UI::new();

    // Main application loop with library switching support
    let mut database = database;
    loop {
        // Run the application with current library
        match ui.run(&mut app, &database).await? {
            Some(_) => {
                // User wants to switch libraries - show library selector
                println!("\n🔍 选择新的图书馆...");
                if let Some(new_library_path) = ui.select_library().await? {
                    println!("✅ 选择了图书馆: {}", new_library_path.display());

                    // Load the new library directly
                    println!("📚 正在加载新图书馆...");

                    // Initialize database connection for new library
                    let new_db_path = new_library_path.join("metadata.db");
                    if !new_db_path.exists() {
                        eprintln!("❌ 错误: 找不到 calibre 数据库: {}", new_db_path.display());
                        std::process::exit(1);
                    }

                    let new_database = Database::new(&new_library_path)
                        .await
                        .with_context(|| format!("Failed to connect to calibre database at: {}", new_db_path.display()))?;

                    // Save to history
                    if let Err(e) = save_library_to_history(&new_library_path, &new_database).await {
                        eprintln!("Warning: Failed to save library to history: {}", e);
                    }

                    // Load new books
                    let new_books = new_database.load_books().await
                        .with_context(|| "Failed to load books from database")?;

                    if new_books.is_empty() {
                        eprintln!("⚠️  Warning: No books found in this calibre library.");
                        std::process::exit(0);
                    }

                    println!("📚 Loaded {} books from calibre library", new_books.len());

                    // Update app state
                    let all_new_books = new_books.clone();
                    app.books = new_books;
                    app.all_books = all_new_books;
                    app.selected_book_index = 0;
                    app.search_query.clear();
                    app.mode = app::AppMode::Normal;
                    app.library_path = new_library_path.clone();

                    // Update database reference
                    database = new_database;

                    // Continue the loop with the new library
                    continue;
                } else {
                    println!("❌ 未选择图书馆，退出程序。");
                    std::process::exit(0);
                }
            },
            None => {
                // Normal exit
                break;
            }
        }
    }

    Ok(())
}

/// Save library to history
async fn save_library_to_history(library_path: &PathBuf, database: &Database) -> anyhow::Result<()> {
    let mut history = LibraryHistory::load().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load history: {}", e);
        LibraryHistory::new()
    });

    // Get library name from directory name
    let library_name = library_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string());

    // Get book count
    let books = database.load_books().await?;
    let book_count = Some(books.len() as i32);

    history.add_library(library_path, library_name, book_count);
    history.save()?;

    Ok(())
}