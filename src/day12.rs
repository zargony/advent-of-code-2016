#[macro_use]
extern crate nom;

use std::str::FromStr;
use nom::{space, digit};

/// An immediate or register value
#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Immediate(u32),
    Register(u8),
}

impl Value {
    /// Get value
    fn get(&self, regs: &[u32]) -> u32 {
        match *self {
            Value::Immediate(x) => x,
            Value::Register(x) => regs[x as usize],
        }
    }
}

/// An assembunny instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Cpy(Value, u8),
    Inc(u8),
    Dec(u8),
    Jnz(Value, isize),
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
                x: map_res!(digit, FromStr::from_str) >>
                (Value::Immediate(x))
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
                y: register >>
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
                sig: opt!(tag!("-")) >>
                y: map_res!(digit, isize::from_str) >>
                (Instruction::Jnz(x, if sig.is_some() { -y } else { y }))
            )
        ).to_result()
    }
}

impl Instruction {
    /// Parse a multiline-text to a vector of instructions
    fn parse(s: &str) -> Vec<Instruction> {
        s.lines().map(|line| Instruction::from_str(line).unwrap()).collect()
    }
}

/// The assembunny CPU
#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    instructions: Vec<Instruction>,
    ip: usize,
    regs: [u32; 4],
}

impl Cpu {
    /// Create new CPU
    fn new(input: &str) -> Cpu {
        Cpu { instructions: Instruction::parse(input), ip: 0, regs: [0; 4] }
    }

    /// Step program. Returns true if done
    fn step(&mut self) -> bool {
        if self.ip >= self.instructions.len() {
            true
        } else {
            let ip = self.ip;
            match self.instructions[ip] {
                Instruction::Cpy(ref v, y) => self.regs[y as usize] = v.get(&self.regs),
                Instruction::Inc(x) => self.regs[x as usize] += 1,
                Instruction::Dec(x) => self.regs[x as usize] -= 1,
                Instruction::Jnz(ref v, y) => if v.get(&self.regs) != 0 {
                    self.ip = (self.ip as isize + y) as usize;
                } else {
                    self.ip += 1
                },
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

fn main() {
    let mut cpu = Cpu::new(include_str!("day12.txt"));
    cpu.run();
    println!("Register a after running: {}", cpu.regs[0]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parsing() {
        assert_eq!(Instruction::from_str("inc a"), Ok(Instruction::Inc(0)));
        assert_eq!(Instruction::parse("cpy 41 a\ninc b\ndec c\njnz d 2\njnz 1 -2"),
            [Instruction::Cpy(Value::Immediate(41), 0), Instruction::Inc(1), Instruction::Dec(2), Instruction::Jnz(Value::Register(3), 2), Instruction::Jnz(Value::Immediate(1), -2)]);
    }

    #[test]
    fn running() {
        let mut cpu = Cpu::new("cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a");
        cpu.run();
        assert_eq!(cpu.regs[0], 42);
    }
}
