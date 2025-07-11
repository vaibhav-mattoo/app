# Alman - Intelligent Alias Manager

A powerful command-line tool and TUI for managing shell aliases with intelligent suggestions based on your command history.

## Features

- 🎯 **Smart Alias Suggestions**: Get intelligent alias suggestions based on your command usage patterns
- 🖥️ **Interactive TUI**: Beautiful terminal user interface for managing aliases
- 📊 **Command Analytics**: Track command frequency and usage patterns
- ⚡ **One-Click Alias Creation**: Apply a suggestion to your shell config in one keystroke
- 🔄 **Multi-Shell Support**: Works with bash, zsh, fish, and POSIX shells
- 📁 **Multiple Alias Files**: Manage aliases across multiple files
- 🎨 **Colored Output**: Beautiful colored CLI output for better readability
- 🔄 **Automatic Alias Loading**: Aliases are automatically sourced in your shell

## Installation

### From Source

```bash
git clone <repository-url>
cd alman
cargo build --release
```

### Shell Integration

After building, integrate with your shell:

```bash
# For bash
eval "$(./target/release/alman init bash)"

# For zsh
eval "$(./target/release/alman init zsh)"

# For fish
./target/release/alman init fish | source

# For POSIX shells (ksh, dash, etc.)
eval "$(./target/release/alman init posix)"
```

**What this does:**
- Sets up command tracking for intelligent suggestions
- Automatically sources all your alias files on shell startup
- Provides `alman_source_aliases` function to reload aliases when they change

## Usage

### Command Line Interface

```bash
# Launch TUI (default)
alman

# Add an alias
alman add -c "git status" gs

# Remove an alias
alman remove gs

# List all aliases
alman list

# Get alias suggestions
alman get-suggestions -n 10

# Change an alias
alman change old-alias new-alias "git status"

# Delete suggestions for a command
alman delete-suggestion gs

# Initialize shell integration
alman init bash

# Reload aliases in current shell (after adding/removing aliases)
alman reload
```

### TUI Controls

- **Navigation**: Arrow keys or `j`/`k`
- **Selection**: Enter
- **Search**: Type to filter commands
- **Add Alias**: `a`
- **Remove Alias**: `r`
- **List Aliases**: `l`
- **Quit**: `q` or `Ctrl+C`

### Working with Aliases

After adding or removing aliases, you can reload them in your current shell:

```bash
# Method 1: Use alman reload command
eval "$(alman reload)"

# Method 2: Call the function directly
alman_source_aliases
```

## Configuration

Alman stores its data in `~/.alman/`:

- `~/.alman/command_database.json` - Command history and analytics
- `~/.alman/deleted_commands.json` - Commands to ignore
- `~/.alman/config.json` - Application configuration
- `~/.alman/aliases` - Default alias file

### Multiple Alias Files

You can manage aliases across multiple files:

```bash
# Add a new alias file to the configuration
alman --alias-file-path ~/.my-aliases add -c "git status" gs

# This will:
# 1. Add ~/.my-aliases to the config
# 2. Make it the default alias file
# 3. Source it automatically in your shell
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---
