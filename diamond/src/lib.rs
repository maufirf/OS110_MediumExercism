pub fn get_diamond(c: char) -> Vec<String> {
    let complete = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let length = complete.chars().enumerate().map(|(i,x)|if x==c {1*i} else {0}).fold(0,|acc,x|acc+x)+1;
    //let finlength = (2 * &length ) - 1;
    let pivot = &length - 1;
    let used = complete[..length].chars().collect::<Vec<char>>();
    let iterating = ((length as isize)-(((2 * &length ) - 1) as isize)..(length as isize)).map(|x|length-(x.abs() as usize)-1).collect::<Vec<usize>>();
    let mut out: Vec<String> = Vec::new();
    for i in iterating.iter().cloned() {
        let mut line = String::new();
        for j in iterating.iter().cloned() {
            if &i+&j == pivot {line.push(used[i]);}
            else {line.push(' ');}
        }
        out.push(line);
    }
    out
    /*
    unimplemented!(
        "Return the vector of strings which represent the diamond with particular char {}",
        c
    );
    */
}
