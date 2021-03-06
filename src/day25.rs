extern crate itertools;
#[macro_use]
extern crate nom;

use std::str::FromStr;
use itertools::Itertools;
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
    output: Vec<i32>,
}

impl Cpu {
    /// Create new CPU
    fn new(input: &str) -> Result<Cpu, nom::ErrorKind> {
        Instruction::parse(input).map(|instructions|
            Cpu { instructions: instructions, ip: 0, regs: [0; 4], output: vec![] }
        )
    }

    /// Reset CPU
    fn reset(&mut self) {
        self.ip = 0;
        for reg in self.regs.iter_mut() { *reg = 0; }
        self.output = vec![];
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
                Instruction::Out(ref v) => self.output.push(v.get(&self.regs)),
            }
            match self.instructions[ip] {
                Instruction::Jnz(_, _) => (),
                _ => self.ip += 1,
            }
            false
        }
    }

    /// Run program
    #[allow(dead_code)]
    fn run(&mut self) {
        while !self.step() { }
    }

    /// Run until output starts to repeat (by detecting the unconditional loop jump)
    fn run_until_repeating(&mut self) {
        while !self.step() {
            if let Instruction::Jnz(Value::Immediate(x), Value::Immediate(y)) = self.instructions[self.ip] {
                if x != 0 && y < -20 {
                    break;
                }
            }
        }
    }
}


fn main() {
    let mut cpu = Cpu::new(include_str!("day25.txt")).unwrap();
    let mut a = 0;
    loop {
        cpu.reset();
        cpu.regs[0] = a;
        cpu.run_until_repeating();
        if cpu.output.iter().tuples().all(|(&a, &b)| a==0 && b==1) {
            break;
        }
        a += 1;
    }
    println!("Lowest possible integer to send clock signal: {}", a);
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
    fn parsing_real() {
        let cpu = Cpu::new(include_str!("day25.txt")).unwrap();
        assert_eq!(cpu.instructions.len(), 30);
    }

    #[test]
    fn running() {
        let mut cpu = Cpu::new("cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a").unwrap();
        cpu.run();
        assert_eq!(cpu.regs[0], 42);
    }
}
