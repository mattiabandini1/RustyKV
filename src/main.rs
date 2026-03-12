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

        if clear_input == "QUIT" {
            println!("Database shutdown in progress...");
            break;
        }

        if clear_input.is_empty() {
            continue;
        }

        match parse_command(clear_input) {
            Command::Set(key, value) => {
                db.set(key, value);
                println!("key and value setted!");
            },
            Command::Get(key) => {
                let result = db.get(&key);
                println!("{:?}", result);
            },
            Command::Unknown => {
                println!("Error! Command not valid.");
            }
        }
    }
}
