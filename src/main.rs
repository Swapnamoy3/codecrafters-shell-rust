#[allow(unused_imports)]
mod os_helpers;
mod tokens;

use crate::tokens::*;

mod cmd;
use crate::cmd::*;

use std::io::{self, Write};




fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}


fn parse_command(command: String) -> COMMAND{

    let start = command.split_whitespace().collect::<Vec<&str>>()[0];

    if start.len() == 0 {return COMMAND::NONE(command);};


    match start {
        "exit" => return COMMAND::EXIT,
        "echo" => {
            let rest = if command.len() > 4 {command[5..].to_string()} else {"".to_string()};
            return COMMAND::ECHO(rest);
        }
        "type" => {
            let rest = if command.len() > 4 {command[5..].to_string()} else {"".to_string()};
            return COMMAND::TYPE(rest);
        },
        "pwd" => return COMMAND::PWD,
        "cd" => {
            let rest = if command.len() > 3 {command[3..].to_string()} else {"".to_string()};
            return COMMAND::CD(rest);
        }
        _ => {

            let words: Vec<&str> = command.split_whitespace().collect();
            if words.len() == 0 {return COMMAND::NONE(command);};
        
            let res = process_command(COMMAND::TYPE(words[0].to_string()));
        
            let program = words[0].to_string();
            let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
            
        
            
            let res = match res {
                RESULT::SUCCESS(_mag) =>{
                    COMMAND::CUSTOM(program, args)
                },
                _ => {
                    COMMAND::NONE(command)
                }
            };

            return res;
        }
    }

}





fn process_command(command: COMMAND) -> RESULT{
    match command{
        COMMAND::ECHO(rest) => RESULT::SUCCESS(Some(rest)),
        COMMAND::TYPE(rest) => cmd_type(rest),
        COMMAND::EXIT => RESULT::SUCCESS(None),
        COMMAND::NONE(command) => RESULT::ERROR(format!("{}: command not found", command)),
        COMMAND::CUSTOM(program, args) => RESULT::RUN(program, args),
        COMMAND::PWD => cmd_pwd(),
        COMMAND::CD(path) => cmd_cd(path)
    }
}


fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = input_command();

        let command = parse_command(command);

        if command == COMMAND::EXIT {
            break;
        }
        
        let res = process_command(command);

        
        let res = match res{
            RESULT::RUN(program, args) => cmd_custom_command(program, args),
            _ => {res}
        };


        

        match res{
            RESULT::SUCCESS(Some(msg)) => println!("{}", msg),
            RESULT::ERROR(msg) => println!("{}", msg),
            _ => {}
        }


    }


    // get_path();
}
