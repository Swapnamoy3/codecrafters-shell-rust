#[allow(unused_imports)]
mod os_helpers;
mod tokens;

use crate::tokens::*;

mod cmd;
use crate::cmd::*;

use std::io::{self, Write};
use std::result;
use shlex;



fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}




fn split_args(command: String) -> Vec<String>{  

    
    

    if command.len() == 0 {return vec![]}

    let args = shlex::split(&command);

    args.unwrap()

}

fn parse_command(command: String) -> Vec<COMMAND>{
    let trimmed_command = command.trim();
    let args = shlex::split(&trimmed_command).unwrap();
    

    let mut commands = vec![];
    let mut arg_vec = vec![];
    for arg in args {
        if arg == "|" {
            commands.push(arg_vec);
            arg_vec = vec![];
        }else{
            arg_vec.push(arg);
        }
    }
    commands.push(arg_vec);

    commands.iter().map(|cmd| {
        identify_command(cmd.to_vec())
    }).collect::<Vec<COMMAND>>()


}

fn identify_command(args: Vec<String>) -> COMMAND{

    let start = args[0].as_str();


    match start {
        "exit" => return COMMAND::EXIT,
        "echo" => COMMAND::ECHO(args[1..].to_vec()),
        "type" => COMMAND::TYPE(args[1..].to_vec()),
        "pwd" => COMMAND::PWD,
        "cd" => COMMAND::CD(args[1..].to_vec()), 
        "cat" => COMMAND::CAT(args[1..].to_vec()),
        // custom commands
        _ => COMMAND::CUSTOM(start.to_string(), args[1..].to_vec()),
    }

}





fn process_command(command: COMMAND) -> Vec<RESULT>{
    match command{
        COMMAND::ECHO(rest) => vec![cmd_echo(rest)],
        COMMAND::TYPE(rest) => cmd_type(rest),
        COMMAND::EXIT => vec![RESULT::SUCCESS(None)],
        COMMAND::NONE(command) => vec![RESULT::ERROR(format!("{}: command not found", command))],
        COMMAND::CUSTOM(program, args) => {
            match cmd_type(vec![program.clone()])[0]{
                RESULT::SUCCESS(Some(_)) => vec![RESULT::RUN(program, args)],
                _ => vec![RESULT::ERROR(format!("{}: command not found", program))],
            }
        }
        COMMAND::PWD => vec![cmd_pwd()],
        COMMAND::CD(path) => vec![cmd_cd(path)],
        COMMAND::CAT(paths) => cmd_cat(paths)
    }
}

fn execute_command(exe: RESULT) -> RESULT{
    match exe{
        RESULT::RUN(program, args) => cmd_custom_command(program, args),
        _ => {exe}
    }
}

fn output(results: Vec<RESULT>){
    for r in results{
        match r{
            RESULT::SUCCESS(Some(msg)) => println!("{}", msg),
            RESULT::ERROR(msg) => println!("{}", msg),
            _ => {}
        }
    }
}


fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = input_command();

        let commands = parse_command(command);

        for cmd in commands{
            if cmd == COMMAND::EXIT {return;}
            let intermediate_results = process_command(cmd);



            let executed_results = intermediate_results.into_iter().map(move|r| {
                execute_command(r)
            }).collect();



            output(executed_results);
        }
        


        


    }


    // get_path();
}


mod test {
    use super::*;
    #[test]
    fn test() {

        let args = split_args("ls 1> file.txt".to_string());
        println!("{:?}", args);

        assert_eq!(1, 1);
    }
}