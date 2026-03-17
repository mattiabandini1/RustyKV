use std::io::{self, Write};

mod parser;
mod memtable;

use parser::{parse_command, Command};
use memtable::MemTable;

fn main() {

    let mut db = MemTable::new();

    println!("---Rust-LSM started---");
    println!("Supported commands: SET <key> <value>, GET <key>, QUIT");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Errors reading keyboards");

        let clear_input = input.trim();

        if clear_input.to_uppercase() == "QUIT" {
            println!("Database shutdown in progress...");

            if db.len() > 0 {
                db.flush_to_disk();
            }

            break;
        }

        if clear_input.is_empty() {
            continue;
        }

        match parse_command(clear_input) {
            Command::Set(key, value) => {
                db.set(key, value);
                println!("key and value setted!");

                if db.len() >= 3 {
                    println!("MemTable full! Flush to disk in progress...");
                    db.flush_to_disk();
                    db = MemTable::new();

                    println!("Download completed. Memtable free!");
                }
            },
            Command::Get(key) => {
                let result = db.get(&key);
                
                match result {
                    Some(value_found) => println!("{}", value_found),

                    None => println!("Key not found!"),
                }
            },
            Command::Unknown => {
                println!("Error! Command not valid.");
            }
        }
    }
}
