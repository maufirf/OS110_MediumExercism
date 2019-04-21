extern crate regex;

use std::collections::HashMap;
use regex::Regex;


/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    /*
    let mut out: HashMap<String, u32> = HashMap::new();
    for word in words.to_lowercase().split(&[' ','\n',',',':'][..]).map(|x|x.to_owned()) {
        match out.get(&word) {
            Some(cnt) => out.insert(word, cnt+1),
            None => out.insert(word, 1),
        };
    }
    out
    */
    let re = Regex::new(r"(\w+'\w+)|\w+").unwrap();
    let mut out: HashMap<String, u32> = HashMap::new();
    for word in re.captures_iter(&words.to_lowercase()) {
        match out.get(&word[0].to_owned()) {
            Some(val) => out.insert(word[0].to_owned(), val+1),
            None => out.insert(word[0].to_owned(), 1),
        };
    }
    out
    //unimplemented!("Count of occurrences of words in {:?}", words);
}
