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

    // Clipboard access
    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            error!("Failed to access clipboard: {}", e);
            std::process::exit(1);
        }
    };

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        interactive_loop(&mut kv, &store_file, &mut clipboard);
        return; // stop after the interactive loop
    }

    match args[1].as_str() {
        "add" if args.len() == 4 => {
            kv.add(args[2].clone(), args[3].clone());
            match kv.save(store_file.to_str().unwrap()) {
                Ok(_) => info!("Added key '{}' successfully", args[2]),
                Err(e) => error!("Error saving after adding '{}': {}", args[2], e),
            }
        }
        "get" if args.len() == 3 => {
            if let Some(val) = kv.get(&args[2]) {
                println!("{}", val);
                info!("Retrieved key '{}'", args[2]);

                // Clone the string for clipboard to avoid moving
                let clipboard_val = val.clone();
                if let Err(e) = clipboard.set_text(clipboard_val) {
                    warn!("Failed to copy '{}' to clipboard: {}", args[2], e);
                } else {
                    info!("Copied '{}' to clipboard", args[2]);

                    // Give clipboard time to register the text before program exits
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            } else {
                warn!("Key not found: '{}'", args[2]);
            }
        }
        "rm" if args.len() == 3 => {
            if kv.remove(&args[2]) {
                match kv.save(store_file.to_str().unwrap()) {
                    Ok(_) => info!("Removed key '{}' successfully", args[2]),
                    Err(e) => error!("Error saving after removing '{}': {}", args[2], e),
                }
            } else {
                warn!("Attempted to remove nonexistent key '{}'", args[2]);
            }
        }
        "list" if args.len() == 2 => {
            let keys = kv.list();
            if keys.is_empty() {
                println!("");
                info!("No keys stored");
            } else {
                info!("Stored keys:");
                for key in keys {
                    println!("{}", key);
                    info!(" - {}", key);
                }
            }
        }
        _ => {
            error!("Unknown command or wrong arguments. Usage:");
            error!("  rustystore add <key> <value>");
            error!("  rustystore get <key>");
            error!("  rustystore rm <key>");
            error!("  rustystore list");
        }
    }
}

fn interactive_loop(kv: &mut KvStore, store_file: &PathBuf, clipboard: &mut Clipboard) {
    use std::io::{self, Write};

    let stdin = io::stdin();

    info!("Starting RustyStore interactive CLI...");

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
                let _ = kv.save(store_file.to_str().unwrap());
            }
            "get" if parts.len() == 2 => {
                if let Some(val) = kv.get(parts[1]) {
                    println!("{}", val);
                    let _ = clipboard.set_text(val);
                }
            }
            "rm" if parts.len() == 2 => {
                kv.remove(parts[1]);
                let _ = kv.save(store_file.to_str().unwrap());
            }
            "list" if parts.len() == 1 => {
                for key in kv.list() {
                    println!("{}", key);
                }
            }
            "q" => break,
            _ => println!("Unknown command"),
        }
    }
}
