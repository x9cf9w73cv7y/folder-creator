# Folder Creator

Ein einfaches GUI-Programm zum Erstellen von Ordnerstrukturen mit persistierenden Einstellungen.

## Was macht dieses Programm?

Folder Creator ist eine grafische Anwendung, die es ermöglicht:

- **Ordnerstrukturen zu erstellen**: Erstellt automatisch einen Hauptordner mit einem Unterordner `drive_c`
- **Benutzerdefinierte Basispfade**: Wähle einen beliebigen Basispfad, in dem die Ordner erstellt werden sollen
- **Persistente Einstellungen**: Der gewählte Basispfad wird gespeichert und beim nächsten Start automatisch geladen

## Voraussetzungen

- [Rust](https://rust-lang.org/tools/install) (mit Cargo)

## Kompilierung

1. Repository klonen:
   ```bash
   git clone https://github.com/x9cf9w73cv7y/folder-creator.git
   cd folder-creator
   ```

2. Projekt bauen:
   ```bash
   cargo build --release
   ```

3. Die ausführbare Datei findest du unter:
   - **Linux/macOS**: `target/release/folder_creator`
   - **Windows**: `target/release/folder_creator.exe`

## Verwendung

### Direkte Ausführung

```bash
cargo run
```

### Als fertige Anwendung

Nach der Kompilierung kannst du die Anwendung direkt starten:

**Linux/macOS:**
```bash
./target/release/folder_creator
```

**Windows:**
```cmd
target\release\folder_creator.exe
```

## Anleitung

1. **Basispfad einstellen** (optional):
   - Klicke auf "⚙️ Optionen" um den Bereich zu erweitern
   - Gib den gewünschten Basispfad ein oder klicke auf 📁 um einen Ordner auszuwählen
   - Der Pfad wird automatisch gespeichert

2. **Ordnername eingeben**:
   - Gib im Hauptbereich einen Namen für den neuen Ordner ein

3. **Ordner erstellen**:
   - Klicke auf "📁 Ordner erstellen"
   - Die Anwendung erstellt:
     - `[Basispfad]/[Ordnername]/`
     - `[Basispfad]/[Ordnername]/drive_c/`

## Konfiguration

Die Einstellungen werden in folgenden Verzeichnissen gespeichert:

- **Linux**: `~/.config/folder_creator/config.json`
- **Windows**: `%APPDATA%/folder_creator/config.json`
- **macOS**: `~/Library/Application Support/folder_creator/config.json`

## Lizenz

Dieses Projekt steht unter einer Open-Source-Lizenz. Siehe [LICENSE](LICENSE) für Details (falls vorhanden).
