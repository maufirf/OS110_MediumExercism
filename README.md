# OS110_MediumExercism
This is an assignment of Operating System course in semester 110 of Jakarta State University's computer science courses, about completing medium level challenges in exercism.io's Rust track

------

I have chosen five completed medium exercism challenges as the submission for current assignment:

- DOT DSL
- Nucleotide Count
- RNA Transcription
- Robot Simulator
- Saddle Points

Also, I will describe on how i solved the Saddle Points problem.

------

# Saddle Points
Snippet:
```rust
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
    sad_poi
}
```

**TODO** complete readme
