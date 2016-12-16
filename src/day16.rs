extern crate itertools;

use std::{fmt, iter, slice};
use std::iter::FromIterator;
use std::str::FromStr;
use itertools::Itertools;

/// Bit data
#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    bits: Vec<bool>,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &bit in &self.bits {
            try!(f.write_str(if bit { "1" } else { "0" }));
        }
        Ok(())
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Data, ()> {
        Ok(s.chars().map(|ch|
            match ch {
                '0' => false,
                '1' => true,
                _ => panic!("Invalid bit"),
            }
        ).collect())
    }
}

impl FromIterator<bool> for Data {
    fn from_iter<T: IntoIterator<Item=bool>>(iter: T) -> Data {
        Data { bits: iter.into_iter().collect() }
    }
}

impl Data {
    /// Length of data
    fn len(&self) -> usize {
        self.bits.len()
    }

    /// An iterater over bits
    fn bits(&self) -> iter::Cloned<slice::Iter<bool>> {
        self.bits.iter().cloned()
    }

    /// Generate new data based on the existing data
    /// FIXME: Should return an iterator
    fn generate(&self) -> Data {
        self.bits().chain(
            iter::once(false)
        ).chain(
            self.bits().map(|b| !b).rev()
        ).collect()
    }

    /// Fill up to the given length with newly generated data
    /// FIXME: Should return an iterator
    fn fill_up(&self, len: usize) -> Data {
        let mut data = self.generate();
        while data.len() < len { data = data.generate(); }
        data.bits().take(len).collect()
    }

    /// Calculate checksum
    fn checksum(&self) -> Data {
        let checksum: Data = self.bits().tuples().map(|(a, b)| !(a ^ b)).collect();
        if checksum.len() % 2 == 0 {
            checksum.checksum()
        } else {
            checksum
        }
    }
}

fn main() {
    let data = Data::from_str("01000100010010111").unwrap().fill_up(272);
    println!("Checksum for data: {}", data.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parsing() {
        assert_eq!(Data::from_str("1101"), Ok(Data { bits: vec![true, true, false, true] }));
    }

    #[test]
    fn filling() {
        assert_eq!(Data::from_str("1").unwrap().generate(), Data::from_str("100").unwrap());
        assert_eq!(Data::from_str("0").unwrap().generate(), Data::from_str("001").unwrap());
        assert_eq!(Data::from_str("11111").unwrap().generate(), Data::from_str("11111000000").unwrap());
        assert_eq!(Data::from_str("111100001010").unwrap().generate(), Data::from_str("1111000010100101011110000").unwrap());
        assert_eq!(Data::from_str("10000").unwrap().fill_up(20), Data::from_str("10000011110010000111").unwrap());
    }

    #[test]
    fn checksumming() {
        assert_eq!(Data::from_str("110010110100").unwrap().checksum(), Data::from_str("100").unwrap());
        assert_eq!(Data::from_str("10000011110010000111").unwrap().checksum(), Data::from_str("01100").unwrap());
    }
}
