# OS110_MediumExercism
This is an assignment of Operating System course in semester 110 of Jakarta State University's computer science courses, about completing medium level challenges in exercism.io's Rust track

------

I have chosen five completed medium exercism challenges as the submission for current assignment:

- First batch:
    - DOT DSL
    - Nucleotide Count
    - RNA Transcription
    - Robot Simulator
    - Saddle Points
- Second batch:
    - Anagram
    - Diamond
    - Protein Translation
    - Word Count
- Third batch:
    - ETL
    - Hamming
    - Isogram
    - Perfect Numbers

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

As stated on the instructions, we're only have to find the element(s) which are the smallest among elements in the same column and are the biggest among elements in the same row, despite the shape of the array.

My code can be simplified to this:

```rust
pub fn find_saddle_points(input: Vec of Vec) -> Vec of saddle point indices {
    1. Prepare the container of saddle point indices
    2. Push indices to container if it fulfil this rule:
        a. Iterate all elements in the array, get indices i and j.
        b. Using iterators:
            I. Check if element on (i, j) is biggest among all elements in row i
               (by making iterable out of the current row, simply input[i])
            II. Check if element on (i, j) is smallest among all elements in column j
                (by zipping column j with all possible i indices then get all elements in column)
        c. If both previous conditions are ture, push current indices to the container
    3. Return the container
}
```
