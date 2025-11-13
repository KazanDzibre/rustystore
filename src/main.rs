mod store;

use arboard::Clipboard;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use store::{KvStore, Storage};

fn get_store_file() -> PathBuf {
    let mut path = dirs::data_dir().expect("Could not find data directory"); // $XDG_DATA_HOME or ~/.local/share
    path.push("rustystore");
    fs::create_dir_all(&path).unwrap();
    path.push("kvstore.json");
    path
}

fn main() {
    let store_file = get_store_file();

    let mut kv = if store_file.exists() {
        KvStore::load(store_file.to_str().unwrap()).unwrap_or_else(|_| KvStore::new())
    } else {
        KvStore::new()
    };

    let mut clipboard = Clipboard::new().unwrap();
    let stdin = io::stdin();

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
                if let Err(e) = kv.save(store_file.to_str().unwrap()) {
                    println!("Error saving: {}", e);
                }
            }
            "get" if parts.len() == 2 => {
                if let Some(val) = kv.get(parts[1]) {
                    println!("{}", val);

                    // Copy to clipboard
                    clipboard.set_text(val.clone()).unwrap();

                    println!("Password copied to clipboard!");
                } else {
                    println!("Key not found");
                }
            }
            "remove" if parts.len() == 2 => {
                if kv.remove(parts[1]) {
                    if let Err(e) = kv.save(store_file.to_str().unwrap()) {
                        println!("Error saving: {}", e);
                    }
                    println!("Deleted");
                } else {
                    println!("Key not found");
                }
            }
            "q" => break,
            _ => println!("Unknown command!"),
        }
    }
}
