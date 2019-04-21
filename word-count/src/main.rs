extern crate regex;

use word_count::word_count;

use regex::Regex;

fn main () {
    /*
    for word in "A few words".to_lowercase().split_whitespace() {
        println!("{}",word);
    }
    */
    let words = "car : 'carpet' as 'don't' can't    \"java\" : javascript!!&@$%^&";
    let re = Regex::new(r"(\w+'\w+)|\w+").unwrap();
    for cap in re.captures_iter(words) {
        println!("{}", &cap[0]);
    }
    println!("{:?}",word_count(words));
}