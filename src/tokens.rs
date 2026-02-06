pub static COMMANDS: [&str; 5] = ["exit", "echo", "exit", "type", "pwd"];

#[derive(PartialEq)]
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