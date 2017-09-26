// https://twitter.com/ericwastl/status/912192839542550530

#[macro_use]
extern crate nom;

use std::cmp;
use std::fmt::{self, Write};
use std::str::{self, FromStr};
use nom::{space, digit};


/// An immediate or register value
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Immediate(i32),
    Register(u8),
}

impl Value {
    /// Get value
    fn get(&self, regs: &[i32]) -> i32 {
        match *self {
            Value::Immediate(x) => x,
            Value::Register(x) => regs[x as usize],
        }
    }
}


/// An assembunny instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Cpy(Value, Value),
    Inc(u8),
    Dec(u8),
    Jnz(Value, Value),
    Out(Value),
}

impl FromStr for Instruction {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Instruction, nom::ErrorKind> {
        named!(register<&str, u8>, alt!(
            value!(0, tag!("a")) |
            value!(1, tag!("b")) |
            value!(2, tag!("c")) |
            value!(3, tag!("d"))
        ));
        named!(value<&str, Value>, alt!(
            do_parse!(
                sig: opt!(tag!("-")) >>
                x: map_res!(digit, i32::from_str) >>
                (Value::Immediate(if sig.is_some() { -x } else { x }))
            ) |
            do_parse!(
                r: register >>
                (Value::Register(r))
            )
        ));
        alt!(s,
            do_parse!(
                tag!("cpy") >> space >>
                x: value >> space >>
                y: value >>
                (Instruction::Cpy(x, y))
            ) |
            do_parse!(
                tag!("inc") >> space >>
                x: register >>
                (Instruction::Inc(x))
            ) |
            do_parse!(
                tag!("dec") >> space >>
                x: register >>
                (Instruction::Dec(x))
            ) |
            do_parse!(
                tag!("jnz") >> space >>
                x: value >> space >>
                y: value >>
                (Instruction::Jnz(x, y))
            ) |
            do_parse!(
                tag!("out") >> space >>
                x: value >>
                (Instruction::Out(x))
            )
        ).to_result()
    }
}

impl Instruction {
    /// Parse a multiline-text to a vector of instructions
    fn parse(s: &str) -> Result<Vec<Instruction>, nom::ErrorKind> {
        s.lines().map(|line| line.parse()).collect()
    }
}


/// The assembunny CPU
#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    instructions: Vec<Instruction>,
    ip: usize,
    regs: [i32; 4],
    output: String,
}

impl Cpu {
    /// Create new CPU
    fn new(input: &str) -> Result<Cpu, nom::ErrorKind> {
        Instruction::parse(input).map(|instructions|
            Cpu { instructions: instructions, ip: 0, regs: [0; 4], output: String::new() }
        )
    }

    /// Detect addition loop. Returns a tuple with related registers.
    /// 0: first summand, gets result, 1: second summand, gets zeroed
    fn detect_add_loop(&self, ip: usize) -> Option<(u8, u8)> {
        use Instruction::*;
        use Value::*;
        if ip >= 2 {
            match (self.instructions[ip-2], self.instructions[ip-1], self.instructions[ip]) {
                (Inc(tr), Dec(cr1), Jnz(Register(cr2), Immediate(-2))) if cr1==cr2 => return Some((tr, cr1)),
                (Dec(cr1), Inc(tr), Jnz(Register(cr2), Immediate(-2))) if cr1==cr2 => return Some((tr, cr1)),
                _ => (),
            }
        }
        None
    }

    /// Detect multiplication loop. Returns a tuple with related registers.
    /// 0: gets increased by result, 1: first factor, 2: second factor, gets zeroed, 3: gets zeroed
    fn detect_mul_loop(&self, ip: usize) -> Option<(u8, u8, u8, u8)> {
        use Instruction::*;
        use Value::*;
        if ip >= 5 {
            match (self.instructions[ip-5], self.detect_add_loop(ip-2), self.instructions[ip-1], self.instructions[ip]) {
                (Cpy(Register(ar), Register(cr1)), Some((tr, cr2)), Dec(br1), Jnz(Register(br2), Immediate(-5))) if cr1==cr2 && br1==br2 => return Some((tr, ar, br1, cr1)),
                _ => (),
            }
        }
        None
    }

    /// Step program. Returns true if done
    fn step(&mut self) -> bool {
        if self.ip >= self.instructions.len() {
            true
        } else {
            let ip = self.ip;
            let ins = self.instructions[ip];
            match ins {
                Instruction::Cpy(_, Value::Immediate(_)) => { /* ignore invalid cpy */ },
                Instruction::Cpy(ref v, Value::Register(y)) => self.regs[y as usize] = v.get(&self.regs),
                Instruction::Inc(x) => self.regs[x as usize] += 1,
                Instruction::Dec(x) => self.regs[x as usize] -= 1,
                Instruction::Jnz(ref v1, ref v2) => {
                    if v1.get(&self.regs) != 0 {
                        if let Some((x, y, z, zz)) = self.detect_mul_loop(ip) {
                            // Optimized multiplication loop
                            self.regs[x as usize] += self.regs[y as usize] * self.regs[z as usize];
                            self.regs[z as usize] = 0;
                            self.regs[zz as usize] = 0;
                            self.ip += 1;
                        } else if let Some((x, y)) = self.detect_add_loop(ip) {
                            // Optimized addition loop
                            self.regs[x as usize] += self.regs[y as usize];
                            self.regs[y as usize] = 0;
                            self.ip += 1;
                        } else {
                            // Unoptimized loop
                            self.ip = (self.ip as i32 + v2.get(&self.regs)) as usize;
                        }
                    } else {
                        self.ip += 1
                    }
                },
                Instruction::Out(ref v) => self.output.push(v.get(&self.regs) as u8 as char),
            }
            match self.instructions[ip] {
                Instruction::Jnz(_, _) => (),
                _ => self.ip += 1,
            }
            false
        }
    }

    /// Run program
    fn run(&mut self) {
        while !self.step() { }
    }
}


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
    let mut cpu = Cpu::new(include_str!("bonuschallenge.txt")).unwrap();
    cpu.run();
    let operations = Operation::parse(&cpu.output).unwrap();
    let mut display = Display::new(50, 6);
    display.run(&operations);
    println!("{}", display);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_parsing() {
        assert_eq!("inc a".parse(), Ok(Instruction::Inc(0)));
        assert_eq!(Instruction::parse("cpy 41 a\ninc b\ndec c\njnz d 2\njnz 1 -2"), Ok(vec![
            Instruction::Cpy(Value::Immediate(41), Value::Register(0)),
            Instruction::Inc(1),
            Instruction::Dec(2),
            Instruction::Jnz(Value::Register(3), Value::Immediate(2)),
            Instruction::Jnz(Value::Immediate(1), Value::Immediate(-2)),
        ]));
    }

    #[test]
    fn cpu_parsing_real() {
        let cpu = Cpu::new(include_str!("bonuschallenge.txt")).unwrap();
        assert_eq!(cpu.instructions.len(), 1236);
    }

    #[test]
    fn cpu_running() {
        let mut cpu = Cpu::new("cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a").unwrap();
        cpu.run();
        assert_eq!(cpu.regs[0], 42);
    }

    #[test]
    fn display_parsing() {
        assert_eq!("rect 12x34".parse(), Ok(Operation::Rect { width: 12, height: 34 }));
        assert_eq!("rotate row y=23 by 45".parse(), Ok(Operation::RotateRow { row: 23, count: 45 }));
        assert_eq!("rotate column x=34 by 56".parse(), Ok(Operation::RotateColumn { column: 34, count: 56 }));
    }

    #[test]
    fn display_operations() {
        let mut display = Display::new(7, 3);
        assert_eq!(format!("{}", display), ".......\n.......\n.......\n");
        display.execute(&Operation::Rect { width: 3, height: 2 });
        assert_eq!(format!("{}", display), "###....\n###....\n.......\n");
        display.execute(&Operation::RotateColumn { column: 1, count: 1 });
        assert_eq!(format!("{}", display), "#.#....\n###....\n.#.....\n");
        display.execute(&Operation::RotateRow { row: 0, count: 4 });
        assert_eq!(format!("{}", display), "....#.#\n###....\n.#.....\n");
        display.execute(&Operation::RotateColumn { column: 1, count: 1 });
        assert_eq!(format!("{}", display), ".#..#.#\n#.#....\n.#.....\n");
    }
}
