#[macro_use]
extern crate nom;

use std::cmp;
use std::str::FromStr;

/// Target of a bot can be either another bot or an output box
#[derive(Debug, PartialEq, Eq)]
pub enum Target {
    Bot(u8),
    Output(u8),
}

/// Bot instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    ValueToBot(u8, u8),
    BotToTarget(u8, Target, Target),
}

impl FromStr for Instruction {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Instruction, nom::ErrorKind> {

        named!(target<&str, Target>, alt!(
            // bot <b>
            do_parse!(
                tag!("bot ") >>
                b: map_res!(nom::digit, FromStr::from_str) >>
                (Target::Bot(b))
            ) |
            // output <o>
            do_parse!(
                tag!("output ") >>
                o: map_res!(nom::digit, FromStr::from_str) >>
                (Target::Output(o))
            )
        ));

        complete!(s, alt!(
            // value <v> goes to bot <b>
            do_parse!(
                tag!("value ") >>
                v: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" goes to bot ") >>
                b: map_res!(nom::digit, FromStr::from_str) >>
                (Instruction::ValueToBot(v, b))
            ) |
            // bot <b> gives low to <tl> and high to <th>
            do_parse!(
                tag!("bot ") >>
                b: map_res!(nom::digit, FromStr::from_str) >>
                tag!(" gives low to ") >>
                tl: target >>
                tag!(" and high to ") >>
                th: target >>
                (Instruction::BotToTarget(b, tl, th))
            )
        )).to_result()
    }
}

impl Instruction {
    /// Parse a multiline-text to a vector of instructions
    fn parse(s: &str) -> Vec<Instruction> {
        s.lines().map(|line| Instruction::from_str(line).unwrap()).collect()
    }
}

pub fn input_values_for_bot(instructions: &[Instruction], bot: u8) -> Vec<u8> {
    instructions.iter().filter_map(|instruction| {
        match instruction {
            &Instruction::ValueToBot(v, b) if b == bot => Some(v),
            &Instruction::BotToTarget(b, ref tl, _) if tl == &Target::Bot(bot) => {
                Some(*input_values_for_bot(instructions, b).iter().min().unwrap())
            },
            &Instruction::BotToTarget(b, _, ref th) if th == &Target::Bot(bot) => {
                Some(*input_values_for_bot(instructions, b).iter().max().unwrap())
            },
            _ => None,
        }
    }).collect()
}

fn target_bots_for_bot(instructions: &[Instruction], bot: u8) -> (Option<u8>, Option<u8>) {
    instructions.iter().filter_map(|instruction| {
        match instruction {
            &Instruction::BotToTarget(b, ref tl, ref th) if b == bot => Some((
                match tl { &Target::Bot(bl) => Some(bl), _ => None },
                match th { &Target::Bot(bh) => Some(bh), _ => None },
            )),
            _ => None,
        }
    }).nth(0).unwrap()
}

pub fn bot_with_input_values(instructions: &[Instruction], v1: u8, v2: u8) -> Option<u8> {
    let (vl, vh) = (cmp::min(v1, v2), cmp::max(v1, v2));
    instructions.iter().filter_map(|instruction| {
        match instruction {
            &Instruction::ValueToBot(v, b) if v == vl || v == vh => Some(b),
            _ => None,
        }
    }).nth(0).and_then(|mut bot| {
        loop {
            let inputs = input_values_for_bot(instructions, bot);
            match (inputs.contains(&vl), inputs.contains(&vh)) {
                (true, true) => break,
                (true, false) => { bot = target_bots_for_bot(instructions, bot).0.unwrap() },
                (false, true) => { bot = target_bots_for_bot(instructions, bot).1.unwrap() },
                _ => unreachable!(),
            }
        }
        Some(bot)
    })
}

fn main() {
    let instructions = Instruction::parse(include_str!("day10.txt"));
    println!("Bot responsible for value-61 and value-17: {}", bot_with_input_values(&instructions, 61, 17).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parsing() {
        assert_eq!(Instruction::from_str("value 5 goes to bot 2"), Ok(Instruction::ValueToBot(5, 2)));
        assert_eq!(Instruction::from_str("bot 2 gives low to bot 1 and high to bot 0"), Ok(Instruction::BotToTarget(2, Target::Bot(1), Target::Bot(0))));
        assert_eq!(Instruction::from_str("bot 1 gives low to output 1 and high to bot 0"), Ok(Instruction::BotToTarget(1, Target::Output(1), Target::Bot(0))));
    }

    #[test]
    fn finding_input_values() {
        let instructions = Instruction::parse("value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives low to output 2 and high to output 0\nvalue 2 goes to bot 2");
        assert_eq!(input_values_for_bot(&instructions, 2), vec![5, 2]);
        assert_eq!(bot_with_input_values(&instructions, 5, 2), Some(2));
    }
}
