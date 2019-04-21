fn main() {
    /*
    let num = 19;
    let sqrt = (num as f64).sqrt().ceil() as u64;
    println!("sqrt({}) = {}",num,sqrt);
    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("{}",i);
    }
    */
    
    //let aliquot = (1..(((num as f64).sqrt().ceil()+1f64) as u64)).filter(|x|num%x==0).fold(0,|acc,x|acc+x);
    let num = 33550335;
    let divisors = (1..num).filter(|x|num%x==0).collect::<Vec<u64>>();
    let aliquot = divisors.iter().cloned().fold(0,|acc,x|acc+x);
    println!("Number {}\n{} = {}",num,divisors.iter().cloned().map(|x|x.to_string()).collect::<Vec<String>>().join(" + "),aliquot);
}