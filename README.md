Rustystore

Rustystore is a simple, secure CLI key-value store written in Rust, designed for fast access to secrets like passwords or API tokens. Its main power comes from Rofi integration, allowing you to quickly search and copy keys to your clipboard with a single hotkey.

Features

Rofi integration for instant access to stored keys

Add, get, and remove key-value pairs

Persistent storage in JSON format (~/.local/share/rustystore/kvstore.json)

Clipboard integration (automatically copies values on get)

Simple CLI interface

Cross-platform paths via the dirs crate

Installation

Clone the repository:

git clone https://github.com/KazanDzibre/rustystore.git
cd rustystore

Build the project:

cargo build --release

The executable will be in:

target/release/rustystore

Optional: Make it globally available

sudo ln -s $(pwd)/target/release/rustystore /usr/local/bin/rustystore

CLI Usage (Without Rofi)

Run the interactive CLI:

rustystore

Commands:

Add a key: add <key> <value>

Get a key: get <key> (copies value to clipboard)

Remove a key: rm <key>

List all keys: list

Quit: q

Rofi Integration (Recommended)

Rofi allows you to select a key from a popup menu and automatically copy its value to your clipboard.

1. Install Rofi

On Ubuntu/Debian:

sudo apt install rofi

On Arch Linux:

sudo pacman -S rofi

2. Create the helper script

For example: ~/.local/bin/rustystore-rofi.sh

#!/usr/bin/env bash

# Get list of keys

keys=$(rustystore list)

# Show Rofi menu

selected_key=$(echo "$keys" | rofi -dmenu -p "Select key:")

# Copy selected key's value

if [ -n "$selected_key" ]; then
rustystore get "$selected_key"
    notify-send "RustyStore" "Value for '$selected_key' copied to clipboard"
fi

Make it executable:

chmod +x ~/.local/bin/rustystore-rofi.sh

3. Bind to a hotkey in your window manager

Example for i3 config (~/.config/i3/config):

# Open RustyStore Rofi menu

bindsym $mod+Shift+p exec --no-startup-id ~/.local/bin/rustystore-rofi.sh

Reload i3 configuration (Mod+Shift+R) and press your hotkey to bring up RustyStore via Rofi.

Storage Location

Linux: ~/.local/share/rustystore/kvstore.json

macOS: ~/Library/Application Support/rustystore/kvstore.json

Windows: %APPDATA%\rustystore\kvstore.json

Dependencies

serde, serde_json — JSON serialization/deserialization

arboard — clipboard integration

dirs — cross-platform system paths

License

Rustystore is licensed under GNU GPL v3.0. See LICENSE for details.
