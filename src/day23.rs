#[macro_use]
extern crate nom;

use std::str::FromStr;
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
    Tgl(u8),
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
                tag!("tgl") >> space >>
                x: register >>
                (Instruction::Tgl(x))
            )
        ).to_result()
    }
}

impl Instruction {
    /// Parse a multiline-text to a vector of instructions
    fn parse(s: &str) -> Result<Vec<Instruction>, nom::ErrorKind> {
        s.lines().map(|line| line.parse()).collect()
    }

    /// Returns the toggled instruction (result of being affected by a tgl instruction)
    fn toggle(&self) -> Instruction {
        match *self {
            Instruction::Cpy(v1, v2) => Instruction::Jnz(v1, v2),
            Instruction::Inc(x) => Instruction::Dec(x),
            Instruction::Dec(x) => Instruction::Inc(x),
            Instruction::Jnz(v1, v2) => Instruction::Cpy(v1, v2),
            Instruction::Tgl(x) => Instruction::Inc(x),
        }
    }
}


/// The assembunny CPU
#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    instructions: Vec<Instruction>,
    ip: usize,
    regs: [i32; 4],
}

impl Cpu {
    /// Create new CPU
    fn new(input: &str) -> Result<Cpu, nom::ErrorKind> {
        Instruction::parse(input).map(|instructions|
            Cpu { instructions: instructions, ip: 0, regs: [0; 4] }
        )
    }

    /// Reset CPU
    #[allow(dead_code)]
    fn reset(&mut self) {
        self.ip = 0;
        for reg in self.regs.iter_mut() { *reg = 0; }
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
                Instruction::Jnz(ref v1, ref v2) => if v1.get(&self.regs) != 0 {
                    self.ip = (self.ip as i32 + v2.get(&self.regs)) as usize;
                } else {
                    self.ip += 1
                },
                Instruction::Tgl(x) => {
                    let addr = self.ip + self.regs[x as usize] as usize;
                    if addr < self.instructions.len() {
                        self.instructions[addr] = self.instructions[addr].toggle();
                    }
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
    let mut cpu = Cpu::new(include_str!("day23.txt")).unwrap();
    cpu.regs[0] = 7;
    cpu.run();
    println!("Value to send to the safe: {}", cpu.regs[0]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
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
    fn parsing_new() {
        assert_eq!(Instruction::parse("cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a"), Ok(vec![
            Instruction::Cpy(Value::Immediate(2), Value::Register(0)),
            Instruction::Tgl(0),
            Instruction::Tgl(0),
            Instruction::Tgl(0),
            Instruction::Cpy(Value::Immediate(1), Value::Register(0)),
            Instruction::Dec(0),
            Instruction::Dec(0),
        ]))
    }

    #[test]
    fn running() {
        let mut cpu = Cpu::new("cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a").unwrap();
        cpu.run();
        assert_eq!(cpu.regs[0], 42);
    }

    #[test]
    fn running_new() {
        let mut cpu = Cpu::new("cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a").unwrap();
        cpu.run();
        assert_eq!(cpu.regs[0], 3);
    }
}
