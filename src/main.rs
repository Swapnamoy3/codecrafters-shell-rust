#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;


fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}


fn print_error(command: String, message: String) {
    println!("{}: {}", command, message);

}

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = input_command();
        
        print_error(command, "command not found".to_string());

    }
}
