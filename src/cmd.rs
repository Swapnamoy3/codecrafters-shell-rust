use crate::os_helpers::*;


use crate::tokens::*;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::env;




pub fn cmd_echo(args: Vec<String>)-> RESULT{
    // println!("{:?}", args);
    let msg = args.join(" ");
    return RESULT::SUCCESS(Some(msg));
}


pub fn cmd_cat(args: Vec<String>)-> Vec<RESULT>{


    let mut paths = vec![];
    let mut redirection = false;
    let mut redirection_path= "".to_string();
    for arg in args{
        if arg == ">" || arg == "1>" {
            redirection = true;
        }


        if redirection {
            redirection_path = arg;
        }else {
            paths.push(arg);
        }
    }
    
    
    let mut response = Vec::new();
    let mut total = String::new(); 
    for path in paths{
        let content = fs::read_to_string(&path);
        match content {
            Ok(content) => total.push_str(content.trim()),
            Err(e) => response.push(RESULT::ERROR(e.to_string())),
        }
    }


    if redirection{
        
        let mut file = fs::File::create(&redirection_path).unwrap();
        file.write_all(total.as_bytes()).unwrap();

    }else{
        response.push(RESULT::SUCCESS(Some(total)));
    }


    response

}

// impl of type
pub fn cmd_type(commands: Vec<String>) -> Vec<RESULT>{  
    fn find(command: String) -> RESULT{
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

    commands.iter().map(|cmd| find(cmd.to_string())).collect()
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

pub fn cmd_cd(mut path: Vec<String>) -> RESULT{

    if path.len() > 1{
        return RESULT::ERROR("cd: too many arguments".to_string());
    }
    if path.len() == 0 {
        path = vec!["~".to_string()];
    }


    let home = env::var("HOME").unwrap();

    if path[0] == "~"{
        path[0] = home;
    }

    if fs::exists(Path::new(&path[0])).unwrap() {
        env::set_current_dir(&path[0]).unwrap();
        return RESULT::SUCCESS(None);
    }


    return RESULT::ERROR(format!("cd: {}: No such file or directory", path[0]));
}