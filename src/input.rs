use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write, stdout };



struct Matcher{
    words: Vec<String>, 
    idx: i32, 
    pat: String,
    pat_set: bool
} 

impl Matcher{
    fn new(words: Vec<String>) -> Matcher {
        Matcher{words, idx: -1, pat: "".to_string(), pat_set: false}
    }

    fn set_pat(&mut self, pat: &String) {
        if self.pat_set {
            return;
        }
        self.pat = pat.clone();
        self.idx = -1;
        self.pat_set = true;
    }

    fn reset_pat(&mut self) {
        self.pat = "".to_string();
        self.idx = -1;
        self.pat_set = false;
    }

    fn next(&mut self) -> Option<String> {
        let len = self.words.len() as i32;
        if len == 0 {
            return None;
        }

        for _ in 0..len {
            self.idx = (self.idx + 1) % len;

            if self.words[self.idx as usize].starts_with(&self.pat) {
                return Some(self.words[self.idx as usize].clone());
            }
        }

        None
    }

}

fn write_char_in_stdout(ch: char) {
    write!(stdout(), "{}", ch).unwrap();
    stdout().flush().unwrap();
}

fn write_str_in_stdout(str: &str) {
    write!(stdout(), "{}", str).unwrap();
    stdout().flush().unwrap();
}

fn write_backspace(){
    write!(stdout(), "\u{08} \u{08}").unwrap();
    stdout().flush().unwrap();
}

pub fn input(keywords: Vec<String>) -> Result<String, io::Error> {

    let mut matcher = Matcher::new(keywords);
    let mut input_buff = String::new();
    let mut ongoing_word = String::new();



    enable_raw_mode()?;
    loop{
        match read().unwrap() {
            Event::Key(key) => {
                
                if key.modifiers == crossterm::event::KeyModifiers::CONTROL && key.code == KeyCode::Char('j') {
                    write_char_in_stdout('\n');
                    write_char_in_stdout('\r');
                    // write_str_in_stdout(format!("input: {}\r\n", input_buff).as_ref());
                    // write_str_in_stdout(format!("ongoing word: {}\r\n", ongoing_word).as_ref());
                    // write_str_in_stdout(format!("matcher pat word: {}\r\n", matcher.pat).as_ref());
                    matcher.reset_pat();
                    break;
                }

                match key.code {
                    KeyCode::Tab | KeyCode::BackTab => {
                        matcher.set_pat(&ongoing_word);
                        let matching_word = matcher.next();

                        if matching_word.is_none() { 
                            write_char_in_stdout('\x07');
                            continue;
                        }

                        while !input_buff.is_empty() {
                            let ch = input_buff.pop().unwrap();
                            if ch != ' '{
                                input_buff.push(ch);
                                break;
                            } 

                            write_backspace();
                        }


                        while !ongoing_word.is_empty() {
                            input_buff.pop();
                            ongoing_word.pop();
                            write_backspace();
                        }

                        for ch in matching_word.unwrap().chars() {
                            ongoing_word.push(ch);
                            input_buff.push(ch);
                            write_char_in_stdout(ch);
                        }
                        write_char_in_stdout(' ');
                        input_buff.push(' ');

                    },

                    KeyCode::Enter => {
                        write_char_in_stdout('\n');
                        write_char_in_stdout('\r');
                        // write_str_in_stdout(format!("input: {}\r\n", input_buff).as_ref());
                        // write_str_in_stdout(format!("ongoing word: {}\r\n", ongoing_word).as_ref());
                        // write_str_in_stdout(format!("matcher pat word: {}\r\n", matcher.pat).as_ref());
                        matcher.reset_pat();
                        break;
                    },
                    
                    KeyCode::Backspace => {
                        if !ongoing_word.is_empty() { ongoing_word.pop(); }
                        if !input_buff.is_empty() { input_buff.pop(); }
                        write_backspace();
                        matcher.reset_pat();
                        matcher.set_pat(&ongoing_word);
                    },
                    
                    KeyCode::Char(ch) => {
                        
                        if ch == ' '{
                            // write_str_in_stdout(format!("\n\rongoing word: {}", ongoing_word).as_ref());
                            ongoing_word.clear();
                        }else{
                            if !input_buff.is_empty() {
                                let last_ch: char = input_buff.pop().unwrap();
                                input_buff.push(last_ch);
                                if last_ch == ' '{
                                    ongoing_word.clear();
                                }
                            }
                            ongoing_word.push(ch);
                        }

                        input_buff.push(ch);
                        matcher.reset_pat();
                        write_char_in_stdout(ch);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    disable_raw_mode()?;
    
    Ok(input_buff)
}

