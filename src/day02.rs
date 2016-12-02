use std::fmt::Write;

/// Directional move
#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(ch: char) -> Move {
        match ch {
            'U' => Move::Up,
            'D' => Move::Down,
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Illegal move"),
        }
    }
}

impl Move {
    fn parse(s: &str) -> Vec<Move> {
        s.chars().map(|ch| Move::from(ch)).collect()
    }

    fn parse_lines(s: &str) -> Vec<Vec<Move>> {
        s.lines().map(|l| Move::parse(l)).collect()
    }
}

/// A 3x3 keypad
#[derive(Debug, PartialEq, Eq)]
pub struct Keypad;

impl Keypad {
    /// Return the button in the given direction
    fn step(btn: u8, m: &Move) -> u8 {
        match *m {
            Move::Up => if btn > 3 { btn - 3 } else { btn },
            Move::Down => if btn < 7 { btn + 3 } else { btn },
            Move::Left => if (btn - 1) % 3 > 0 { btn - 1 } else { btn },
            Move::Right => if (btn - 1) % 3 < 2 { btn + 1 } else { btn },
        }
    }

    /// Return the button after walking the given path
    fn walk(btn: u8, moves: &[Move]) -> u8 {
        moves.iter().fold(btn, |btn, m| Keypad::step(btn, m))
    }

    /// Return the buttons after walking the given paths
    fn walk_n(mut btn: u8, moves: &[Vec<Move>]) -> Vec<u8> {
        moves.iter().map(|ms| { btn = Keypad::walk(btn, ms); btn }).collect()
    }

    /// Return the buttons after walking the given paths as a string
    fn walk_n_str(btn: u8, moves: &[Vec<Move>]) -> String {
        Keypad::walk_n(btn, moves).iter().fold(String::new(), |mut s, b| {
            s.write_fmt(format_args!("{}", b)).unwrap(); s
        })
    }
}

fn main() {
    let moves = Move::parse_lines(include_str!("day02.txt"));
    println!("Bathroom code: {}", Keypad::walk_n_str(5, &moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Move::parse("ULL"), [Move::Up, Move::Left, Move::Left]);
        assert_eq!(Move::parse("RRDDD"), [Move::Right, Move::Right, Move::Down, Move::Down, Move::Down]);
        assert_eq!(Move::parse("LURDL"), [Move::Left, Move::Up, Move::Right, Move::Down, Move::Left]);
        assert_eq!(Move::parse("UUUUD"), [Move::Up, Move::Up, Move::Up, Move::Up, Move::Down]);
    }

    #[test]
    fn button_arrangement() {
        fn assert_4dir(btn: u8, up: u8, down: u8, left: u8, right: u8) {
            assert_eq!(Keypad::step(btn, &Move::Up), up);
            assert_eq!(Keypad::step(btn, &Move::Down), down);
            assert_eq!(Keypad::step(btn, &Move::Left), left);
            assert_eq!(Keypad::step(btn, &Move::Right), right);
        }
        assert_4dir(1, 1, 4, 1, 2);
        assert_4dir(3, 3, 6, 2, 3);
        assert_4dir(5, 2, 8, 4, 6);
        assert_4dir(7, 4, 7, 7, 8);
        assert_4dir(9, 6, 9, 8, 9);
    }

    #[test]
    fn walking() {
        assert_eq!(Keypad::walk(5, &Move::parse("ULL")), 1);
        assert_eq!(Keypad::walk(1, &Move::parse("RRDDD")), 9);
        assert_eq!(Keypad::walk(9, &Move::parse("LURDL")), 8);
        assert_eq!(Keypad::walk(8, &Move::parse("UUUUD")), 5);
    }

    #[test]
    fn walking_n() {
        assert_eq!(Keypad::walk_n(5, &Move::parse_lines("ULL\nRRDDD\nLURDL\nUUUUD")), [1, 9, 8, 5]);
        assert_eq!(Keypad::walk_n_str(5, &Move::parse_lines("ULL\nRRDDD\nLURDL\nUUUUD")), "1985");
    }
}
