extern crate md5;

use std::{fmt, iter};


/// Direction
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Up, Down, Left, Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Direction::*;
        f.write_str(match *self { Up => "U", Down => "D", Left => "L", Right => "R" })
    }
}


/// Path through rooms (a sequence of directions)
pub struct Path {
    x: usize,
    y: usize,
    directions: Vec<Direction>,
    ctx: md5::Context,
    digest: md5::Digest,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.directions.fmt(f)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for dir in &self.directions { try!(dir.fmt(f)); }
        Ok(())
    }
}

impl Path {
    /// Create new, empty path starting at (0,0)
    fn new(passcode: &str) -> Path {
        let mut ctx = md5::Context::new();
        ctx.consume(passcode);
        Path { x: 0, y: 0, directions: Vec::new(), ctx: ctx, digest: ctx.compute() }
    }

    /// Path to string
    fn to_string(&self) -> String {
        format!("{}", self)
    }

    /// Checks if a given direction is clear (i.e. no wall or closed door)
    fn clear(&self, direction: &Direction) -> bool {
        use Direction::*;
        match *direction {
            Up    if self.y > 0 && self.digest[0] >> 4  > 0xa => true,
            Down  if self.y < 3 && self.digest[0] & 0xf > 0xa => true,
            Left  if self.x > 0 && self.digest[1] >> 4  > 0xa => true,
            Right if self.x < 3 && self.digest[1] & 0xf > 0xa => true,
            _ => false
        }
    }

    /// Create a new path by appending the given next direction. Returns `None`
    /// if the given direction is blocked by a wall or a closed door
    fn go(&self, direction: &Direction) -> Option<Path> {
        use Direction::*;
        let mut new_ctx = self.ctx.clone();
        new_ctx.consume(format!("{}", direction));
        match self.clear(&direction) {
            true => Some(Path {
                x: match *direction { Left => self.x-1, Right => self.x+1, _ => self.x },
                y: match *direction { Up   => self.y-1, Down  => self.y+1, _ => self.y },
                directions: self.directions.iter().chain(iter::once(direction)).cloned().collect(),
                ctx: new_ctx,
                digest: new_ctx.compute(),
            }),
            false => None,
        }
    }
}


/// Finds valid paths from (0,0) to (3,3) using breadth-first search (i.e. the
/// first path found will be (one of) the shortest possible paths)
#[derive(Debug)]
pub struct PathFinder {
    paths: Vec<Path>,
    pos: usize,
}

impl PathFinder {
    /// Create new pathfinder for the given passcode
    fn new(passcode: &str) -> PathFinder {
        PathFinder { paths: vec![Path::new(passcode)], pos: 0 }
    }
}

impl Iterator for PathFinder {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        use Direction::*;
        loop {
            if self.pos >= self.paths.len() && !self.paths.is_empty() {
                let mut new_paths = Vec::new();
                for path in &self.paths {
                    for dir in [Up, Down, Left, Right].iter() {
                        if let Some(new_path) = path.go(dir) {
                            new_paths.push(new_path);
                        }
                    }
                }
                self.paths.clear();
                self.paths.extend(new_paths);
                self.pos = 0;
            }
            if !self.paths.is_empty() {
                if self.paths[self.pos].x == 3 && self.paths[self.pos].y == 3 {
                    let path = self.paths.remove(self.pos);
                    return Some(path.to_string());
                }
                self.pos += 1;
            } else {
                return None;
            }
        }
    }
}


fn main() {
    const INPUT: &'static str = "yjjvjgan";
    let mut finder = PathFinder::new(INPUT);
    let shortest_path = finder.next().unwrap();
    println!("Shortest path for passcode '{}': {}", INPUT, shortest_path);
    let longest_path = finder.max_by_key(|p| p.len()).unwrap();
    println!("Length of longest path for passcode '{}': {}", INPUT, longest_path.len());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solving() {
        assert_eq!(PathFinder::new("ihgpwlah").next().unwrap().to_string(), "DDRRRD");
        assert_eq!(PathFinder::new("kglvqrro").next().unwrap().to_string(), "DDUDRLRRUDRD");
        assert_eq!(PathFinder::new("ulqzkmiv").next().unwrap().to_string(), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
