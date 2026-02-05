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


enum COMMANDS{
    EXIT, 
    ECHO(String),
    ERROR(String, String),
}
fn parse_command(command: String) -> COMMANDS{
    if(command.starts_with("exit")){
        return COMMANDS::EXIT;
    }
    else if (command.starts_with("echo")) {
        let rest = command[5..].to_string(); 
        return COMMANDS::ECHO(rest);
    }


    return COMMANDS::ERROR(command, "command not found".to_string());
    
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = input_command();

        match parse_command(command.clone()){
            COMMANDS::EXIT => break,
            COMMANDS::ECHO(rest) => println!("{}", rest),
            COMMANDS::ERROR(command, message) => print_error(command, message),
        };


    }
}
