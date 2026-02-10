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

use std::collections::BTreeSet;

fn prefix(a: &String, b: &String) -> String {
    let mut i = 0;
    while i < a.len() && i < b.len() && a.as_bytes()[i] == b.as_bytes()[i] {
        i += 1;
    }
    a[..i].to_string()
}
pub fn longest_prefix(words: &BTreeSet<String>) -> String {
    let mut longest = words.first().unwrap().clone();

    for word in words.iter() {
        longest = prefix(&longest, word)
    }
    longest
}