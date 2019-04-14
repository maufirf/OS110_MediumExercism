pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut sad_poi: Vec<(usize, usize)> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() { // if
            if input[i].iter() // all elements in the same row
                .all(|&x| input[i][j] >= x) // should current element bigger or equals to them
            && //and
            (0..input.len()) // all elements in all rows
            .zip(vec![j; input.len()].iter()) // in the same column
            .all(|(im, jm)| input[i][j] <= input[im as usize][*jm as usize]) // should current element smaller or equals to them.
            {sad_poi.push((i,j));} //add current element to result vec
        }
    }
    /*unimplemented!(
        "find the saddle points of the following matrix: {:?}",
        input
    )*/
    sad_poi
}
