#[allow(unused_imports)]
mod os_helpers;
mod tokens;

use crate::tokens::*;

mod cmd;
use crate::cmd::*;

use std::io::{self, Write};
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
fn split_redirection(args: Vec<String>) -> (Vec<String>, String){

    let splitted = args.split(|s| {
        s == ">" || s == "1>"
    }).collect::<Vec<&[String]>>();

    if splitted.len() == 1 {return (args, "".to_string())};


    (splitted[0].to_vec(), splitted[1][0].to_string())
}

fn parse_command(command: String) -> Vec<(COMMAND, String)>{
    let trimmed_command = command.trim();
    let args = shlex::split(&trimmed_command).unwrap();
    

    let  commands = args.split(|s| {s == "|"});
    

    commands.map(|cmd| {

        let (cmd, redirection) = split_redirection(cmd.to_vec());
        (identify_command(cmd.to_vec()), redirection)
    }).collect::<Vec<(COMMAND, String)>>()
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



fn output(results: Vec<RESULT>, redirection: String){
    for r in results{
        match r{
            RESULT::SUCCESS(Some(msg)) => {
                if redirection.len() == 0{
                    println!("{}", msg);
                }else{
                    let mut file = std::fs::File::create(&redirection).unwrap();
                    file.write_all(msg.as_bytes()).unwrap();
                }
            },
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

        for (cmd, redirection) in commands{
            if cmd == COMMAND::EXIT {return;}
            let executed_results = execute_command(cmd);
            output(executed_results, redirection);
        }
        
    }


    // get_path();
}


mod test {
    use std::process::Command;
    use super::*;
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