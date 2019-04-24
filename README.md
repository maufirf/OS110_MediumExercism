*Github.io project page - [parampaa2.github.io/OS110_MediumExercism](https://parampaa2.github.io/OS110_MediumExercism/)*

# OS110_MediumExercism
This is an assignment of Operating System course in semester 110 of Jakarta State University's computer science courses, about completing medium level challenges in exercism.io's Rust track

------

I have chosen five completed medium exercism challenges as the submission for current assignment:

- First batch:
    - [DOT DSL](https://exercism.io/tracks/rust/exercises/dot-dsl/solutions/56e379095cdf4ec9b740cfdd0093631d)
    - [Nucleotide Count](https://exercism.io/tracks/rust/exercises/nucleotide-count/solutions/40c9c657f7c94b34a66ce25be0160d25)
    - [RNA Transcription](https://exercism.io/tracks/rust/exercises/rna-transcription/solutions/4f86d3fc47ee461a8819f8a107a24fb7)
    - [Robot Simulator](https://exercism.io/tracks/rust/exercises/robot-simulator/solutions/c462db8ed7034b51b566d76f63efc575)
    - [Saddle Points](https://exercism.io/tracks/rust/exercises/saddle-points/solutions/ce00a1b1c01442c3bcc4192208a932f8)
- Second batch:
    - [Anagram](https://exercism.io/tracks/rust/exercises/anagram/solutions/7380c4cf14e94280831877e4497eb0e5)
    - [Diamond](https://exercism.io/tracks/rust/exercises/diamond/solutions/96f913e084f14f0fb9b329cf82c32e10)
    - [Protein Translation](https://exercism.io/tracks/rust/exercises/protein-translation/solutions/ade90e6610bb4ab0a38f3ae5f55ec532)
    - [Word Count](https://exercism.io/tracks/rust/exercises/word-count/solutions/e0b803a43e4749cb9f16f9e6e66aef56)
- Third batch:
    - [ETL](https://exercism.io/tracks/rust/exercises/etl/solutions/aa444191be5543ac8e9a7cb16de4f43a)
    - [Hamming](https://exercism.io/tracks/rust/exercises/hamming/solutions/08792d5256c34dd48f519427a9b87820)
    - [Isogram](https://exercism.io/tracks/rust/exercises/isogram/solutions/db7b3da0bc8441e28d556f380d0c713b)
    - [Perfect Numbers](https://exercism.io/tracks/rust/exercises/perfect-numbers/solutions/17820e20136e4753b26a7b8bab98a075)

Also, I will describe on how i solved the Saddle Points problem.

------

# Diamond
### Snippet:
```rust
pub fn get_diamond(c: char) -> Vec<String> {
    let complete = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; //Prepare the set of valid alphabets, in order
    let length = complete.chars().enumerate().map(|(i,x)|if x==c {1*i} else {0}).fold(0,|acc,x|acc+x)+1; //Find the char index in the valid alphabets string
    //let finlength = (2 * &length ) - 1; // This used to be a variable of my final diamond length, but since it is only used once i decide to get rid of it.
    let pivot = &length - 1; //I use this number to mark which place to print a char
    let used = complete[..length].chars().collect::<Vec<char>>(); //Vec of used chars, not the complete alphabet.
    let iterating = ((length as isize)-(((2 * &length ) - 1) as isize)..(length as isize)).map(|x|length-(x.abs() as usize)-1).collect::<Vec<usize>>(); //This is the array i will use in iteration
    let mut out: Vec<String> = Vec::new(); //Resulting string container
    for i in iterating.iter().cloned() { //Iterate through the iterating vec
        let mut line = String::new(); //Prepare current line printing
        for j in iterating.iter().cloned() { //Iterate through the iterating vec,again
            if &i+&j == pivot {line.push(used[i]);} //if sum of index i and j is same like pivot, then it's time to print char.
            else {line.push(' ');} //well if not, just a regular spacebar.
        }
        out.push(line); //At the end of the first level iteration, push the current line to container
    }
    out //Return the container as output
}
```
### The problem
The goal was straightforward, we're only have to make strings that would look like this, specifically a diamond consisted of ordered chars, this is the example for character `E` 'diamond':
```
    A
   B B
  C   C
 D     D
E       E
 D     D
  C   C
   B B 
    A
```
### Pre-coding analysis / problem solving
Without much thinking, i try to reinterpret the characters as numbers, starting from 0:
```
    0
   1 1
  2   2
 3     3
4       4
 3     3
  2   2
   1 1 
    0
    
hence, 'ABCDE' ~ '01234'
also mark that 4 is the maximum index we can get from it.
```
The diamonds has two symmetries that lies on the middle of both x and y axis through the relative center of the diamond object, which means we can divide the diamond into 4, almost similar, repeating pattern which look like this:
```
    0
   1
  2
 3
4
```
After revealing that, i try to make a grid that represents the position of each character:
```
\ 0 1 2 3 4
 +---------
0|        0
1|      1
2|    2
3|  3
4|4
```
Interesting enough, if we sum each of the element's x and y coordinate, we'll have the same number.

```0 -> (0,4) -> 0 + 4 = 4```

```1 -> (1,3) -> 1 + 3 = 4```

```2 -> (2,2) -> 2 + 2 = 4```

```3 -> (3,1) -> 3 + 1 = 4```

```4 -> (4,0) -> 4 + 0 = 4```

And from this point, i will refer to that 'same number' as `pivot`. From this, we can conclude the letters are only 'printed' if the sum of their coordinates matches with `pivot`. Then, to finish my analisis, i continue to use current interpretation to the whole diamond, not just the first fraction. That means i also apply the grid to the whole diamond, but instead of continuing the coordinate numbers following order, i will just mirror the coordinate after the pivot:
```
\ 0 1 2 3 4 3 2 1 0
 +-----------------
0|        0
1|      1   1
2|    2       2
3|  3           3
4|4               4
3|  3           3
2|    2       2
1|      1   1
0|        0
```
In a nutshell, these are the hints that so far we have revealed:
- Alphabets can be interpreted as its index, so `ABCDE` yields `01234`.
- Both axis X and Y has the same number pattern, which is `[0, 1, 2, 3, 4, 3, 2, 1]`. That means, we just need to generate only one array  to be used twice, for both axes.
- Also if we inspect it deep enough, along the y axes, the alphabet indices matches the grid indices on each of the corresponding lines. on `y=0`, we got character index `0` which is `A`, on `y=3` we got character index `3` which is `D`, and so on.
- `pivot`s, suprisingly, is always the `number of characters used` decreased by `1`.

Then, all we have to do is:
1. Decide what letters are going to be used. From letter `A` to the selected character, in ascending order. so we'd have `ABCDE` for letter `E` diamond.
2. Decide the `pivot`. The maximum index of the letter has always the same value to the pivot.
3. Generate an `iterating` array, it goes from `0`, then to `pivot`, then descend back to `0`. So, for example ,for letter `E`, we'll have `[0, 1, 2, 3, 4, 3, 2, 1, 0]` as `iterating` array.
4. Loop over rows (y axis) as `i` using `iterating` array, then the columns (x axis) as `j` also using `iterating` array.
5. In the loop, whenever the sum of `i` and `j` matches `pivot`, print the letter with index of `i`, instead just print a whitespace instead.

### Code
```rust
pub fn get_diamond(c: char) -> Vec<String> {
```
#### Making the array of letters and the pivot
```rust
    //Prepare the set of valid alphabets, in order
    let complete = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    //Find how long should the used letters array be
    let length = 
        complete //the complete alphabet string
            .chars() //convert into char iterator
            .enumerate() //enumerate each character from 0 in the beginning
            .map(|(i,x)|if x==c {1*i} else {0}) //mark which character is chosen as maximum used char, mapping them as i if true, else leave it be 0.
            .fold(0,|acc,x|acc+x)+1; //get the marked number as length
            
    //Determine the pivot. The pivot is always the length decreased by 1
    let pivot = &length - 1;
    
    //Determine the alphabets that we're only using
    let used =
        complete[..length] // The slice of relevant parts of complete alphabet
            .chars() // Convert it to character iterator
            .collect::<Vec<char>>(); // Recollect it as vector
```
#### Generating the iterating array
```rust
    let iterating =
        ((length as isize)-(((2 * &length ) - 1) as isize)..(length as isize)) //Make iterator from -length to length
            // The array yields [-4, -3, -2, -1, 0, 1, 2, 3, 4] for letter E now
            // as isize cast is obligatory because i use negative values. usize will cause overflow
        .map(|x|length-(x.abs() as usize)-1) //Remap the array by:
            // first absoluting all of them
            // then the result of subtracting variable length with each members of current array
            // is the new iterator, which yields [0, 1, 2, 3, 4, 3, 2, 1, 0] now.
            // Take note i convert it back to usize as it is positive now.
        .collect::<Vec<usize>>(); //Recollect it as vector
```
#### Looping over and printing
```rust
    let mut out: Vec<String> = Vec::new(); //Resulting string container
    
    //Looping
    for i in iterating.iter().cloned() {            //Iterate through the iterating vec
        let mut line = String::new();               //Prepare current line printing
        for j in iterating.iter().cloned() {        //Iterate through the iterating vec,again
            if &i+&j == pivot {line.push(used[i]);} //if sum of index i and j is same like pivot, then it's time to print char.
            else {line.push(' ');}                  //well if not, just a regular spacebar.
        }
        out.push(line); //At the end of the first level iteration, push the current line to container
    }
```
#### Returning
```rust
    out //Return the container as output
}
```
so that concludes that!
