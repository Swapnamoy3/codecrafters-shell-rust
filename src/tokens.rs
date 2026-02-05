pub static COMMANDS: [&str; 5] = ["exit", "echo", "exit", "type", "pwd"];

#[derive(PartialEq)]
pub enum RESULT{
    ERROR(String),
    RUN(String, Vec<String>),
    SUCCESS(Option<String>)
}

#[derive(PartialEq)]
pub enum COMMAND{
    EXIT, 
    ECHO(Vec<String>),
    TYPE(String),
    PWD,
    CD(String),
    CUSTOM(String, Vec<String>),
    NONE(String), 
    CAT(Vec<String>)
}