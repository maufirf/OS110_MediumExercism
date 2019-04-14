// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x: x, y: y, d: d }
        //unimplemented!("Create a robot at (x, y) ({}, {}) facing {:?}", x, y, d,)
    }

    pub fn turn_right(mut self) -> Self {
        match self.d {
            Direction::North => self.d = Direction::East,
            Direction::East => self.d = Direction::South,
            Direction::South => self.d = Direction::West,
            Direction::West => self.d = Direction::North,
        }
        self
        //unimplemented!()
    }

    pub fn turn_left(mut self) -> Self {
        match self.d {
            Direction::North => self.d = Direction::West,
            Direction::East => self.d = Direction::North,
            Direction::South => self.d = Direction::East,
            Direction::West => self.d = Direction::South,
        }
        self
        //unimplemented!()
    }

    pub fn advance(mut self) -> Self {
        //OKAY SO A COMPASS DOES LOOK LIKE THIS:
        //               N
        //             W + E
        //               S
        match self.d {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
        self
        //unimplemented!()
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for i in instructions.chars() {
            /*match i {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
            };*/
            if i == 'A' {self = self.advance();}
            else if i == 'L' {self = self.turn_left();}
            else if i == 'R' {self = self.turn_right();}
        }
        self
        /*unimplemented!(
            "Follow the given sequence of instructions: {}",
            instructions
        )*/
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
        //unimplemented!()
    }

    pub fn direction(&self) -> &Direction {
        &self.d
        //unimplemented!()
    }
}
