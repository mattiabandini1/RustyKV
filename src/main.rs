mod parser;
mod memtable;
use parser::{parse_command, Command};
use memtable::MemTable;

fn main() {

    let mut db = MemTable::new();

    let command_1 = "SET utente_99 super_segreto";
    let command_2 = "GET utente_99";

    match parse_command(command_1) {
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

    match parse_command(command_2) {
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
