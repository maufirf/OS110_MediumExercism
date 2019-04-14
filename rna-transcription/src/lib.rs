#[derive(Debug, PartialEq)]
pub struct DNA(Vec<char>);

#[derive(Debug, PartialEq)]
pub struct RNA(Vec<char>);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let valid_nuc: Vec<char> = vec!['A','T','C','G'];
        let dna_vec: Vec<char> = dna.chars().collect();
        /*
        match dna.chars().try_fold(0, |acc, x| {
            if !valid_nuc.contains(&x) {return Err(acc)}
            Some(acc + 1),
        }) {
            Some(0) => return Ok(DNA(dna.chars().collect::<Vec<char>>())),
            Some(a) => return Err(a),
            None => return Err(0),
        }
        */
        for i in 0..dna.len() {
            if !valid_nuc.contains(&dna_vec[i]) {return Err(i)}
        }
        Ok(DNA(dna_vec))
        //unimplemented!("Construct new DNA from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.iter().map(|&dnuc| {
            match dnuc {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => 'x',
                }
        }).collect::<Vec<char>>()[..].to_vec())

        /*let rna: Vec<char> = Vec::new();
        for i in 0..self.0.len() {

        }*/
        //unimplemented!("Transform DNA {:?} into corresponding RNA", self);
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let valid_nuc: Vec<char> = vec!['A','U','C','G'];
        let rna_vec: Vec<char> = rna.chars().collect();
        /*match rna.chars().all(|x| valid_nuc.contains(&x)) {
            true => return Ok(RNA(rna.chars().collect::<Vec<char>>())),
            false => return Err(0),
        }*/
        for i in 0..rna.len() {
            if !valid_nuc.contains(&rna_vec[i]) {return Err(i)}
        }
        Ok(RNA(rna_vec))
        //unimplemented!("Construct new RNA from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
    }
}
