#[allow(unused_imports)]
use std::io::{self, Write};
use std::fs;
use std::env;
use std::path::PathBuf;



#[cfg(unix)] // This ensures the following code only compiles on Unix-like systems
use std::os::unix::fs::PermissionsExt;

fn is_executable(file: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        file.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    {
        true
    }
}




static COMMANDS: [&str; 4] = ["exit", "echo", "exit", "type"];

#[derive(PartialEq)]
enum RESULT{
    ERROR(String),
    SUCCESS(String)
}

#[derive(PartialEq)]
enum COMMAND{
    EXIT, 
    ECHO(String),
    TYPE(String),
    NONE(String)
}
fn input_command() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    buffer.to_string()
}

fn get_path() -> Vec<PathBuf>{
    let paths = env::split_paths(&env::var_os("PATH").unwrap_or_default()).collect::<Vec<PathBuf>>();

    paths
}

fn find_command(command: String) -> RESULT{
    for cmd in COMMANDS{
        if(cmd == command){
            return RESULT::SUCCESS(format!("{} is a shell builtin", command));
        }
    }


    let paths = get_path();
    for path in paths{
        let desired_file_path = path.join(&command);
        if(fs::exists(&desired_file_path).unwrap()){
            if(is_executable(&fs::metadata(&desired_file_path).unwrap())){
                return RESULT::SUCCESS(format!("{} is {}", command, desired_file_path.display()));
            }
        }
    }



    return RESULT::ERROR(format!("{}: not found", command));
}


fn parse_command(command: String) -> COMMAND{
    if(command.starts_with("exit")){
        return COMMAND::EXIT;
    }
    else if (command.starts_with("echo")) {
        let rest = if command.len() > 4 {command[5..].to_string()} else {"".to_string()};
        return COMMAND::ECHO(rest);
    }else if(command.starts_with("type")){
        let rest = if command.len() > 4 {command[5..].to_string()} else {"".to_string()};
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

        if(command == COMMAND::EXIT){
            break;
        }
        
        let res = process_command(command);

        match res{
            RESULT::SUCCESS(msg) => println!("{}", msg),
            RESULT::ERROR(msg) => println!("{}", msg),
        }


    }


    // get_path();
}
