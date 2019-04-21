use std::collections::HashSet;
use std::collections::BTreeMap;

pub fn anagrams_for<'a>(rawword: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = rawword.to_lowercase();
    let mut out: HashSet<&'a str> = HashSet::new();
    let key = countchar(word);
    for (i, truth) in possible_anagrams.iter().cloned().map(|wrd|countchar(wrd.to_lowercase())==key).enumerate() {
        if truth && possible_anagrams[i].to_lowercase() != rawword.to_lowercase() {
            out.insert(possible_anagrams[i]);
        }
    }
    out
    /*
    unimplemented!(
        "For the '{}' word find anagrams among the following words: {:?}",
        word,
        possible_anagrams
    );
    */
}

fn countchar(word: String) -> BTreeMap<char, usize> {
    let mut container: BTreeMap<char, usize> = BTreeMap::new();
    for chr in word.chars() {
        match container.get(&chr) {
            Some(cnt) => container.insert(chr, cnt+1),
            None => container.insert(chr, 1),
        };
    }
    container
}