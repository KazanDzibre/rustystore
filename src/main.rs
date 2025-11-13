mod logger;
mod store;

use arboard::Clipboard;
use log::{error, info, warn};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use store::{KvStore, Storage};

fn get_data_dir() -> PathBuf {
    let mut path: PathBuf = dirs::data_dir().expect("Could not find data directory");
    path.push("rustystore");
    fs::create_dir_all(&path).unwrap();
    path
}

fn get_store_file() -> PathBuf {
    let mut path = get_data_dir();
    path.push("kvstore.json");
    info!("Using store file at {:?}", path);
    path
}

fn main() {
    let mut log_path: PathBuf = get_data_dir();
    log_path.push("rustystore.log");
    logger::init_logger(&log_path); // call the new logger

    let store_file = get_store_file();

    let mut kv = if store_file.exists() {
        match KvStore::load(store_file.to_str().unwrap()) {
            Ok(store) => {
                info!("Loaded existing store from {:?}", store_file);
                store
            }
            Err(e) => {
                error!("Failed to load store: {}. Creating new one.", e);
                KvStore::new()
            }
        }
    } else {
        info!("No existing store found. Creating new one.");
        KvStore::new()
    };

    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            error!("Failed to access clipboard: {}", e);
            std::process::exit(1);
        }
    };

    let stdin = io::stdin();

    info!("Starting RustyStore CLI...");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" if parts.len() == 3 => {
                kv.add(parts[1].to_string(), parts[2].to_string());
                match kv.save(store_file.to_str().unwrap()) {
                    Ok(_) => info!("Added key '{}' and saved successfully", parts[1]),
                    Err(e) => error!("Error saving after adding '{}': {}", parts[1], e),
                }
            }
            "get" if parts.len() == 2 => match kv.get(parts[1]) {
                Some(val) => {
                    println!("{}", val);
                    if let Err(e) = clipboard.set_text(val.clone()) {
                        warn!("Failed to copy to clipboard: {}", e);
                    } else {
                        info!("Copied value of '{}' to clipboard", parts[1]);
                    }
                }
                None => warn!("Key not found: '{}'", parts[1]),
            },
            "remove" if parts.len() == 2 => {
                if kv.remove(parts[1]) {
                    match kv.save(store_file.to_str().unwrap()) {
                        Ok(_) => info!("Removed key '{}' and saved successfully", parts[1]),
                        Err(e) => error!("Error saving after removing '{}': {}", parts[1], e),
                    }
                } else {
                    warn!("Attempted to remove nonexistent key '{}'", parts[1]);
                }
            }
            "q" => {
                info!("Exiting RustyStore CLI.");
                break;
            }
            _ => warn!("Unknown command: {:?}", parts),
        }
    }
}
