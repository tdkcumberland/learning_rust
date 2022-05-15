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
    x : i32,
    y : i32,
    d : Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            d: d
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.direction() {
            Direction::North => Robot { d: Direction::East, .. self},
            Direction::East => Robot { d: Direction::South, .. self},
            Direction::South => Robot { d: Direction::West, .. self},
            Direction::West => Robot { d: Direction::North, .. self}
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.direction() {
            Direction::North => Robot {d: Direction::West, .. self},
            Direction::East => Robot { d: Direction::North, .. self},
            Direction::South => Robot {d: Direction::East, .. self},
            Direction::West => Robot { d: Direction::South, .. self}
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction() {
            Direction::North => Robot { x: self.x, y: self.y + 1, d: self.d},
            Direction::East => Robot { x: self.x + 1, y: self.y, d: self.d},
            Direction::South => Robot { x: self.x, y: self.y - 1, d: self.d},
            Direction::West => Robot { x: self.x - 1, y: self.y, d: self.d}
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut output: Robot = self;

        for c in instructions.chars() {
            output = match c {
                'R' => output.turn_right(),
                'L' => output.turn_left(),
                'A' => output.advance(),
                _ => panic!("Unknow command")
            };
        }

        output
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
