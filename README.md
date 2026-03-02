# Folder Creator

A simple GUI application for creating folder structures with persistent settings.

## What does this program do?

Folder Creator is a graphical application that enables:

- **Creating folder structures**: Automatically creates a main folder with a `drive_c` subdirectory
- **Custom base paths**: Choose any base path where folders should be created
- **Persistent settings**: The selected base path is saved and automatically loaded on the next startup

## Prerequisites

- [Rust](https://rust-lang.org/tools/install) (with Cargo)

## Compilation

1. Clone the repository:
   ```bash
   git clone https://github.com/x9cf9w73cv7y/folder-creator.git
   cd folder-creator
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The executable can be found at:
   - **Linux/macOS**: `target/release/folder_creator`
   - **Windows**: `target/release/folder_creator.exe`

## Usage

### Direct Execution

```bash
cargo run
```

### As Compiled Application

After compilation, you can run the application directly:

**Linux/macOS:**
```bash
./target/release/folder_creator
```

**Windows:**
```cmd
target\release\folder_creator.exe
```

## Instructions

1. **Set Base Path** (optional):
   - Click on "⚙️ Options" to expand the section
   - Enter the desired base path or click 📁 to select a folder
   - The path is automatically saved

2. **Enter Folder Name**:
   - Enter a name for the new folder in the main section

3. **Create Folder**:
   - Click "📁 Create Folder"
   - The application creates:
     - `[Base Path]/[Folder Name]/`
     - `[Base Path]/[Folder Name]/drive_c/`

## Configuration

Settings are stored in the following directories:

- **Linux**: `~/.config/folder_creator/config.json`
- **Windows**: `%APPDATA%/folder_creator/config.json`
- **macOS**: `~/Library/Application Support/folder_creator/config.json`

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

See [LICENSE](LICENSE) for full details.

Copyright (C) 2026

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
