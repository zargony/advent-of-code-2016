extern crate itertools;

use std::{fmt, iter};
use std::str::FromStr;

use itertools::Itertools;


/// A row of tiles in a room
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Row {
    tiles: Vec<bool>,
}

impl FromStr for Row {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Row, &'static str> {
        let tiles: Result<Vec<bool>, _> = s.chars().map(|ch| match ch {
            '.' => Ok(false),
            '^' => Ok(true),
            _ => Err("Invalid tile format"),
        }).collect();
        tiles.map(|tiles|
            Row { tiles: tiles }
        )
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tile in &self.tiles {
            match *tile {
                false => try!(f.write_str(".")),
                true => try!(f.write_str("^")),
            }
        }
        Ok(())
    }
}

impl Row {
    /// Number of tiles
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Number of safe tiles
    fn num_safe_tiles(&self) -> usize {
        self.tiles.iter().filter(|t| !**t).count()
    }
}


/// Iterator that yiels consecutive rows
pub struct RowIter {
    row: Row,
}

impl Iterator for RowIter {
    type Item = Row;

    fn next(&mut self) -> Option<Row> {
        let row = self.row.clone();
        self.row = Row {
            tiles: iter::once(&false)
                .chain(self.row.tiles.iter())
                .chain(iter::once(&false))
                .tuple_windows()
                .map(|(&l, _, &r)| (l && !r) || (r && !l))
                .collect(),
        };
        Some(row)
    }
}

impl RowIter {
    /// Create new iterator with the given first row
    fn new(row: Row) -> RowIter {
        RowIter { row: row }
    }
}


fn main() {
    let first_row: Row = include_str!("day18.txt").parse().unwrap();
    let mut it = RowIter::new(first_row.clone());
    let num_safe_tiles = (0..40).fold(0, |sum, _| sum + it.next().unwrap().num_safe_tiles());
    println!("Number of safe tiles (40 rows): {}", num_safe_tiles);
    let mut it = RowIter::new(first_row);
    let num_safe_tiles = (0..400000).fold(0, |sum, _| sum + it.next().unwrap().num_safe_tiles());
    println!("Number of safe tiles (400k rows): {}", num_safe_tiles);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example5() {
        let mut it = RowIter::new("..^^.".parse().unwrap());
        assert_eq!(it.next(), Some("..^^.".parse().unwrap()));
        assert_eq!(it.next(), Some(".^^^^".parse().unwrap()));
        assert_eq!(it.next(), Some("^^..^".parse().unwrap()));
    }

    #[test]
    fn example10() {
        let mut it = RowIter::new(".^^.^.^^^^".parse().unwrap());
        assert_eq!(it.next(), Some(".^^.^.^^^^".parse().unwrap()));
        assert_eq!(it.next(), Some("^^^...^..^".parse().unwrap()));
        assert_eq!(it.next(), Some("^.^^.^.^^.".parse().unwrap()));
        assert_eq!(it.next(), Some("..^^...^^^".parse().unwrap()));
        assert_eq!(it.next(), Some(".^^^^.^^.^".parse().unwrap()));
        assert_eq!(it.next(), Some("^^..^.^^..".parse().unwrap()));
        assert_eq!(it.next(), Some("^^^^..^^^.".parse().unwrap()));
        assert_eq!(it.next(), Some("^..^^^^.^^".parse().unwrap()));
        assert_eq!(it.next(), Some(".^^^..^.^^".parse().unwrap()));
        assert_eq!(it.next(), Some("^^.^^^..^^".parse().unwrap()));
    }
}
