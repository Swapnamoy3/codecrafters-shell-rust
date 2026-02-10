use std::collections::BTreeSet;

pub struct Matcher{
    words: Vec<String>, 
    idx: i32, 
    pat: String,
    pat_set: bool
} 

impl Matcher{
    pub fn new(words: Vec<String>) -> Matcher {
        Matcher{words, idx: -1, pat: "".to_string(), pat_set: false}
    }

    pub fn set_pat(&mut self, pat: &String) {
        if self.pat_set {
            return;
        }
        self.pat = pat.clone();
        self.idx = -1;
        self.pat_set = true;
    }

    pub fn reset_pat(&mut self) {
        self.pat = "".to_string();
        self.idx = -1;
        self.pat_set = false;
    }

    pub fn next(&mut self) -> Option<String> {
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

    pub fn get_match(&self) -> Option<String> {
        for word in &self.words {
            if word.starts_with(&self.pat) {
                return Some(word.clone());
            }
        }
        None
    }

    pub fn get_all_matches(&self) -> BTreeSet<String> {
        
        self.words.clone().into_iter().filter(|s| {
            s.starts_with(&self.pat)
        }).collect()
    }

}