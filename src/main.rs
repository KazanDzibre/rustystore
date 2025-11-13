mod store;

use std::io::{self, Write};
use store::{KvStore, Storage};

const STORE_FILE: &str = "kvstore.json";

fn main() {
    let mut kv = match KvStore::load(STORE_FILE) {
        Ok(store) => store,
        Err(_) => KvStore::new(),
    };
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
                if let Err(e) = kv.save(STORE_FILE) {
                    println!("Error saving: {}", e);
                }
            }
            "get" if parts.len() == 2 => {
                if let Some(val) = kv.get(parts[1]) {
                    println!("{}", val);
                } else {
                    println!("Key not found");
                }
            }
            "remove" if parts.len() == 2 => {
                if kv.remove(parts[1]) {
                    if let Err(e) = kv.save(STORE_FILE) {
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
