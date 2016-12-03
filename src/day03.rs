use std::str::FromStr;

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
    fn validating() {
        let triangles = Triangle::parse("5 10 25\n10 20 25");
        assert!(!triangles[0].is_valid());
        assert!( triangles[1].is_valid());
    }
}
