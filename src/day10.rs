#[macro_use]
extern crate nom;

use std::cmp;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

/// Target of a bot can be either another bot or an output box
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Target {
    Bot(u8),
    Output(u8),
}

/// Source of a bot can either be the low or high output of another bot or
/// a value from an input box
#[derive(Debug, PartialEq, Eq)]
pub enum Source {
    Value(u8),
    BotLow(u8),
    BotHigh(u8),
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

/// A bot in the factory
#[derive(Debug, PartialEq, Eq)]
pub struct Bot {
    sources: Vec<Source>,
    target_low: Option<Target>,
    target_high: Option<Target>,
}

impl Bot {
    /// Create a new bot (with nothing to do yet)
    fn new() -> Bot {
        Bot { sources: Vec::new(), target_low: None, target_high: None }
    }
}

/// A factory
#[derive(Debug, PartialEq, Eq)]
pub struct Factory {
    bots: HashMap<u8, Bot>,
    input_values: RefCell<HashMap<u8, Option<Vec<u8>>>>,
}

impl FromStr for Factory {
    type Err = ();

    fn from_str(s: &str) -> Result<Factory, ()> {
        Ok(Factory::from(Instruction::parse(s)))
    }
}

impl<T: AsRef<[Instruction]>> From<T> for Factory {
    fn from(instructions: T) -> Factory {
        let mut bots = HashMap::new();
        for instruction in instructions.as_ref() {
            match instruction {
                &Instruction::ValueToBot(v, b) => {
                    let bot = bots.entry(b).or_insert_with(|| Bot::new());
                    bot.sources.push(Source::Value(v));
                },
                &Instruction::BotToTarget(b, ref tl, ref th) => {
                    {
                        let bot = bots.entry(b).or_insert_with(|| Bot::new());
                        bot.target_low = Some(tl.to_owned());
                        bot.target_high = Some(th.to_owned());
                    }
                    if let &Target::Bot(bl) = tl {
                        let targetbot = bots.entry(bl).or_insert_with(|| Bot::new());
                        targetbot.sources.push(Source::BotLow(b));
                    }
                    if let &Target::Bot(bh) = th {
                        let targetbot = bots.entry(bh).or_insert_with(|| Bot::new());
                        targetbot.sources.push(Source::BotHigh(b));
                    }
                },
            }
        }
        Factory { bots: bots, input_values: RefCell::new(HashMap::new()) }
    }
}

impl Factory {
    /// Find out input values for a given bot
    fn find_input_values_for_bot(&self, bot_id: u8) -> Option<Vec<u8>> {
        self.bots.get(&bot_id).map(|bot| {
            bot.sources.iter().map(|source| {
                match source {
                    &Source::Value(v) => v,
                    &Source::BotLow(b) => *self.input_values_for_bot(b).unwrap().iter().min().unwrap(),
                    &Source::BotHigh(b) => *self.input_values_for_bot(b).unwrap().iter().max().unwrap(),
                }
            }).collect()
        })
    }

    /// Find out input values for a given bot (w/caching)
    fn input_values_for_bot(&self, bot_id: u8) -> Option<Vec<u8>> {
        if let Some(v) = self.input_values.borrow().get(&bot_id) {
            return v.clone();
        }
        let v = self.find_input_values_for_bot(bot_id);
        self.input_values.borrow_mut().insert(bot_id, v.clone());
        v
    }

    /// Find bot that gets the given input values
    fn bot_with_input_values(&self, v1: u8, v2: u8) -> Option<u8> {
        fn search(factory: &Factory, bot_id: u8, v1: u8, v2: u8) -> Option<u8> {
            factory.bots.get(&bot_id).and_then(|bot| {
                factory.input_values_for_bot(bot_id).and_then(|inputs| {
                    if inputs.contains(&v1) && inputs.contains(&v2) {
                        Some(bot_id)
                    } else if inputs.iter().min().iter().any(|&v| *v == v1 || *v == v2) {
                        match bot.target_low {
                            Some(Target::Bot(b)) => search(factory, b, v1, v2),
                            _ => None,
                        }
                    } else if inputs.iter().max().iter().any(|&v| *v == v1 || *v == v2) {
                        match bot.target_high {
                            Some(Target::Bot(b)) => search(factory, b, v1, v2),
                            _ => None,
                        }
                    } else {
                        unreachable!()
                    }
                })
            })
        }
        self.bots.iter().filter(|&(_, bot)| {
            bot.sources.iter().any(|s| s == &Source::Value(v1) || s == &Source::Value(v2))
        }).nth(0).and_then(|(bot_id, _)| {
            search(self, *bot_id, cmp::min(v1, v2), cmp::max(v1, v2))
        })
    }

    /// Find value of output box
    fn value_of_output(&self, output: u8) -> Option<u8> {
        self.bots.iter().filter_map(|(bot_id, bot)| {
            if bot.target_low == Some(Target::Output(output)) {
                self.input_values_for_bot(*bot_id).and_then(|inputs| inputs.iter().min().map(|v| *v))
            } else if bot.target_high == Some(Target::Output(output)) {
                self.input_values_for_bot(*bot_id).and_then(|inputs| inputs.iter().max().map(|v| *v))
            } else {
                None
            }
        }).nth(0)
    }
}

fn main() {
    let factory = Factory::from_str(include_str!("day10.txt")).unwrap();
    println!("Bot responsible for value-61 and value-17: {}", factory.bot_with_input_values(61, 17).unwrap());
    let output_product = factory.value_of_output(0).unwrap() as u32
        * factory.value_of_output(1).unwrap() as u32
        * factory.value_of_output(2).unwrap() as u32;
    println!("Product of outputs 0, 1 and 2: {}", output_product);
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
    fn finding_values() {
        let factory = Factory::from_str("value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives low to output 2 and high to output 0\nvalue 2 goes to bot 2").unwrap();
        assert_eq!(factory.input_values_for_bot(2), Some(vec![5, 2]));
        assert_eq!(factory.bot_with_input_values(5, 2), Some(2));
        assert_eq!(factory.value_of_output(0), Some(5));
        assert_eq!(factory.value_of_output(1), Some(2));
        assert_eq!(factory.value_of_output(2), Some(3));
    }
}
