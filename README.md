# tuilibre

[中文文档 / Chinese Documentation](README-zh.md) | English Documentation


A TUI (Terminal User Interface) tool for managing calibre digital libraries.

## Description

tuilibre is a fast, keyboard-driven terminal application for browsing and managing calibre digital book libraries. It provides a simple yet powerful interface for searching, viewing, and organizing your digital book collection.

## Features (MVP)

- Connect to calibre databases
- Browse book list with keyboard navigation
- View detailed book information
- Simple search functionality
- Cross-platform support (Linux, macOS, Windows)

## Requirements

- Rust 1.70+
- SQLite (for calibre database access)

## Installation

### Building from source

```bash
git clone https://github.com/yourusername/tuilibre.git
cd tuilibre
cargo build --release
```

### Usage

Run tuilibre with the path to your calibre library:

```bash
# Use current directory
tuilibre

# Specify library path
tuilibre /path/to/calibre/library
```

## Controls

### Normal Mode
- `Enter` or `→`: View book details
- `Esc` or `←`: Return to library selection
- `↑/↓` or `j/k`: Navigate book list
- `/`: Enter search mode
- `q`: Quit application

### Search Mode
- `Enter` or `→`: Execute search and view results
- `Esc` or `←`: Return to normal mode (clears search)
- Character keys: Input search query
- `Backspace`: Delete last character

### Details Mode
- `Enter` or `→`: Open book with system default application
- `Esc` or `←`: Return to previous mode
- `q`: Quit application

### Library Selection Mode
- `Enter` or `→`: Select library
- `Esc` or `←`: Exit application
- `↑/↓` or `j/k`: Navigate library list
- `/`: Enter search mode for libraries



## License

This project is licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

**AI Assistance**: This software was written with the assistance of artificial intelligence. While AI tools were used to accelerate development, the final code has been reviewed, tested, and approved by human developers.

**No Warranty**: This software is provided "as-is", without warranty of any kind, either express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement.