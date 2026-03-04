#[derive(Debug, PartialEq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Unknown,
}

pub fn parse_command(input: &str) -> Command {

}
