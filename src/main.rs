#[allow(unused_imports)]
use std::io::{self, Write};
use std::{fmt::format, process::Command};


fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}





static COMMANDS: [&str; 4] = ["exit", "echo", "exit", "type"];

enum RESULT{
    ERROR(String),
    SUCCESS(String)
}
enum COMMAND{
    EXIT, 
    ECHO(String),
    TYPE(String),
    NONE(String)
}

fn find_command(command: String) -> RESULT{
    for cmd in COMMANDS{
        if(cmd == command){
            return RESULT::SUCCESS(format!("{} is a builtin command", command));
        }
    }

    return RESULT::ERROR(format!("{}: is not found", command));
}


fn parse_command(command: String) -> COMMAND{
    if(command.starts_with("exit")){
        return COMMAND::EXIT;
    }
    else if (command.starts_with("echo")) {
        let rest = command[5..].to_string(); 
        return COMMAND::ECHO(rest);
    }else if(command.starts_with("type")){
        let rest = command[5..].to_string();
        return COMMAND::TYPE(rest);
    }


    
    return COMMAND::NONE(command);
}


fn process_command(command: COMMAND) -> RESULT{
    match command{
        COMMAND::ECHO(rest) => RESULT::SUCCESS(format!("{}", rest)),
        COMMAND::TYPE(rest) => find_command(rest),
        COMMAND::EXIT => RESULT::SUCCESS("".to_string()),
        COMMAND::NONE(command) => RESULT::ERROR(format!("{}: command not found", command)),
    }
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = input_command();

        let command = parse_command(command);
        let res = process_command(command);

        match res{
            RESULT::SUCCESS(msg) => println!("{}", msg),
            RESULT::ERROR(msg) => println!("{}", msg),
        }


    }
}
