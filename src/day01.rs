use std::collections::HashSet;
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
    /// Returns the cardinal direction after turning in the given turning direction
    fn turn(&self, turn: &Turn) -> Direction {
        match (turn, self) {
            (&Turn::Left, &Direction::North) => Direction::West,
            (&Turn::Left, &Direction::East) => Direction::North,
            (&Turn::Left, &Direction::South) => Direction::East,
            (&Turn::Left, &Direction::West) => Direction::South,
            (&Turn::Right, &Direction::North) => Direction::East,
            (&Turn::Right, &Direction::East) => Direction::South,
            (&Turn::Right, &Direction::South) => Direction::West,
            (&Turn::Right, &Direction::West) => Direction::North,
        }
    }
}

/// Position on a cartesian plane (x and y) and heading (cardinal direction)
#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
    direction: Direction,
    visited: HashSet<(i32, i32)>,
}

impl Position {
    /// Create a position at starting location 0,0, facing north
    fn new() -> Position {
        Position { x: 0, y: 0, direction: Direction::North, visited: HashSet::new() }
    }

    /// Create a position that has walked the given steps from the starting location
    fn walked(steps: &[Step], until_visited_twice: bool) -> Position {
        let mut pos = Position::new();
        pos.walk(steps, until_visited_twice);
        pos
    }

    /// Go to the given relative location. Returns true if we've been there before
    fn goto(&mut self, dx: i32, dy: i32) -> bool {
        self.x += dx;
        self.y += dy;
        let res = !self.visited.insert((self.x, self.y));
        res
    }

    /// Turns and walks accordingly to the given step instruction. If `until_visited_twice`
    /// is true, only walk until a location is hit that we've been before and return true.
    fn step(&mut self, step: &Step, until_visited_twice: bool) -> bool {
        self.direction = self.direction.turn(&step.turn);
        for _ in 0..step.dist {
            if match self.direction {
                Direction::North => self.goto(0, 1),
                Direction::East => self.goto(1, 0),
                Direction::South => self.goto(0, -1),
                Direction::West => self.goto(-1, 0),
            } && until_visited_twice {
                return true;
            }
        }
        false
    }

    /// Walk the path given by a slice of step instructions
    fn walk(&mut self, steps: &[Step], until_visited_twice: bool) {
        for step in steps {
            if self.step(&step, until_visited_twice) && until_visited_twice {
                return;
            }
        }
    }

    /// Returns the distance walked from the starting point (taxicab geometry)
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let steps = Step::parse(include_str!("day01.txt")).unwrap();
    let pos = Position::walked(&steps, false);
    println!("Distance to easter bunny: {}", pos.distance());
    let pos = Position::walked(&steps, true);
    println!("Distance to easter bunny (until visited twice): {}", pos.distance());
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
        assert_eq!(Position::walked(&steps, false).distance(), 5);
        let steps = Step::parse("R2, R2, R2").unwrap();
        assert_eq!(Position::walked(&steps, false).distance(), 2);
        let steps = Step::parse("R5, L5, R5, R3").unwrap();
        assert_eq!(Position::walked(&steps, false).distance(), 12);
    }

    #[test]
    fn distance_until_visited_twice() {
        let steps = Step::parse("R8, R4, R4, R8").unwrap();
        assert_eq!(Position::walked(&steps, true).distance(), 4);
    }
}
