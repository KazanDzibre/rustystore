#!/usr/bin/env bash

LOGFILE="$HOME/.local/share/rustystore/rustystore-rofi.log"

# Get the list of keys from RustyStore
keys=$(rustystore list)

# Use rofi to select a key
selected_key=$(echo "$keys" | rofi -dmenu -p "Select key:")

# If a key was selected
if [ -n "$selected_key" ]; then
    # Copy its value to the clipboard using RustyStore
    rustystore get "$selected_key"

    # Notify user
    notify-send "RustyStore" "Value for '$selected_key' copied to clipboard"

    # Log the action with timestamp
    echo "$(date '+%Y-%m-%d %H:%M:%S') Selected key: '$selected_key'" >> "$LOGFILE"
fi
