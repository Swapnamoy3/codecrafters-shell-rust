pub static COMMANDS: [&str; 4] = ["exit", "echo", "exit", "type"];

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
    CUSTOM(String, Vec<String>),
    NONE(String)
}