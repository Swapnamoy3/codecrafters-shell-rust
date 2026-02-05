#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();
    println!("{}: command not found", buffer);
}
