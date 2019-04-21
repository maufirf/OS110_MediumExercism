use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut newh: BTreeMap<char,i32> = BTreeMap::new();
    h.iter().for_each(|(i,chrvec)| { chrvec.iter().map(|x|x.to_lowercase().to_string().chars().next().unwrap()).for_each(|chr| { newh.insert(chr, *i); }); });
    newh
    //unimplemented!("How will you transform the tree {:?}?", h)
}
