use std::iter::FromIterator;

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

/// A generic keypad
#[derive(Debug, PartialEq, Eq)]
pub struct Keypad<'a>(&'a [&'a [Option<char>]]);

impl<'a> Keypad<'a> {
    /// Create a 3x3 keypad
    fn new_3x3() -> Keypad<'static> {
        const BUTTONS_3X3: &'static [&'static [Option<char>]] = &[
            &[Some('1'), Some('2'), Some('3')],
            &[Some('4'), Some('5'), Some('6')],
            &[Some('7'), Some('8'), Some('9')]
        ];
        Keypad(BUTTONS_3X3)
    }

    /// Create a bathroom keypad
    fn new_bathroom() -> Keypad<'static> {
        const BUTTONS_BATHROOM: &'static [&'static [Option<char>]] = &[
            &[None,      None,      Some('1'), None,      None     ],
            &[None,      Some('2'), Some('3'), Some('4'), None     ],
            &[Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            &[None,      Some('A'), Some('B'), Some('C'), None     ],
            &[None,      None,      Some('D'), None,      None     ],
        ];
        Keypad(BUTTONS_BATHROOM)
    }

    /// Find coordinates of given button. Button must not appear more than once
    fn find(&self, btn: char) -> Option<(usize, usize)> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if *ch == Some(btn) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Return the button in the given direction
    fn step(&self, btn: char, m: &Move) -> Option<char> {
        self.find(btn).and_then(|(mut x, mut y)| {
            match *m {
                Move::Up => if y > 0 { y -= 1; },
                Move::Down => if y < self.0.len() - 1 { y += 1; },
                Move::Left => if x > 0 { x -= 1; },
                Move::Right => if x < self.0[0].len() - 1 { x += 1; },
            }
            self.0[y][x]
        })
    }

    /// Return the button after walking the given path
    fn walk(&self, btn: char, moves: &[Move]) -> char {
        moves.iter().fold(btn, |btn, m| self.step(btn, m).unwrap_or(btn))
    }

    /// Return the buttons after walking the given paths
     fn walk_n(&self, mut btn: char, moves: &[Vec<Move>]) -> Vec<char> {
        moves.iter().map(|ms| { btn = self.walk(btn, ms); btn }).collect()
    }

    /// Return the buttons after walking the given paths as a string
    fn walk_n_str(&self, btn: char, moves: &[Vec<Move>]) -> String {
        String::from_iter(self.walk_n(btn, moves).into_iter())
    }
}

fn main() {
    let moves = Move::parse_lines(include_str!("day02.txt"));
    let kp = Keypad::new_3x3();
    println!("Bathroom code: {}", kp.walk_n_str('5', &moves));
    let kp = Keypad::new_bathroom();
    println!("Correct bathroom code: {}", kp.walk_n_str('5', &moves));
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
    fn button_arrangement_3x3() {
        fn assert_4dir(kp: &Keypad, btn: char, up: char, down: char, left: char, right: char) {
            assert_eq!(kp.step(btn, &Move::Up), Some(up));
            assert_eq!(kp.step(btn, &Move::Down), Some(down));
            assert_eq!(kp.step(btn, &Move::Left), Some(left));
            assert_eq!(kp.step(btn, &Move::Right), Some(right));
        }
        let kp = Keypad::new_3x3();
        assert_4dir(&kp, '1', '1', '4', '1', '2');
        assert_4dir(&kp, '3', '3', '6', '2', '3');
        assert_4dir(&kp, '5', '2', '8', '4', '6');
        assert_4dir(&kp, '7', '4', '7', '7', '8');
        assert_4dir(&kp, '9', '6', '9', '8', '9');
    }

    #[test]
    fn walking() {
        let kp = Keypad::new_3x3();
        assert_eq!(kp.walk('5', &Move::parse("ULL")), '1');
        assert_eq!(kp.walk('1', &Move::parse("RRDDD")), '9');
        assert_eq!(kp.walk('9', &Move::parse("LURDL")), '8');
        assert_eq!(kp.walk('8', &Move::parse("UUUUD")), '5');
    }

    #[test]
    fn walking_n() {
        let kp = Keypad::new_3x3();
        assert_eq!(kp.walk_n('5', &Move::parse_lines("ULL\nRRDDD\nLURDL\nUUUUD")), ['1', '9', '8', '5']);
        assert_eq!(kp.walk_n_str('5', &Move::parse_lines("ULL\nRRDDD\nLURDL\nUUUUD")), "1985");
    }

    #[test]
    fn bathroom_walking() {
        let kp = Keypad::new_bathroom();
        assert_eq!(kp.walk('5', &Move::parse("ULL")), '5');
        assert_eq!(kp.walk('5', &Move::parse("RRDDD")), 'D');
        assert_eq!(kp.walk('D', &Move::parse("LURDL")), 'B');
        assert_eq!(kp.walk('B', &Move::parse("UUUUD")), '3');
    }

    #[test]
    fn bathroom_walking_n() {
        let kp = Keypad::new_bathroom();
        assert_eq!(kp.walk_n('5', &Move::parse_lines("ULL\nRRDDD\nLURDL\nUUUUD")), ['5', 'D', 'B', '3']);
        assert_eq!(kp.walk_n_str('5', &Move::parse_lines("ULL\nRRDDD\nLURDL\nUUUUD")), "5DB3");
    }
}
