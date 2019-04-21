fn main () {
    let target = vec![
        "                         A                         ",
        "                        B B                        ",
        "                       C   C                       ",
        "                      D     D                      ",
        "                     E       E                     ",
        "                    F         F                    ",
        "                   G           G                   ",
        "                  H             H                  ",
        "                 I               I                 ",
        "                J                 J                ",
        "               K                   K               ",
        "              L                     L              ",
        "             M                       M             ",
        "            N                         N            ",
        "           O                           O           ",
        "          P                             P          ",
        "         Q                               Q         ",
        "        R                                 R        ",
        "       S                                   S       ",
        "      T                                     T      ",
        "     U                                       U     ",
        "    V                                         V    ",
        "   W                                           W   ",
        "  X                                             X  ",
        " Y                                               Y ",
        "Z                                                 Z",
        " Y                                               Y ",
        "  X                                             X  ",
        "   W                                           W   ",
        "    V                                         V    ",
        "     U                                       U     ",
        "      T                                     T      ",
        "       S                                   S       ",
        "        R                                 R        ",
        "         Q                               Q         ",
        "          P                             P          ",
        "           O                           O           ",
        "            N                         N            ",
        "             M                       M             ",
        "              L                     L              ",
        "               K                   K               ",
        "                J                 J                ",
        "                 I               I                 ",
        "                  H             H                  ",
        "                   G           G                   ",
        "                    F         F                    ",
        "                     E       E                     ",
        "                      D     D                      ",
        "                       C   C                       ",
        "                        B B                        ",
        "                         A                         ",
    ];

    let complete = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let selchar = 'Z';
    let length = complete.chars().enumerate().map(|(i,x)|if x==selchar {1*i} else {0}).fold(0,|acc,x|acc+x)+1;
    println!("Selected char = {}, then the length should be {}",selchar,length);
    let finlength = (2 * &length ) - 1;
    let pivot = &length - 1;
    let used = complete[..length].chars().collect::<Vec<char>>();
    println!("{:?}",used);
    let iterating = ((length as isize)-(finlength as isize)..(length as isize)).map(|x|length-(x.abs() as usize)-1).collect::<Vec<usize>>();
    println!("{:?}",iterating);
    let iterchar = iterating.iter().cloned().map(|x|used[x]).collect::<Vec<char>>();
    println!("{:?}",iterchar);
    for i in iterating.iter().cloned() {
        let mut line = String::new();
        for j in iterating.iter().cloned() {
            if &i+&j == pivot {line.push(used[i]);}
            else {line.push(' ');}
        }
        println!("{:?} :: {:?}",line,target[i]);
    }
}