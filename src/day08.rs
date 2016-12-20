#[macro_use]
extern crate nom;

use std::cmp;
use std::fmt::{self, Write};
use std::str::{self, FromStr};


/// An operation is an instruction to modify pixels on a display
#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    /// Draw rectangle in top left corner
    Rect { width: usize, height: usize },
    /// Rotate pixels right in a row
    RotateRow { row: usize, count: usize },
    /// Rotate pixels down in a column
    RotateColumn { column: usize, count: usize },
}

impl FromStr for Operation {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Operation, nom::ErrorKind> {
        complete!(s, alt!(
            do_parse!(
                tag!("rect ") >>
                width: map_res!(nom::digit, FromStr::from_str) >>
                tag!("x") >>
                height: map_res!(nom::digit, FromStr::from_str) >>
                (Operation::Rect { width: width, height: height })
            ) |
            do_parse!(
                tag!("rotate row y=") >>
                row: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" by ") >>
                count: map_res!(nom::digit, FromStr::from_str) >>
                (Operation::RotateRow { row: row, count: count })
            ) |
            do_parse!(
                tag!("rotate column x=") >>
                column: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" by ") >>
                count: map_res!(nom::digit, FromStr::from_str) >>
                (Operation::RotateColumn { column: column, count: count })
            )
        )).to_result()
    }
}

impl Operation {
    /// Parse a multiline-text to a vector of operations
    fn parse(s: &str) -> Result<Vec<Operation>, nom::ErrorKind> {
        s.lines().map(|line| line.parse()).collect()
    }
}


/// A rectangular display with monochromatic pixels
#[derive(Debug, PartialEq, Eq)]
pub struct Display {
    pixels: Vec<Vec<bool>>,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.pixels.len() {
            for x in 0..self.pixels[y].len() {
                match self.pixels[y][x] {
                    false => try!(f.write_char('.')),
                    true => try!(f.write_char('#')),
                }
            }
            try!(f.write_str("\n"));
        }
        Ok(())
    }
}

impl Display {
    /// Create a new display with the given number of pixels in width and height
    fn new(width: usize, height: usize) -> Display {
        Display {
            pixels: vec![vec![false; width]; height],
        }
    }

    /// Count lit pixels (voltage check)
    fn count_lit(&self) -> usize {
        self.pixels.iter().map(|row| row.iter().filter(|px| **px).count()).sum()
    }

    /// Draw a rectangle
    fn rect(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for dy in 0..h {
            for dx in 0..w {
                let yy = cmp::min(self.pixels.len() - 1, y + dy);
                let xx = cmp::min(self.pixels[yy].len() - 1, x + dx);
                self.pixels[yy][xx] = true;
            }
        }
    }

    /// Rotate a row
    fn rotate_row(&mut self, y: usize, c: usize) {
        let ofs = self.pixels[y].len() - c;
        self.pixels[y] = self.pixels[y].iter().skip(ofs).chain( self.pixels[y].iter().take(ofs) ).map(|px| *px).collect();
    }

    /// Rotate a column
    fn rotate_column(&mut self, x: usize, c: usize) {
        let ofs = self.pixels.len() - c;
        let pixels: Vec<bool> = self.pixels.iter().map(|row| row[x]).collect();
        for (y, px) in pixels.iter().skip(ofs).chain( pixels.iter().take(ofs) ).map(|px| *px).enumerate() {
            self.pixels[y][x] = px;
        }
    }

    /// Execute an operation
    fn execute(&mut self, op: &Operation) {
        match *op {
            Operation::Rect { width: w, height: h } => self.rect(0, 0, w, h),
            Operation::RotateRow { row: y, count: c } => self.rotate_row(y, c),
            Operation::RotateColumn { column: x, count: c } => self.rotate_column(x, c),
        }
    }

    /// Execute a list of operations
    fn run(&mut self, ops: &[Operation]) {
        for op in ops {
            self.execute(op);
        }
    }
}


fn main() {
    let operations = Operation::parse(include_str!("day08.txt")).unwrap();
    let mut display = Display::new(50, 6);
    display.run(&operations);
    println!("Lit pixels on display: {}", display.count_lit());
    println!("{}", display);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("rect 12x34".parse(), Ok(Operation::Rect { width: 12, height: 34 }));
        assert_eq!("rotate row y=23 by 45".parse(), Ok(Operation::RotateRow { row: 23, count: 45 }));
        assert_eq!("rotate column x=34 by 56".parse(), Ok(Operation::RotateColumn { column: 34, count: 56 }));
    }

    #[test]
    fn display_operations() {
        let mut display = Display::new(7, 3);
        assert_eq!(format!("{}", display), ".......\n.......\n.......\n");
        assert_eq!(display.count_lit(), 0);
        display.execute(&Operation::Rect { width: 3, height: 2 });
        assert_eq!(format!("{}", display), "###....\n###....\n.......\n");
        assert_eq!(display.count_lit(), 6);
        display.execute(&Operation::RotateColumn { column: 1, count: 1 });
        assert_eq!(format!("{}", display), "#.#....\n###....\n.#.....\n");
        assert_eq!(display.count_lit(), 6);
        display.execute(&Operation::RotateRow { row: 0, count: 4 });
        assert_eq!(format!("{}", display), "....#.#\n###....\n.#.....\n");
        assert_eq!(display.count_lit(), 6);
        display.execute(&Operation::RotateColumn { column: 1, count: 1 });
        assert_eq!(format!("{}", display), ".#..#.#\n#.#....\n.#.....\n");
        assert_eq!(display.count_lit(), 6);
    }
}
