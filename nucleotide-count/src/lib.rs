use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_nucs = vec!['A','C','T','G'];
    if valid_nucs.contains(&nucleotide) {//&& dna.chars().all(|ch| valid_nucs.contains(&ch)){
        for ch in dna.chars() {
            match valid_nucs.contains(&ch) {
                true => true,
                false => return Err(ch),
            };
        }
        //false => Err(nucleotide),
        //true => Ok(dna.chars().map(|nuc| if nuc == nucleotide {1} else {0}).sum()),
    }
    else { return Err(nucleotide) }
    Ok(dna.chars().map(|nuc| if nuc == nucleotide {1} else {0}).sum())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut out: HashMap<char, usize> = HashMap::new();
    for ch in "ATCG".chars() {
        let ch_t = count(ch, dna)?;
        out.insert(ch, ch_t);
    }
    Ok(out)
    //unimplemented!("hentai {}", dna)
    /*let res = "ATCG".chars().map(|nuc| (nuc, count(nuc, dna).unwrap())).collect::<Vec<(char, usize)>>();
    let mut out = HashMap::new();
    for i in res.iter() {
        out.insert(i.0, j.0);
    }*/

}
