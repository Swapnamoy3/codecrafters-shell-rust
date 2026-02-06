pub static COMMANDS: [&str; 5] = ["exit", "echo", "exit", "type", "pwd"];

#[derive(PartialEq, Debug)]
pub enum RESULT{
    ERROR(String),
    SUCCESS(Option<String>)
}

#[derive(PartialEq, Debug)]
pub enum COMMAND{
    EXIT, 
    ECHO(Vec<String>),
    TYPE(Vec<String>),
    PWD,
    CD(Vec<String>),
    EXEC(String, Vec<String>),
    NONE(String), 
    CAT(Vec<String>), 
}

#[derive(PartialEq, Debug)]
pub enum REDIRECTION{
    STDOUT(String),
    STDERR(String),
    NONE
    // STDOUTappend(String),
    // STDERRappend(String),
}