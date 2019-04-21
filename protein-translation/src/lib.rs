//use std::marker::PhantomData;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    //phantom: PhantomData<&'a ()>,
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        let idx: isize = self.pairs.iter().enumerate().map(|(i, x)|if x.0==codon {i+1} else {0}).fold(0,|acc,idx|acc+idx) as isize - 1;
        return if idx==-1 {None} else {Some(self.pairs[idx as usize].1)}
        //unimplemented!("Return the protein name for a '{}' codon or None, if codon string is invalid", codon);
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut out: Vec<&'a str> = Vec::new();
        for i in (0..(rna.len()/3)).map(|x|x*3) {
            let name = self.name_for(rna.get(i..(i+3)).unwrap_or("ERRTYPE!")).unwrap_or("ERRON!");
            if name=="stop codon" {return Some(out)}
            out.push(name);
        }
        if out.contains(&"ERRON!") {return None}
        Some(out)
        //unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or None if the RNA string is invalid", rna);
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    //println!("{:?}",pairs);
    CodonsInfo { pairs : pairs }
    //unimplemented!("Construct a new CodonsInfo struct from given pairs: {:?}",pairs);
}
