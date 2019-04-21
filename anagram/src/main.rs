use std::collections::BTreeMap;

fn countchar(word: &str) -> BTreeMap<char, usize> {
    let mut container: BTreeMap<char, usize> = BTreeMap::new();
    for chr in word.chars() {
        match container.get(&chr) {
            Some(cnt) => container.insert(chr, cnt+1),
            None => container.insert(chr, 1),
        };
    }
    container
}
/*
fn countchar2(word: &str) -> BTreeMap<char, usize> {
    let mut container: BTreeMap<char, usize> = BTreeMap::new();
    word.chars().map(|chr| {
        match container.get(&chr) {
            Some(cnt) => container.insert(chr, cnt+1),
            None => container.insert(chr, 1),
        };
    });
    container
}
*/
fn main() {
    let allergy = countchar("allergy"); //BTreeMap::new();
    let gallery = countchar("gallery");
    //chrs.increment('l');
    //chrs.increment('a');
    //chrs.increment('a');
    //let a: BTreeMap<char,usize> = vec!
    println!("{:?}",allergy);
    println!("{:?}",gallery);
    println!("{}",allergy==gallery);
}