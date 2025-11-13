Rustystore

Rustystore is a simple, secure CLI key-value store written in Rust. It allows you to store secrets (like passwords or API tokens) and quickly retrieve them. It supports JSON-based persistence and automatically copies values to the clipboard.

Features

Add, get, and remove key-value pairs

Persistent storage in JSON format (~/.local/share/rustystore/kvstore.json)

Clipboard integration (copy secrets automatically on get)

Simple CLI interface

Cross-platform paths via dirs crate

Installation

1. Clone the repository
   git clone https://github.com/KazanDzibre/rustystore.git
   cd rustystore

2. Build the project
   cargo build --release

The executable will be in:

target/release/rustystore

3. Optional: Make it globally available

Create a symlink to /usr/local/bin:

sudo ln -s $(pwd)/target/release/rustystore /usr/local/bin/rustystore

Now you can run rustystore from anywhere.

Usage

Run the CLI:

rustystore

Commands

Add a key

> add <key> <value>

Example:

> add ghtoken token

Get a key

Automatically copies the value to the clipboard:

> get <key>

Example:

> get ghtoken
> token
> Password copied to clipboard!

Remove a key

> remove <key>

Quit

> q

Storage Location

The JSON file is stored in your data directory according to the OS:

Linux: $XDG_DATA_HOME/rustystore/kvstore.json (defaults to ~/.local/share/rustystore/kvstore.json)

macOS: ~/Library/Application Support/rustystore/kvstore.json

Windows: %APPDATA%\rustystore\kvstore.json

Dependencies

serde

- serde_json — serialization/deserialization of the JSON store

arboard
— clipboard integration

dirs
— cross-platform system paths

Add these to your Cargo.toml if not already included:

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
arboard = "3"
dirs = "5"

Notes

Each user has their own store; no root permissions are required.

Clipboard contents may require a persistent clipboard manager on Linux/X11.

Currently uses JSON; in the future, the storage backend can be swapped (trait-based design).

License

MIT License. See LICENSE file.
