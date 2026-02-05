use crate::os_helpers::*;


use crate::tokens::*;

use std::fs;
use std::process::Command;
use std::env;




pub fn cmd_type(command: String) -> RESULT{ // impl of type 
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


pub fn cmd_pwd() -> RESULT{
    let path = env::current_dir().unwrap();
    return RESULT::SUCCESS(format!("{}", path.display()));
}

pub fn cmd_custom_command(program: String, args: Vec<String>)-> RESULT{
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