extern crate itertools;

use std::str::FromStr;
use itertools::Itertools;

/// A triangle, specified by the side lengths
#[derive(Debug, PartialEq, Eq)]
pub struct Triangle {
    la: u32,
    lb: u32,
    lc: u32,
}

impl Triangle {
    /// Parse a text with triangle lengths (3 numbers per line). Panics on parsing errors.
    fn parse(s: &str) -> Vec<Triangle> {
        s.lines().map(|line| {
            let mut it = line.split_whitespace().map(u32::from_str);
            Triangle {
                la: it.next().unwrap().unwrap(),
                lb: it.next().unwrap().unwrap(),
                lc: it.next().unwrap().unwrap(),
            }
        }).collect()
    }

    /// Vertically parse a text with triangle lengths (3 numbers per column)
    fn parse_vertical(s: &str) -> Vec<Triangle> {
        s.lines().tuples().flat_map(|(line1, line2, line3)| {
            let mut it1 = line1.split_whitespace().map(u32::from_str);
            let mut it2 = line2.split_whitespace().map(u32::from_str);
            let mut it3 = line3.split_whitespace().map(u32::from_str);
            (0..3).map(|_| Triangle {
                la: it1.next().unwrap().unwrap(),
                lb: it2.next().unwrap().unwrap(),
                lc: it3.next().unwrap().unwrap(),
            }).collect::<Vec<_>>()
        }).collect()
    }

    /// True if the triangle is valid (i.e. any side length plus any other side length is
    /// greater than the other length)
    fn is_valid(&self) -> bool {
        self.la + self.lb > self.lc &&
        self.la + self.lc > self.lb &&
        self.lb + self.lc > self.la
    }
}

fn main() {
    const INPUT: &'static str = include_str!("day03.txt");
    let num_valid = Triangle::parse(INPUT).iter().filter(|t| t.is_valid()).count();
    println!("Number of valid triangles: {}", num_valid);
    let num_valid = Triangle::parse_vertical(INPUT).iter().filter(|t| t.is_valid()).count();
    println!("Number of valid vertical triangles: {}", num_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let triangles = Triangle::parse("5 10 25\n10 20 25");
        assert_eq!(triangles[0], Triangle { la:  5, lb: 10, lc: 25 });
        assert_eq!(triangles[1], Triangle { la: 10, lb: 20, lc: 25 });
    }

    #[test]
    fn parsing_vertically() {
        let triangles = Triangle::parse_vertical("101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603");
        assert_eq!(triangles[0], Triangle { la: 101, lb: 102, lc: 103 });
        assert_eq!(triangles[1], Triangle { la: 301, lb: 302, lc: 303 });
        assert_eq!(triangles[2], Triangle { la: 501, lb: 502, lc: 503 });
        assert_eq!(triangles[3], Triangle { la: 201, lb: 202, lc: 203 });
        assert_eq!(triangles[4], Triangle { la: 401, lb: 402, lc: 403 });
        assert_eq!(triangles[5], Triangle { la: 601, lb: 602, lc: 603 });
    }

    #[test]
    fn validating() {
        let triangles = Triangle::parse("5 10 25\n10 20 25");
        assert!(!triangles[0].is_valid());
        assert!( triangles[1].is_valid());
    }
}
