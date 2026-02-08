#[allow(unused_imports)]
mod os_helpers;
mod tokens;

use crate::tokens::*;

mod cmd;
use crate::cmd::*;

mod input;
use crate::input::*;

use std::fs::exists;
use std::io::{self, Write};
use shlex;
use std::{fs};

fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}




fn split_args(command: String) -> Vec<String>{  
    todo!();
}

fn split_redirection(args: Vec<String>) -> (Vec<String>, REDIRECTION){

    let splitted = args.split(|s| {s == "2>>"}).collect::<Vec<&[String]>>();
    if splitted.len() != 1 {
        return  (splitted[0].to_vec(), REDIRECTION::STDERR_APPEND(splitted[1][0].to_string()));
    }

    let splitted = args.split(|s| {s == ">>" || s == "1>>"}).collect::<Vec<&[String]>>();
    if splitted.len() != 1 {
        return (splitted[0].to_vec(), REDIRECTION::STDOUT_APPEND(splitted[1][0].to_string()));
    }


    let splitted = args.split(|s| {
        s == "2>" 
    }).collect::<Vec<&[String]>>();

    if splitted.len() != 1 {
        return  (splitted[0].to_vec(), REDIRECTION::STDERR(splitted[1][0].to_string()));
    }



    let splitted = args.split(|s| {s == ">" || s == "1>"}).collect::<Vec<&[String]>>();
    if splitted.len() != 1 {
        return (splitted[0].to_vec(), REDIRECTION::STDOUT(splitted[1][0].to_string()));
    }

    return (args, REDIRECTION::NONE)
}

fn parse_command(command: String) -> Vec<(COMMAND, REDIRECTION)>{
    let trimmed_command = command.trim();
    let args = shlex::split(&trimmed_command).unwrap();
    

    let  commands = args.split(|s| {s == "|"});
    

    commands.map(|cmd| {

        let (cmd, redirection) = split_redirection(cmd.to_vec());
        (identify_command(cmd.to_vec()), redirection)
    }).collect::<Vec<(COMMAND, REDIRECTION)>>()
}



fn identify_command(args: Vec<String>) -> COMMAND{
    if args.len() == 0 {return COMMAND::NONE("".to_string())}
    
    let start = args[0].as_str();


    match start {
        "exit" => return COMMAND::EXIT,
        "echo" => COMMAND::ECHO(args[1..].to_vec()),
        "type" => COMMAND::TYPE(args[1..].to_vec()),
        "pwd" => COMMAND::PWD,
        "cd" => COMMAND::CD(args[1..].to_vec()), 
        "cat" => COMMAND::CAT(args[1..].to_vec()),
        // custom commands
        _ => COMMAND::EXEC(start.to_string(), args[1..].to_vec()),
    }

}




fn execute_command(command: COMMAND) -> Vec<RESULT>{
    match command{
        COMMAND::ECHO(rest) => vec![cmd_echo(rest)],
        COMMAND::TYPE(rest) => cmd_type(rest),
        COMMAND::EXIT => vec![RESULT::SUCCESS(None)],
        COMMAND::NONE(command) => vec![RESULT::ERROR(format!("{}: command not found", command))],
        COMMAND::EXEC(program, args) => vec![cmd_custom_command(program, args)],
        COMMAND::PWD => vec![cmd_pwd()],
        COMMAND::CD(path) => vec![cmd_cd(path)],
        COMMAND::CAT(paths) => cmd_cat(paths)
    }
}

fn write_in_file(path: &str, content: &str){
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.trim().as_bytes()).unwrap();
    file.flush().unwrap();
}

fn append_in_file(path: &str, content: &str) {
    if !exists(path).unwrap(){
        write_in_file(path, content.trim());
        return;
    }


    let mut prev_contents = fs::read(path).unwrap();
    let content = format!("\n{}", content.trim()).as_bytes().to_vec();
    
    let x = prev_contents.pop();
    match x {
        None =>{},
        Some(ch) =>{
            if ch != b'\n' {prev_contents.push(ch)}
        } 
    }

    prev_contents.extend(content);

    fs::write(path, prev_contents).unwrap();
}

fn output(results: Vec<RESULT>, redirection: REDIRECTION){


    match &redirection{
        REDIRECTION::STDOUT(path) => write_in_file(&path, ""),
        REDIRECTION::STDERR(path) => write_in_file(&path, ""),
        REDIRECTION::STDOUT_APPEND(path) => append_in_file(&path, ""),
        REDIRECTION::STDERR_APPEND(path) => append_in_file(&path, ""),
        _ => {}
    }

    for r in results{
        match (&r, &redirection){
            (RESULT::SUCCESS(Some(msg)), REDIRECTION::STDOUT(path)) => write_in_file(&path, msg),
            (RESULT::SUCCESS(Some(msg)), REDIRECTION::STDOUT_APPEND(path)) => append_in_file(&path, msg),
            (RESULT::ERROR(msg), REDIRECTION::STDERR(path)) => write_in_file(&path, msg),
            (RESULT::ERROR(msg), REDIRECTION::STDERR_APPEND(path)) => append_in_file(&path, msg),
            _ => {
                match r{
                    RESULT::SUCCESS(Some(msg)) => println!("{}", msg),
                    RESULT::ERROR(msg) => println!("{}", msg),
                    _ => {}
                }
            }
            
        }
    }
}


fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = input().unwrap();

        let commands = parse_command(command);

        for (cmd, redirection) in commands{
            if cmd == COMMAND::EXIT {return;}
            let executed_results = execute_command(cmd);
            output(executed_results, redirection);
        }
        
    }


    // get_path();
}


mod test {
    #[test]
    fn test() {
        let txts = vec!["echo hi", ">", "test.txt"];
        let x = txts.split(|s| { s == &">"});
        for i in x{
            println!("{:?}", i);
        }

        assert_eq!(1, 1);
    }
}