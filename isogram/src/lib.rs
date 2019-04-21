pub fn check(candidate: &str) -> bool {
    let candlow = &candidate.to_lowercase()[..];
    let mut cont: Vec<char> = Vec::new();
    for x in candlow.chars().filter(|x|x.is_alphabetic()){match cont.contains(&x){true=>return false,false=>cont.push(x),};}
    true
    //unimplemented!("Is {} an isogram?", candidate);
}
