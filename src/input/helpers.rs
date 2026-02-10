use std::io::{Write, stdout };


pub fn write_char_in_stdout(ch: char) {
    write!(stdout(), "{}", ch).unwrap();
}

pub fn write_str_in_stdout(str: &str) {
    write!(stdout(), "{}", str).unwrap();
}

pub fn space(){
    write!(stdout(), " ").unwrap();
}

pub fn backspace(){
    write!(stdout(), "\u{08} \u{08}").unwrap();
}

pub fn newline(){
    write!(stdout(), "\n\r").unwrap();
}

pub fn ring_bell(){
    write!(stdout(), "\x07").unwrap();
}

pub fn update_stdout(){
    stdout().flush().unwrap();
}