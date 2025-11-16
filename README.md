# Rustystore

Rustystore is a secure, minimalistic CLI key-value store written in Rust. It’s designed for fast retrieval of secrets (passwords, tokens, etc.) and integrates cleanly with Rofi for instant, hotkey-based access.

Rofi integration is the recommended way to use Rustystore, but the tool works fully without it.

# Features

Rofi integration for fast key search and clipboard copy

Add, get, and remove key-value pairs

Automatic clipboard copying on get

JSON-based persistent storage

Cross-platform data directories (dirs crate)

Lightweight, dependency-minimal CLI

# Installation

## Clone and build:

git clone https://github.com/KazanDzibre/rustystore.git
cd rustystore
cargo build --release

Optional: install globally:

sudo ln -s $(pwd)/target/release/rustystore /usr/local/bin/rustystore

CLI Usage (No Rofi)

## Run:

rustystore

## Commands:

Command Description
add <key> <value> Add a new key-value pair
get <key> Get value and copy it to clipboard
rm <key> Remove a key
list List all keys
q Quit

Example (generic key):

add sample_key myvalue
get sample_key

# Rofi Integration (Recommended)

Rofi provides a fast popup interface for selecting stored keys and copying their values.

1. Install Rofi

Debian/Ubuntu:

sudo apt install rofi

Arch:

sudo pacman -S rofi

2. Create the Rofi helper script

Create:

~/.local/bin/rustystore-rofi.sh

Contents:

#!/usr/bin/env bash

# Get list of keys

keys=$(rustystore list)

# Show menu

selected_key=$(echo "$keys" | rofi -dmenu -p "Select key:")

# Copy the key's value

if [ -n "$selected_key" ]; then
rustystore get "$selected_key"
    notify-send "Rustystore" "Value for '$selected_key' copied to clipboard"
fi

Make executable:

chmod +x ~/.local/bin/rustystore-rofi.sh

3. Bind it to a hotkey (Example: i3 Window Manager)

This is an example configuration for i3. Other window managers (Sway, bspwm, Openbox, KDE, etc.) will use different keybinding systems.

Add to ~/.config/i3/config:

bindsym $mod+Shift+p exec --no-startup-id ~/.local/bin/rustystore-rofi.sh

Reload i3:

Mod+Shift+R

Press the hotkey → select key in Rofi → value copied to clipboard.

Storage Location

Rustystore stores data in the OS-specific user data directory:

Linux: ~/.local/share/rustystore/kvstore.json

macOS: ~/Library/Application Support/rustystore/kvstore.json

Windows: %APPDATA%\rustystore\kvstore.json

Each user gets their own store. No root access required.

Dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
arboard = "3"
dirs = "5"

License

This project is licensed under GNU GPL v3.0.
See the LICENSE file for details.
