# File Management CLI

A robust, cross-platform file management utility built in Rust using `clap` (v4).

## Features

- **Rename**: Safely rename files and directories.
- **Create**: Create new files, optionally within a specified directory.
- **Delete**: Recursively delete files or directories.
- **Move**: Move files with automatic fallback for cross-device operations.
- **Copy**: Copy files directly or search for a specific file within a source directory.
- **Properties**: View detailed metadata, including file format, kind, and size.
- **CreateDirectory**: Create new directories (supports recursive creation).

## Installation

1. Ensure you have [Rust](https://rustup.rs/) installed.
2. Clone the repository and build:
   ```bash
   cargo build --release
   ```
3. Use the binary:
   ```bash
   ./target/release/filemanager --help
   ```

## Usage

### Examples

- **Copy a file**:
  ```bash
  filemanager copy --source-path ./src --destination-path ./backup --file-name main.rs
  ```
- **Create a directory**:
  ```bash
  filemanager create-directory my_new_folder
  ```
- **View file properties**:
  ```bash
  filemanager properties README.md
  ```
- **Rename a file**:
  ```bash
  filemanager rename old.txt new.txt
  ```

## Development

This project emphasizes robust error handling, eliminating panics, and providing clear CLI feedback. It leverages:
- `clap`: For powerful command-line argument parsing.
- `walkdir`: For efficient directory traversal.
- `file-format`: For accurate file type identification.

## License
MIT
