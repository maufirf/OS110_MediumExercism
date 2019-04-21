#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num==0 {return None}
    if num==1 {return Some(Classification::Deficient)}
    //let aliquot = (1..(((num as f64).sqrt().ceil()+1f64) as u64)).filter(|x|num%x==0).fold(0,|acc,x|acc+x);
    let aliquot = (1..num).filter(|x|num%x==0).fold(0,|acc,x|acc+x);
    match (aliquot>num,aliquot==num,aliquot<num) {
        (true,false,false) => return Some(Classification::Abundant),
        (false,true,false) => return Some(Classification::Perfect),
        (false,false,true) => return Some(Classification::Deficient),
        _ => return None,
    }
    //unimplemented!("classify {}", num);
}
