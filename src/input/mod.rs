use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;


mod matcher;
use matcher::*;

mod helpers;
use helpers::*;

struct Input{
    matcher: Matcher,
    input_buff: String,
    ongoing_word: String,
    prev_words: Vec<String>,
    tab_once: bool,
}
impl Input{
    fn new(keywords: Vec<String>) -> Input{
        Input{
            matcher: Matcher::new(keywords),
            input_buff: String::new(),
            ongoing_word: String::new(),
            prev_words: Vec::new(),
            tab_once: false,
        }
    }

    fn set_tab_once(&mut self){
        self.tab_once = true;
    }

    fn unset_tab_once(&mut self){
        self.tab_once = false;
    }


    fn press_space(&mut self){
        space();
        self.prev_words.push(self.ongoing_word.clone());
        self.ongoing_word = String::new();
        self.input_buff.push(' ');
        // self.matcher.reset_pat();
    }
    
    fn press_enter(&mut self){
        self.prev_words.push(self.ongoing_word.clone());
        self.ongoing_word = String::new();
        newline();
    }

    fn press_char(&mut self, ch: char){
        write_char_in_stdout(ch);
        self.ongoing_word.push(ch);
        self.input_buff.push(ch);
    }
    
    fn press_backspace(&mut self){
        if self.input_buff.is_empty() {return;}
        self.input_buff.pop();

        if !self.ongoing_word.is_empty(){
            self.ongoing_word.pop();
        }else{
            if self.prev_words.is_empty() {return;}
            self.ongoing_word = self.prev_words.pop().unwrap();
        }

        backspace();
    }
    
    fn press_single_tab(&mut self){
        self.matcher.set_pat(&self.ongoing_word);
        let matching = self.matcher.get_match();
        let matchings = self.matcher.get_all_matches();
        let prefix = longest_prefix(&matchings);

        if prefix.len() > self.ongoing_word.len()  {

            while !self.ongoing_word.is_empty() {self.press_backspace();}
            for ch in prefix.chars(){
                self.press_char(ch);
            }

            self.matcher.set_pat(&self.ongoing_word);
            return;
        }



        if matchings.len() > 1{
            ring_bell();
            return;
        }
        if let Some(matching) = matching{
            while !self.ongoing_word.is_empty() {self.press_backspace();}
            for ch in matching.chars(){
                self.press_char(ch);
            }
            self.press_space();

        }else{ring_bell();}
    }
    fn press_tab(&mut self){
        if !self.tab_once {
            self.press_single_tab();
            self.tab_once = true;
            return ;
        }

        let matchings = self.matcher.get_all_matches();
        newline();
        for matching in matchings{
            write_str_in_stdout(&matching);
            space(); space();
        }
        newline();  

        write_str_in_stdout(format!("$ {}", self.input_buff).as_ref());
        self.tab_once = false;
    }

    fn print_state(&self){
        println!("input_buff: {}", self.input_buff);
        println!("ongoing_word: {}", self.ongoing_word);
        println!("prev_words: {:#?}", self.prev_words);
        println!("tab_once: {}", self.tab_once);
    }


}

pub fn input(keywords: Vec<String>) -> Result<String, io::Error> {


    let mut input = Input::new(keywords);

    enable_raw_mode()?;
    loop{

        let event = read().unwrap();
        if !event.is_key(){ break; }
        let Event::Key(key) = event else { break; };


        // this is a hack for pressing enter for automated testers -> same as ENTER
        if key.modifiers == crossterm::event::KeyModifiers::CONTROL && key.code == KeyCode::Char('j') {input.press_enter(); break;}

        match key.code{
            KeyCode::Tab | KeyCode::BackTab => {input.press_tab();},

            KeyCode::Enter => {input.press_enter(); break;},
            KeyCode::Char(' ') =>{input.unset_tab_once(); input.press_space();},
            KeyCode::Char(ch) => {input.unset_tab_once(); input.press_char(ch);},
            KeyCode::Backspace => {input.unset_tab_once(); input.press_backspace();},
            _ => {}
        }

        update_stdout();

    }
    disable_raw_mode()?;

    // input.print_state();    
    Ok(input.input_buff)
}





// fn main(){
//     let keywords = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
//     let input = input(keywords).unwrap();
//     println!("{}", input);
// }