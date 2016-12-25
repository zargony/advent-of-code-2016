extern crate itertools;
extern crate permutohedron;

use std::fmt;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use itertools::Itertools;


pub struct Map {
    walls: Vec<Vec<bool>>,
    waypoints: HashMap<char, (usize, usize)>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Map, ()> {
        let mut waypoints = HashMap::new();
        let walls = s.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, ch)| {
                match ch {
                    '#' => true,
                    '.' => false,
                    _   => { waypoints.insert(ch, (x, y)); false },
                }
            }).collect()
        }).collect();
        Ok(Map {
            walls: walls,
            waypoints: waypoints,
        })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MapDisplay { map: self, path: None }.fmt(f)
    }
}

impl Map {
    /// Height of map
    fn height(&self) -> usize {
        self.walls.len()
    }

    /// Width of map
    fn width(&self) -> usize {
        self.walls[0].len()
    }

    /// Check if the given position is blocked by a wall
    fn is_wall(&self, x: usize, y: usize) -> bool {
        x >= self.width() || y >= self.height() || self.walls[y][x]
    }

    /// Returns possible moves from the given position
    fn possible_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        [
            if          !self.is_wall(x+1, y) { Some((x+1, y)) } else { None },
            if          !self.is_wall(x, y+1) { Some((x, y+1)) } else { None },
            if x > 0 && !self.is_wall(x-1, y) { Some((x-1, y)) } else { None },
            if y > 0 && !self.is_wall(x, y-1) { Some((x, y-1)) } else { None },
        ].iter().filter_map(|x| *x).collect()
    }

    /// Display map with given path
    #[allow(dead_code)]
    fn display<'a>(&'a self, path: &'a [(usize, usize)]) -> MapDisplay {
        MapDisplay { map: self, path: Some(path) }
    }

    /// Find shortest path between given waypoints
    fn find_path(&self, from: char, to: char) -> Option<Vec<(usize, usize)>> {
        self.waypoints.get(&from).and_then(|&(from_x, from_y)| {
            self.waypoints.get(&to).and_then(|&(to_x, to_y)| {
                let mut paths: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
                paths.push_back(vec![(from_x, from_y)]);
                let mut shortest = HashMap::new();
                shortest.insert((from_x, from_y), 0);
                while let Some(path) = paths.pop_front() {
                    let &(x, y) = path.last().unwrap();
                    for (next_x, next_y) in self.possible_moves(x, y) {
                        if path.contains(&(next_x, next_y)) {
                            continue;
                        }
                        if shortest.get(&(next_x, next_y)).map(|l| *l <= path.len()).unwrap_or(false) {
                            continue;
                        }
                        let mut new_path = path.clone();
                        new_path.push((next_x, next_y));
                        if next_x == to_x && next_y == to_y {
                            return Some(new_path);
                        }
                        paths.push_back(new_path);
                        shortest.insert((next_x, next_y), path.len());
                    }
                }
                None
            })
        })
    }

    /// Calculate distances between all waypoints
    fn waypoint_dists(&self) -> HashMap<(char, char), usize> {
        let mut dists = HashMap::new();
        let mut it = self.waypoints.keys();
        while let Some(&from) = it.next() {
            for &to in it.clone() {
                if let Some(path) = self.find_path(from, to) {
                    dists.insert((from ,to), path.len() - 1);
                    dists.insert((to, from), path.len() - 1);
                }
            }
        }
        dists
    }

    /// Calculate shortest distance for visiting all remaining waypoints
    fn shortest_dist_visiting_all(&self, from: char, returning: bool) -> Option<usize> {
        let dists = self.waypoint_dists();
        let mut shortest = None;
        let mut waypoints: Vec<_> = self.waypoints.keys().filter(|&&ch| ch != from).collect();
        for waypoints in permutohedron::Heap::new(&mut waypoints) {
            let first = *waypoints[0];
            let mut dist = *dists.get(&(from, first)).unwrap();
            dist += waypoints.iter().tuple_windows().map(|(&&a, &&b)| dists.get(&(a, b)).unwrap()).sum();
            if returning {
                let last = **waypoints.last().unwrap();
                dist += *dists.get(&(last, from)).unwrap();
            }
            if shortest.is_none() || dist < shortest.unwrap() {
                shortest = Some(dist);
            }
        }
        shortest
    }
}


pub struct MapDisplay<'a> {
    map: &'a Map,
    path: Option<&'a [(usize, usize)]>,
}

impl<'a> fmt::Display for MapDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let waypoints: HashMap<(usize, usize), char> = self.map.waypoints.iter().map(|(ch, pos)| (*pos, *ch)).collect();
        for (y, row) in self.map.walls.iter().enumerate() {
            try!(f.write_str("\n"));
            for (x, wall) in row.iter().enumerate() {
                let on_path = self.path.map_or(false, |path| path.contains(&(x, y)));
                let waypoint = waypoints.get(&(x, y));
                match (*wall, on_path, waypoint) {
                    (_, _, Some(ch)) => try!(f.write_fmt(format_args!("{}", ch))),
                    (false, false, _) => try!(f.write_str(".")),
                    (true, false, _) => try!(f.write_str("#")),
                    (_, true, _) => try!(f.write_str("*")),
                }
            }
        }
        Ok(())
    }
}


fn main() {
    let map: Map = include_str!("day24.txt").parse().unwrap();
    println!("Fewest number of steps to visit all markers (non-returning): {}", map.shortest_dist_visiting_all('0', false).unwrap());
    println!("Fewest number of steps to visit all markers (returning): {}", map.shortest_dist_visiting_all('0', true).unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = "###########\n#0.1.....2#\n#.#######.#\n#4.......3#\n###########";

    #[test]
    fn parsing() {
        let map: Map = TEST_DATA.parse().unwrap();
        assert_eq!(map.height(), 5);
        assert_eq!(map.width(), 11);
    }

    #[test]
    fn finding_paths() {
        let map: Map = TEST_DATA.parse().unwrap();
        assert_eq!(map.find_path('0', '4'), Some(vec![(1, 1), (1, 2), (1, 3)]));
        assert_eq!(map.find_path('4', '1'), Some(vec![(1, 3), (1, 2), (1, 1), (2, 1), (3, 1)]));
        assert_eq!(map.find_path('1', '2'), Some(vec![(3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1)]));
        assert_eq!(map.find_path('2', '3'), Some(vec![(9, 1), (9, 2), (9, 3)]));
    }

    #[test]
    fn distances() {
        let map: Map = TEST_DATA.parse().unwrap();
        let dists = map.waypoint_dists();
        assert_eq!(dists.get(&('0', '4')), Some(&2));
        assert_eq!(dists.get(&('4', '1')), Some(&4));
        assert_eq!(dists.get(&('1', '2')), Some(&6));
        assert_eq!(dists.get(&('2', '3')), Some(&2));
    }

    #[test]
    fn shortest_roundtrip() {
        let map: Map = TEST_DATA.parse().unwrap();
        assert_eq!(map.shortest_dist_visiting_all('0', false), Some(14));
    }
}
