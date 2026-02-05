#[allow(unused_imports)]
mod os_helpers;
use crate::os_helpers::*;

mod tokens;
use crate::tokens::*;

use std::io::{self, Write};
use std::fs;
use std::process::Command;
use std::env;


fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}


fn find_command(command: String) -> RESULT{
    for cmd in COMMANDS{
        if cmd == command {
            return RESULT::SUCCESS(format!("{} is a shell builtin", command));
        }
    }


    let paths = get_path();
    for path in paths{
        let desired_file_path = path.join(&command);
        if fs::exists(&desired_file_path).unwrap() {

            if is_executable(&fs::metadata(&desired_file_path).unwrap()) {
                return RESULT::SUCCESS(format!("{} is {}", command, desired_file_path.display()));
            }
        }
    }



    return RESULT::ERROR(format!("{}: not found", command));
}


fn parse_command(command: String) -> COMMAND{
    if command.starts_with("exit") {
        return COMMAND::EXIT;
    }
    else if command.starts_with("echo") {
        let rest = if command.len() > 4 {command[5..].to_string()} else {"".to_string()};
        return COMMAND::ECHO(rest);
    }else if command.starts_with("type"){
        let rest = if command.len() > 4 {command[5..].to_string()} else {"".to_string()};
        return COMMAND::TYPE(rest);
    } else if command.starts_with("pwd") {return COMMAND::PWD;}

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


fn pwd() -> RESULT{
    let path = env::current_dir().unwrap();
    return RESULT::SUCCESS(format!("{}", path.display()));
}

fn process_command(command: COMMAND) -> RESULT{
    match command{
        COMMAND::ECHO(rest) => RESULT::SUCCESS(format!("{}", rest)),
        COMMAND::TYPE(rest) => find_command(rest),
        COMMAND::EXIT => RESULT::SUCCESS("".to_string()),
        COMMAND::NONE(command) => RESULT::ERROR(format!("{}: command not found", command)),
        COMMAND::CUSTOM(program, args) => RESULT::RUN(program, args),
        COMMAND::PWD => pwd(),
    }
}

fn run_custom_command(program: String, args: Vec<String>)-> RESULT{
    let output = Command::new(program)
        .args(args)
        .output()
        .expect("failed to execute process");


    match output.status.code(){
        Some(code) => 
            if code == 0 {
                RESULT::SUCCESS(format!("{}", String::from_utf8_lossy(&output.stdout).trim()))
            }else{
                RESULT::ERROR(format!("{}", String::from_utf8_lossy(&output.stderr).trim()))
            }
        ,
        None => RESULT::ERROR("failed to execute process".to_string()),
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
            RESULT::RUN(program, args) => run_custom_command(program, args),
            _ => {res}
        };


        

        match res{
            RESULT::SUCCESS(msg) => println!("{}", msg),
            RESULT::ERROR(msg) => println!("{}", msg),
            _ => {}
        }


    }


    // get_path();
}
