#[macro_use]
extern crate nom;

use std::str::FromStr;
use nom::space;


#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    avail: u32,
    percent: u32,
}

impl FromStr for Node {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Node, nom::ErrorKind> {
        complete!(s, do_parse!(
            tag!("/dev/grid/node-x") >>
            x: map_res!(nom::digit, FromStr::from_str) >>
            tag!("-y") >>
            y: map_res!(nom::digit, FromStr::from_str) >>
            space >>
            size: map_res!(nom::digit, FromStr::from_str) >>
            tag!("T") >>
            space >>
            used: map_res!(nom::digit, FromStr::from_str) >>
            tag!("T") >>
            space >>
            avail: map_res!(nom::digit, FromStr::from_str) >>
            tag!("T") >>
            space >>
            percent: map_res!(nom::digit, FromStr::from_str) >>
            tag!("%") >>
            (Node { x: x, y: y, size: size, used: used, avail: avail, percent: percent })
        )).to_result()
    }
}

impl Node {
    /// Parse a multiline-text to a vector of nodes
    fn parse(s: &str) -> Result<Vec<Node>, nom::ErrorKind> {
        s.lines().skip(2).map(|line| line.parse()).collect()
    }
}


/// Count viable pairs of nodes
fn count_viable_node_pairs(nodes: &[Node]) -> usize {
    nodes.iter().filter(|a| a.used > 0).map(|a| {
        nodes.iter().filter(|b| a.used < b.avail).count()
    }).sum()
}


fn main() {
    let nodes = Node::parse(include_str!("day22.txt")).unwrap();
    println!("Viable pairs of nodes: {}", count_viable_node_pairs(&nodes));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("/dev/grid/node-x0-y0     89T   65T    24T   73%".parse(),
            Ok(Node { x: 0, y: 0, size: 89, used: 65, avail: 24, percent: 73 }));
    }
}
