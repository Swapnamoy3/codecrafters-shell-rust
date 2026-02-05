pub static COMMANDS: [&str; 5] = ["exit", "echo", "exit", "type", "pwd"];

#[derive(PartialEq)]
pub enum RESULT{
    ERROR(String),
    RUN(String, Vec<String>),
    SUCCESS(String)
}

#[derive(PartialEq)]
pub enum COMMAND{
    EXIT, 
    ECHO(String),
    TYPE(String),
    PWD,
    CUSTOM(String, Vec<String>),
    NONE(String)
}