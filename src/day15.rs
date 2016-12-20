#[macro_use]
extern crate nom;

use std::str::FromStr;
use nom::digit;


/// A rotating disc
#[derive(Debug, PartialEq, Eq)]
pub struct Disc {
    num: u32,
    positions: u32,
    offset: u32,
}

impl FromStr for Disc {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Disc, nom::ErrorKind> {
        do_parse!(s,
            tag!("Disc #") >>
            num: map_res!(digit, FromStr::from_str) >>
            tag!(" has ") >>
            positions: map_res!(digit, FromStr::from_str) >>
            tag!(" positions; at time=0, it is at position ") >>
            offset: map_res!(digit, FromStr::from_str) >>
            tag!(".") >>
            (Disc { num: num, positions: positions, offset: offset })
        ).to_result()
    }
}

impl Disc {
    /// Parse a multiline-text to a vector of discs
    fn parse(s: &str) -> Result<Vec<Disc>, nom::ErrorKind> {
        s.lines().map(|line| line.parse()).collect()
    }

    /// Determine if the sphere can pass at the given time
    fn sphere_can_pass(&self, t: u32) -> bool {
        (t + self.num + self.offset) % self.positions == 0
    }
}


/// Find earliest time at which a sphere can pass through all discs
pub fn time_at_which_sphere_can_pass(discs: &[Disc]) -> u32 {
    (0..).find(|&t|
        discs.iter().all(|disc|
            disc.sphere_can_pass(t)
        )
    ).unwrap()
}

fn main() {
    let mut discs = Disc::parse(include_str!("day15.txt")).unwrap();
    println!("Time to press the button to get a capsule: {}", time_at_which_sphere_can_pass(&discs));
    let extra_disc = Disc { num: discs.len() as u32 + 1, positions: 11, offset: 0 };
    discs.push(extra_disc);
    println!("Time to press the button w/extra disc: {}", time_at_which_sphere_can_pass(&discs));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("Disc #1 has 5 positions; at time=0, it is at position 4.".parse(), Ok(Disc { num: 1, positions: 5, offset: 4 }));
        assert_eq!("Disc #2 has 2 positions; at time=0, it is at position 1.".parse(), Ok(Disc { num: 2, positions: 2, offset: 1 }));
    }

    #[test]
    fn name() {
        let discs = Disc::parse("Disc #1 has 5 positions; at time=0, it is at position 4.\nDisc #2 has 2 positions; at time=0, it is at position 1.").unwrap();
        assert!( discs[0].sphere_can_pass(0));
        assert!(!discs[1].sphere_can_pass(0));
        assert!( discs[0].sphere_can_pass(5));
        assert!( discs[1].sphere_can_pass(5));
        assert_eq!(time_at_which_sphere_can_pass(&discs), 5);
    }
}
