#[macro_use]
extern crate nom;

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use nom::{space, digit};


#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
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
    #[inline]
    fn width(&self) -> usize { self.nodes.len() }

    /// Height
    #[inline]
    fn height(&self) -> usize { self.nodes[0].len() }

    /// Count viable pairs of nodes
    fn count_viable_node_pairs(&self) -> usize {
        self.nodes.iter().flat_map(|col| col.iter()).filter(|a| a.used > 0).map(|a| {
            self.nodes.iter().flat_map(|col| col.iter()).filter(|b| a.used < b.avail).count()
        }).sum()
    }

    /// Get node at given position
    #[inline]
    fn get(&self, x: usize, y: usize) -> &Node {
        &self.nodes[x][y]
    }

    /// Get node at given position
    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Node {
        &mut self.nodes[x][y]
    }

    /// Move all data from one node to another (empty) node
    fn move_data(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        assert!(self.get(x2, y2).used == 0);
        assert!(self.get(x2, y2).avail > self.get(x1, y1).used);
        self.get_mut(x2, y2).used += self.get(x1, y1).used;
        self.get_mut(x2, y2).avail -= self.get(x1, y1).used;
        self.get_mut(x1, y1).avail += self.get(x1, y1).used;
        self.get_mut(x1, y1).used = 0;
    }

    /// Returns a new cluster state with the given data movement applied
    fn with_moved_data(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Cluster {
        let mut cluster = self.clone();
        cluster.move_data(x1, y1, x2, y2);
        cluster
    }

    /// Returns neighbors of the node at the given position
    fn neighbors(&self, x: usize, y: usize) -> Vec<&Node> {
        [
            if y > 0               { Some(self.get(x, y-1)) } else { None },
            if y < self.height()-1 { Some(self.get(x, y+1)) } else { None },
            if x > 0               { Some(self.get(x-1, y)) } else { None },
            if x < self.width()-1  { Some(self.get(x+1, y)) } else { None },
        ].iter().filter_map(|n| *n).collect()
    }

    /// Move data around to free the given position. Returns a tuple with the
    /// number of steps taken and the final cluster with the given position freed
    fn free(&self, x: usize, y: usize, blacklist: &[(usize, usize)]) -> Option<(usize, Cluster)> {
        let mut states: VecDeque<_> = self.nodes.iter()
            .flat_map(|col| col.iter())
            .filter(|node| node.used == 0)
            .map(|node| (0, (node.x, node.y), self.clone()))
            .collect();
        let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();
        while let Some((depth, (xx, yy), cluster)) = states.pop_front() {
            let node = cluster.get(xx, yy);
            debug_assert!(node.used == 0);
            for neighbor in self.neighbors(xx, yy) {
                if blacklist.contains(&(neighbor.x, neighbor.y)) {
                    continue;
                }
                if let Some(&dist) = shortest.get(&(neighbor.x, neighbor.y)) {
                    if dist <= depth {
                        continue;
                    }
                }
                if node.avail > neighbor.used {
                    shortest.insert((neighbor.x, neighbor.y), depth);
                    let new_cluster = cluster.with_moved_data(neighbor.x, neighbor.y, node.x, node.y);
                    if neighbor.x == x && neighbor.y == y {
                        return Some((depth + 1, new_cluster));
                    }
                    states.push_back((depth + 1, (neighbor.x, neighbor.y), new_cluster));
                }
            }
        }
        None
    }

    /// Count steps needed to move data from (x,0) to (0,0)
    /// Strategy: free (x-1,0), move (x,0) to (x-1,0), then recurse with x-1
    fn move_top_data(&self, x: usize) -> Option<usize> {
        if x == 0 {
            return Some(0);
        }
        self.free(x - 1, 0, &[(x, 0)]).and_then(|(steps, cluster)| {
            cluster.with_moved_data(x, 0, x - 1, 0).move_top_data(x - 1).map(|s|
                steps + 1 + s
            )
        })
    }
}


fn main() {
    let cluster = Cluster::new(include_str!("day22.txt")).unwrap();
    println!("Viable pairs of nodes: {}", cluster.count_viable_node_pairs());
    let steps = cluster.move_top_data(cluster.width() - 1).unwrap();
    println!("Fewest number of steps to move top right node to top left corner: {}", steps);
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

    #[test]
    fn moving_top_data() {
        let cluster = Cluster::new(TEST_DATA).unwrap();
        assert_eq!(cluster.move_top_data(cluster.width() - 1), Some(7));
    }
}
