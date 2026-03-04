mod parser;
use parser::{parse_command, Command};

fn main() {
    println!("---Rust-LSM started!---");

    let test_set = "    SET     user_3      password123     ";
    let test_get = "get     user_3";
    let test_fail = "DESTROY this db please!";

    println!("Test SET: {:?}", parse_command(test_set));
    println!("Test GET: {:?}", parse_command(test_get));
    println!("Test FAIL: {:?}", parse_command(test_fail));

    println!("---End test---")
}
