/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len()!=s2.len() {return None}
    Some(s1.to_lowercase().chars().zip(s2.to_lowercase().chars()).map(|(s1c,s2c)|if s1c==s2c{0}else{1}).fold(0,|acc,x|acc+x))
    //unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
}
