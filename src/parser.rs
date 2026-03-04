#[derive(Debug, PartialEq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Unknown,
}

pub fn parse_command(input: &str) -> Command {
    let mut part = input.split_whitespace();
    let operation = part.next();

    match operation {
        None => Command::Unknown,
        Some(verb) => {
            let lowercase_verb = verb.to_lowercase();

            match lowercase_verb.as_str() {
                "get" => {
                    match part.next() {
                        Some(key) => Command::Get(key.to_string()),
                        None => Command::Unknown,
                    }
                },
                "set" => {
                    match (part.next(), part.next()) {
                        (Some(key), Some(value)) => {
                            Command::Set(key.to_string(), value.to_string())
                        },
                        _ => Command::Unknown,
                    }
                },
                _ => Command::Unknown
            }
        }
    }
}
