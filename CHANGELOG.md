# Tuilibre Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-22

### Added
- 🎯 **Library Management System**
  - Complete calibre database integration
  - Cross-platform library discovery (Linux, macOS, Windows)
  - Smart library history with usage tracking
  - Automatic library switching

- 🔍 **Advanced Search Functionality**
  - Real-time search as you type (like fzf)
  - Multi-field search: titles, authors, tags, and paths
  - Pinyin support for Chinese book searches
  - Path-based searching for organizational books

- 📚 **User Interface**
  - Clean, intuitive TUI with ratatui
  - Bilingual Chinese/English interface
  - Book list with author and path information
  - Detailed book view with comprehensive metadata
  - Tag support and display

- 🔄 **Navigation & Controls**
  - Vim-style navigation (j/k, h/l, gg, G)
  - Search mode with real-time filtering
  - Quick library switching with ESC key
  - Keyboard-driven interface
  - ESC to return to library selection from book list

- 💾 **History & Persistence**
  - JSON-based library history storage
  - Automatic library usage tracking
  - Priority display of recently used libraries
  - Cross-session library preferences

- 🖥️ **File Integration**
  - xdg-open integration for book opening
  - Cross-platform file handling
  - Automatic file path resolution
  - Support for multiple book formats

### Technical Features
- **Database**: SQLite with async sqlx
- **UI Framework**: ratatui TUI library
- **Async Runtime**: tokio for non-blocking operations
- **Error Handling**: Comprehensive anyhow-based error handling
- **Cross-Platform**: Linux, macOS, Windows support

### Search Capabilities
```
Search examples:
- zhongguo     → Chinese books about China
- programming → Books with programming in path/title
- fiction      → Fiction books
- 2023         → Books from year 2023
- tech         → Technical books
```

### Controls
```
Book List (Normal Mode):
↑↓/j/k        → Navigate books
Enter         → View book details
/             → Search mode
ESC           → Library selection
q             → Quit

Search Mode:
j/k or ↑↓     → Navigate results
Enter         → View selected book details
ESC           → Return to full list
q             → Quit

Library Selection:
↑↓/j/k     → Navigate libraries
Enter      → Select and open library
q          → Quit

Details View:
ESC        → Return to previous view
Enter      → Open book with system default
q          → Quit
```

### Configuration
- **History File**: `~/.config/tuilibre/libraries.json`
- **Supported Databases**: Any calibre library with `metadata.db`
- **Search Paths**:
  - Current directory
  - User home and Documents folders
  - System common directories
  - Configurable search locations

### Performance
- **Fast Startup**: Async database connections
- **Real-time Search**: Instant search feedback
- **Memory Efficient**: Streaming book loading
- **Responsive UI**: Non-blocking operations

### Dependencies
- ratatui 0.24 - Terminal UI framework
- sqlx 0.7 - Async SQL database toolkit
- tokio 1.0 - Async runtime
- crossterm 0.27 - Terminal handling
- clap 4.0 - Command line parsing
- serde 1.0 - Serialization
- dirs 5.0 - System directories

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd tuilibre

# Build the release version
cargo build --release

# Run the application
./target/release/tuilibre
```

### Usage Examples
```bash
# Launch library selector
tuilibre

# Open specific library
tuilibre /path/to/calibre/library

# With flag syntax
tuilibre --library /path/to/calibre/library
```

### Known Limitations
- Requires calibre database (metadata.db)
- Terminal-based interface only
- No web interface
- No built-in book reader integration

### Future Roadmap
- [ ] Book reading mode integration
- [ ] Cover image display
- [ ] Advanced filtering options
- [ ] Book metadata editing
- [ ] Export/import library lists
- [ ] Plugin system for custom formats

---

## Acknowledgments

Thanks to the Rust community and the creators of the amazing libraries that make tuilibre possible:

- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI framework
- [sqlx](https://github.com/launchbadge/sqlx) for the async SQL toolkit
- [calibre](https://calibre-ebook.com/) for the incredible ebook management system

---

*For more information, visit the [project repository](https://github.com/yourusername/tuilibre).*
