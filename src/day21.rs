#[macro_use]
extern crate nom;

use std::str::FromStr;


/// One step of the weird password scrambling
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Swap(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateAtLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Instruction {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Instruction, nom::ErrorKind> {
        complete!(s, alt!(
            do_parse!(
                tag!("swap position ") >>
                x: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" with position ") >>
                y: map_res!(nom::digit, FromStr::from_str) >>
                (Instruction::Swap(x, y))
            ) |
            do_parse!(
                tag!("swap letter ") >>
                x: take!(1) >>
                tag!(" with letter ") >>
                y: take!(1) >>
                (Instruction::SwapLetter(x.chars().next().unwrap(), y.chars().next().unwrap()))
            ) |
            do_parse!(
                tag!("rotate left ") >>
                x: alt!(
                    value!(1, tag!("1 step")) |
                    do_parse!(x: map_res!(nom::digit, FromStr::from_str) >> tag!(" steps") >> (x))
                ) >>
                (Instruction::RotateLeft(x))
            ) |
            do_parse!(
                tag!("rotate right ") >>
                x: alt!(
                    value!(1, tag!("1 step")) |
                    do_parse!(x: map_res!(nom::digit, FromStr::from_str) >> tag!(" steps") >> (x))
                ) >>
                (Instruction::RotateRight(x))
            ) |
            do_parse!(
                tag!("rotate based on position of letter ") >>
                x: take!(1) >>
                (Instruction::RotateAtLetter(x.chars().next().unwrap()))
            ) |
            do_parse!(
                tag!("reverse positions ") >>
                x: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" through ") >>
                y: map_res!(nom::digit, FromStr::from_str) >>
                (Instruction::Reverse(x, y))
            ) |
            do_parse!(
                tag!("move position ") >>
                x: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" to position ") >>
                y: map_res!(nom::digit, FromStr::from_str) >>
                (Instruction::Move(x, y))
            )
        )).to_result()
    }
}

impl Instruction {
    /// Parse a multiline-text to a vector of instructions
    fn parse(s: &str) -> Result<Vec<Instruction>, nom::ErrorKind> {
        s.lines().map(|line| line.parse()).collect()
    }

    /// Apply instruction to a given string
    fn apply(&self, s: &mut String) {
        let mut bytes = unsafe { s.as_mut_vec() };
        match *self {
            Instruction::Swap(x, y) => {
                bytes.swap(x, y);
            },
            Instruction::SwapLetter(x, y) => {
                for ch in bytes {
                    if *ch == x as u8 { *ch = y as u8 }
                    else if *ch == y as u8 { *ch = x as u8 }
                }
            },
            Instruction::RotateLeft(x) => {
                let i = x % bytes.len();
                let rotated: Vec<u8> = bytes[i..].iter().chain(bytes[..i].iter()).cloned().collect();
                *bytes = rotated;
            },
            Instruction::RotateRight(x) => {
                let i = bytes.len() - (x % bytes.len());
                let rotated: Vec<u8> = bytes[i..].iter().chain(bytes[..i].iter()).cloned().collect();
                *bytes = rotated;
            },
            Instruction::RotateAtLetter(x) => {
                let i = bytes.iter().position(|ch| *ch == x as u8).unwrap();
                let i = if i >= 4 { i + 2 } else { i + 1 };
                let i = bytes.len() - (i % bytes.len());
                let rotated: Vec<u8> = bytes[i..].iter().chain(bytes[..i].iter()).cloned().collect();
                *bytes = rotated;
            },
            Instruction::Reverse(mut x, mut y) => {
                while y > x {
                    bytes.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            },
            Instruction::Move(x, y) => {
                let ch = bytes.remove(x);
                bytes.insert(y, ch);
            },
        }
    }
}


/// A weird password scrambler
#[derive(Debug)]
pub struct Scrambler {
    instructions: Vec<Instruction>,
}

impl Scrambler {
    /// Create new scrambler using the given instructions
    fn new(s: &str) -> Result<Scrambler, nom::ErrorKind> {
        Instruction::parse(s).map(|instructions|
            Scrambler { instructions: instructions }
        )
    }

    /// Scramble the given password
    fn scramble(&self, s: &str) -> String {
        self.instructions.iter().fold(s.to_owned(), |mut res, ins| {
            ins.apply(&mut res);
            res
        })
    }
}


fn main() {
    let scrambler = Scrambler::new(include_str!("day21.txt")).unwrap();
    println!("Scrambled password: {}", scrambler.scramble("abcdefgh"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("swap position 4 with position 0".parse(), Ok(Instruction::Swap(4, 0)));
        assert_eq!("swap letter d with letter b".parse(), Ok(Instruction::SwapLetter('d', 'b')));
        assert_eq!("reverse positions 0 through 4".parse(), Ok(Instruction::Reverse(0, 4)));
        assert_eq!("rotate left 1 step".parse(), Ok(Instruction::RotateLeft(1)));
        assert_eq!("move position 1 to position 4".parse(), Ok(Instruction::Move(1, 4)));
        assert_eq!("move position 3 to position 0".parse(), Ok(Instruction::Move(3, 0)));
        assert_eq!("rotate based on position of letter b".parse(), Ok(Instruction::RotateAtLetter('b')));
        assert_eq!("rotate based on position of letter d".parse(), Ok(Instruction::RotateAtLetter('d')));
    }

    #[test]
    fn instructions() {
        let mut s = "abcde".to_owned();
        Instruction::Swap(4, 0).apply(&mut s);
        assert_eq!(s, "ebcda");
        Instruction::SwapLetter('d', 'b').apply(&mut s);
        assert_eq!(s, "edcba");
        Instruction::Reverse(0, 4).apply(&mut s);
        assert_eq!(s, "abcde");
        Instruction::RotateLeft(1).apply(&mut s);
        assert_eq!(s, "bcdea");
        Instruction::Move(1, 4).apply(&mut s);
        assert_eq!(s, "bdeac");
        Instruction::Move(3, 0).apply(&mut s);
        assert_eq!(s, "abdec");
        Instruction::RotateAtLetter('b').apply(&mut s);
        assert_eq!(s, "ecabd");
        Instruction::RotateAtLetter('d').apply(&mut s);
        assert_eq!(s, "decab");
    }

    #[test]
    fn scrambling() {
        let scrambler = Scrambler::new("swap position 4 with position 0\nswap letter d with letter b\nreverse positions 0 through 4\nrotate left 1 step\nmove position 1 to position 4\nmove position 3 to position 0\nrotate based on position of letter b\nrotate based on position of letter d").unwrap();
        assert_eq!(scrambler.scramble("abcde"), "decab");
    }
}
