use std::fmt;

/// A Maze of office cubicles
pub struct Maze {
    seed: u32,
}

impl Maze {
    /// Create new maze using the given seed
    fn new(seed: u32) -> Maze {
        Maze { seed: seed }
    }

    /// Returns true if there's a wall at the given coordinate
    fn is_solid_at(&self, x: u32, y: u32) -> bool {
        (x*x + 3*x + 2*x*y + y + y*y + self.seed).count_ones() % 2 > 0
    }

    /// Returns an object that displays the maze in the given size
    #[allow(dead_code)]
    fn display(&self, width: u32, height: u32) -> MazeDisplay {
        MazeDisplay { maze: self, width: width, height: height, path: None }
    }

    /// Returns an object that displays the maze and path in the given size
    #[allow(dead_code)]
    fn display_path<'a>(&'a self, width: u32, height: u32, path: &'a [(u32, u32)]) -> MazeDisplay {
        MazeDisplay { maze: self, width: width, height: height, path: Some(path) }
    }

    /// Returns a pathfinder that targets the given coordinates
    fn pathfinder(&self, x: u32, y: u32, maxdepth: usize) -> PathFinder {
        PathFinder::new(self, x, y, maxdepth)
    }
}

/// Maze display helper
pub struct MazeDisplay<'a> {
    maze: &'a Maze,
    width: u32,
    height: u32,
    path: Option<&'a [(u32, u32)]>,
}

impl<'a> fmt::Display for MazeDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match (self.maze.is_solid_at(x, y), self.path.map(|p| p.contains(&(x, y))).unwrap_or(false)) {
                    (true, _) => try!(f.write_str("#")),
                    (false, true) => try!(f.write_str("O")),
                    (false, false) => try!(f.write_str(".")),
                }
            }
            try!(f.write_str("\n"));
        }
        Ok(())
    }
}

/// Maze path finder
pub struct PathFinder<'a> {
    maze: &'a Maze,
    target: (u32, u32),
    maxdepth: usize,
    paths: Vec<Vec<(u32, u32)>>,
    pos: usize,
}

impl<'a> PathFinder<'a> {
    /// Create new pathfinder that targets the given location, starting at (1,1)
    fn new(maze: &Maze, x: u32, y: u32, maxdepth: usize) -> PathFinder {
        let paths = vec![vec![(1, 1)]];
        PathFinder { maze: maze, target: (x, y), maxdepth: maxdepth, paths: paths, pos: 0 }
    }
}

impl<'a> Iterator for PathFinder<'a> {
    type Item = Vec<(u32, u32)>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.paths.is_empty() {
            while self.pos < self.paths.len() {
                let path = self.paths[self.pos].clone();    // FIXME: allocation :-(
                self.pos += 1;
                if *path.last().unwrap() == self.target {
                    return Some(path);
                }
            }
            fn offset(n: u32, o: i32) -> u32 {
                if o < 0 { n - (-o) as u32 } else { n + o as u32 }
            }
            let mut new_paths = Vec::new();
            for path in &self.paths {
                if path.len() >= self.maxdepth { break; }
                let last = path.last().unwrap();
                for &(dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
                    if dx < 0 && last.0 == 0 { continue; }
                    if dy < 0 && last.1 == 0 { continue; }
                    let next = (offset(last.0, dx), offset(last.1, dy));
                    if self.maze.is_solid_at(next.0, next.1) { continue; }
                    if self.paths.iter().any(|path| path.iter().any(|p| *p == next)) { continue; }
                    let mut new_path = path.to_owned();
                    new_path.push(next);
                    new_paths.push(new_path);
                }
            }
            self.paths = new_paths;
            self.pos = 0;
        }
        None
    }
}

fn main() {
    let maze = Maze::new(1362);
    //print!("{}", maze.display(50, 25));
    let path = maze.pathfinder(31, 39, 500).min_by_key(|path| path.len()).unwrap();
    //print!("{}", maze.display_path(50, 25, &path));
    println!("Fewest number of steps to reach 31,39: {}", path.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze() {
        let maze = Maze::new(10);
        assert_eq!(format!("{}", maze.display(10, 7)), ".#.####.##\n..#..#...#\n#....##...\n###.#.###.\n.##..#..#.\n..##....#.\n#...##.###\n");
    }

    #[test]
    fn pathfinding() {
        let maze = Maze::new(10);
        let path = maze.pathfinder(7, 4, 100).min_by_key(|path| path.len()).unwrap();
        assert_eq!(format!("{}", maze.display_path(10, 7, &path)), ".#.####.##\n.O#..#...#\n#OOO.##...\n###O#.###.\n.##OO#OO#.\n..##OOO.#.\n#...##.###\n")
    }
}
