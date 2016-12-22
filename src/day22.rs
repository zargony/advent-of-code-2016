#[macro_use]
extern crate nom;

use std::str::FromStr;
use nom::{space, digit};


#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32,
    avail: u32,
}

impl FromStr for Node {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Node, nom::ErrorKind> {
        complete!(s, do_parse!(
            tag!("/dev/grid/node-x") >>
            x: map_res!(digit, FromStr::from_str) >>
            tag!("-y") >>
            y: map_res!(digit, FromStr::from_str) >>
            space >>
            size: map_res!(digit, FromStr::from_str) >>
            tag!("T") >>
            space >>
            used: map_res!(digit, FromStr::from_str) >>
            tag!("T") >>
            space >>
            avail: map_res!(digit, FromStr::from_str) >>
            tag!("T") >>
            space >>
            digit >>
            tag!("%") >>
            (Node { x: x, y: y, size: size, used: used, avail: avail })
        )).to_result()
    }
}

impl Node {
    /// Parse a multiline-text to a vector of nodes
    fn parse(s: &str) -> Result<Vec<Node>, nom::ErrorKind> {
        s.lines().skip(2).map(|line| line.parse()).collect()
    }
}


/// A rectangular cluster of nodes
pub struct Cluster {
    nodes: Vec<Vec<Node>>,
}

impl Cluster {
    /// Create new cluster using the given node descriptions
    fn new(s: &str) -> Result<Cluster, nom::ErrorKind> {
        let mut nodes = vec![];
        for node in try!(Node::parse(s)) {
            if node.x == nodes.len() { nodes.push(vec![]); }
            if node.x >= nodes.len() { panic!("Invalid node order"); }
            if node.y == nodes[node.x].len() { nodes[node.x].push(node); } else { panic!("Invalid node order"); }
        }
        Ok(Cluster { nodes: nodes })
    }

    /// Width
    #[allow(dead_code)]
    #[inline]
    fn width(&self) -> usize { self.nodes.len() }

    /// Height
    #[allow(dead_code)]
    #[inline]
    fn height(&self) -> usize { self.nodes[0].len() }

    /// Count viable pairs of nodes
    fn count_viable_node_pairs(&self) -> usize {
        self.nodes.iter().flat_map(|col| col.iter()).filter(|a| a.used > 0).map(|a| {
            self.nodes.iter().flat_map(|col| col.iter()).filter(|b| a.used < b.avail).count()
        }).sum()
    }
}


fn main() {
    let cluster = Cluster::new(include_str!("day22.txt")).unwrap();
    println!("Viable pairs of nodes: {}", cluster.count_viable_node_pairs());
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = "root@ebhq-gridcenter# df -h\r
Filesystem            Size  Used  Avail  Use%\r
/dev/grid/node-x0-y0   10T    8T     2T   80%\r
/dev/grid/node-x0-y1   11T    6T     5T   54%\r
/dev/grid/node-x0-y2   32T   28T     4T   87%\r
/dev/grid/node-x1-y0    9T    7T     2T   77%\r
/dev/grid/node-x1-y1    8T    0T     8T    0%\r
/dev/grid/node-x1-y2   11T    7T     4T   63%\r
/dev/grid/node-x2-y0   10T    6T     4T   60%\r
/dev/grid/node-x2-y1    9T    8T     1T   88%\r
/dev/grid/node-x2-y2    9T    6T     3T   66%";

    #[test]
    fn parsing() {
        assert_eq!("/dev/grid/node-x0-y0     89T   65T    24T   73%".parse(),
            Ok(Node { x: 0, y: 0, size: 89, used: 65, avail: 24 }));
        let cluster = Cluster::new(TEST_DATA).unwrap();
        assert_eq!(cluster.width(), 3);
        assert_eq!(cluster.height(), 3);
    }
}
