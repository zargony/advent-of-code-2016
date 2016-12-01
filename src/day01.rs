use std::str::FromStr;

/// Turning direction of a step
#[derive(Debug, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Turn, &'static str> {
        match s.as_ref() {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err("Illegal turn"),
        }
    }
}

/// A step consists of an initial turn (left or right) and
/// a number of steps to walk into the new direction
#[derive(Debug, PartialEq, Eq)]
pub struct Step {
    turn: Turn,
    dist: i32,
}

impl FromStr for Step {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Step, &'static str> {
        Ok(Step {
            turn: try!(Turn::from_str(&s[0..1])),
            dist: try!(i32::from_str(&s[1..]).map_err(|_| "illegal step distance")),
        })
    }
}

impl Step {
    /// Parse a comma-separated string of step instructions. Returns a vector of steps
    fn parse(input: &str) -> Result<Vec<Step>, &'static str> {
        let mut steps = Vec::new();
        for s in input.split(',') {
            steps.push(try!(Step::from_str(s.trim())));
        }
        Ok(steps)
    }
}

/// Cardinal direction
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Returns the direction left of us
    fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// Returns the direction right of us
    fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Returns the cardinal direction after turning in the given turning direction
    fn turn(&self, turn: &Turn) -> Direction {
        match *turn {
            Turn::Left => self.left(),
            Turn::Right => self.right(),
        }
    }
}

/// Position on a cartesian plane (x and y) and heading (cardinal direction)
#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    /// Returns a new position at location 0,0 facing north
    fn new() -> Position {
        Position { x: 0, y: 0, direction: Direction::North }
    }

    /// Returns the position after walking the given steps from the starting location
    fn walked(steps: &[Step]) -> Position {
        let mut pos = Position::new();
        pos.walk(steps);
        pos
    }

    /// Turns and walks accordingly to the given step instruction
    fn step(&mut self, step: &Step) {
        self.direction = self.direction.turn(&step.turn);
        match self.direction {
            Direction::North => self.y += step.dist,
            Direction::East => self.x += step.dist,
            Direction::South => self.y -= step.dist,
            Direction::West => self.x -= step.dist,
        }
    }

    /// Walk the path given by a slice of step instructions
    fn walk(&mut self, steps: &[Step]) {
        for step in steps {
            self.step(&step);
        }
    }

    /// Returns the distance walked from the starting point (taxicab geometry)
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let steps = Step::parse(include_str!("day01.txt")).unwrap();
    let pos = Position::walked(&steps);
    println!("Distance to easter bunny: {}", pos.distance());
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parsing() {
        assert_eq!(Step::from_str("R2"), Ok(Step { turn: Turn::Right, dist: 2 }));
        assert_eq!(Step::parse("R2, L3"), Ok(vec![Step { turn: Turn::Right, dist: 2 }, Step { turn: Turn::Left, dist: 3 }]));
    }

    #[test]
    fn distance() {
        let steps = Step::parse("R2, L3").unwrap();
        assert_eq!(Position::walked(&steps).distance(), 5);
        let steps = Step::parse("R2, R2, R2").unwrap();
        assert_eq!(Position::walked(&steps).distance(), 2);
        let steps = Step::parse("R5, L5, R5, R3").unwrap();
        assert_eq!(Position::walked(&steps).distance(), 12);
    }
}
