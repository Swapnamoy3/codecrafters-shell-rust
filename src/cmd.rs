use crate::os_helpers::*;


use crate::tokens::*;

use std::fs;
use std::path;
use std::path::Path;
use std::process::Command;
use std::env;



pub fn cmd_echo(args: Vec<String>)-> RESULT{
    // println!("{:?}", args);
    let msg = args.join(" ");
    return RESULT::SUCCESS(Some(msg));
}

pub fn cmd_cat(path: Vec<String>)-> RESULT{

    let content = path.iter().map(|pth|{
        fs::read_to_string(pth).unwrap()
    }).collect::<Vec<String>>().join("").trim().to_string();

    return RESULT::SUCCESS(Some(content));
}

// impl of type
pub fn cmd_type(command: String) -> RESULT{  
    for cmd in COMMANDS{
        if cmd == command {
            return RESULT::SUCCESS(Some(format!("{} is a shell builtin", command)));
        }
    }
    

    let paths = get_path();
    for path in paths{
        let desired_file_path = path.join(&command);
        if fs::exists(&desired_file_path).unwrap() {
            
            if is_executable(&fs::metadata(&desired_file_path).unwrap()) {
                return RESULT::SUCCESS(Some(format!("{} is {}", command, desired_file_path.display())));
            }
        }
    }

    
    
    return RESULT::ERROR(format!("{}: not found", command));
}

// impl of pwd
pub fn cmd_pwd() -> RESULT{
    let path = env::current_dir().unwrap();
    return RESULT::SUCCESS(Some(format!("{}", path.display())));
}


// impl of custom
pub fn cmd_custom_command(program: String, args: Vec<String>)-> RESULT{
    let output = Command::new(program)
        .args(args)
        .output()
        .expect("failed to execute process");


    match output.status.code(){
        Some(code) => 
            if code == 0 {
                RESULT::SUCCESS(Some(format!("{}", String::from_utf8_lossy(&output.stdout).trim())))
            }else{
                RESULT::ERROR(format!("{}", String::from_utf8_lossy(&output.stderr).trim()))
            }
        ,
        None => RESULT::ERROR("failed to execute process".to_string()),
    }
}

pub fn cmd_cd(mut path: String) -> RESULT{

    
    let home = env::var("HOME").unwrap();

    if path == "~"{
        path = home;
    }

    if fs::exists(Path::new(&path)).unwrap() {
        env::set_current_dir(&path).unwrap();
        return RESULT::SUCCESS(None);
    }


    return RESULT::ERROR(format!("cd: {}: No such file or directory", path));
}