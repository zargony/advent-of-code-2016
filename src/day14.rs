extern crate md5;
extern crate onig;
extern crate time;

use std::io::Write;
use onig::Regex;

/// Helper function for displaying a nibble
#[inline]
fn nibble2char(n: u8) -> char {
    match n {
        0...9 => ('0' as u8 + n) as char,
        10...15 => ('a' as u8 + n - 10) as char,
        _ => panic!("nibble must be in range 0..15"),
    }
}

/// A hash finder uses a brute force approach to find a MD5 digest
/// for a given prefix that has 3 or more repeating characters in its
/// hex representation
pub struct HashFinder<'a> {
    prefix: &'a str,
    pad: u32,
    re: onig::Regex,
    stretch: usize,
}

impl<'a> HashFinder<'a> {
    /// Create new hash finder for the given prefix
    fn new(prefix: &str, stretch: usize) -> HashFinder {
        HashFinder {
            prefix: prefix,
            pad: 0,
            re: Regex::new("([a-z0-9])\\1\\1+").unwrap(),
            stretch: stretch,
        }
    }
}

impl<'a> Iterator for HashFinder<'a> {
    type Item = (u32, usize, char);

    /// Finds the next MD5 hexdigest with 3 or more repeating characters
    /// by appending an ever increasing number to the prefix. Yields a
    /// tuple for every match that contains the appended number and
    /// a vector of strings with 3+ repeated characters.
    fn next(&mut self) -> Option<(u32, usize, char)> {
        loop {
            let mut md5 = md5::Context::new();
            md5.consume(self.prefix.as_bytes());
            md5.write_fmt(format_args!("{}", self.pad)).unwrap();
            let mut digest = md5.compute();
            for _ in 0..self.stretch {
                let mut md5 = md5::Context::new();
                for &b in digest.iter() {
                    let buf = [nibble2char(b >> 4) as u8, nibble2char(b &0xf) as u8];
                    md5.consume(&buf);
                }
                digest = md5.compute();
            }
            let hexdigest = format!("{:x}", digest);
            self.pad += 1;
            if let Some(cap) = self.re.captures(&hexdigest) {
                let snippet = cap.at(0).unwrap();
                return Some((self.pad - 1, snippet.len(), snippet.chars().nth(0).unwrap()));
            }
        }
    }
}

/// The OTP finder yields valid one-time passwords
pub struct OTPFinder<'a> {
    finder: HashFinder<'a>,
    snippets: Vec<(u32, usize, char)>,
    pos: usize,
}

impl<'a> OTPFinder<'a> {
    /// Create new OTP finder for the given seed
    fn new(seed: &str, stretch: usize) -> OTPFinder {
        OTPFinder { finder: HashFinder::new(seed, stretch), snippets: Vec::new(), pos: 0 }
    }
}

impl<'a> Iterator for OTPFinder<'a> {
    type Item = u32;

    /// Yields the pad number of the next one-time password
    fn next(&mut self) -> Option<u32> {
        loop {
            while self.pos >= self.snippets.len() || self.snippets.last().unwrap().0 < self.snippets[self.pos].0 + 1000 {
                if let Some(snippet) = self.finder.next() {
                    self.snippets.push(snippet);
                }
            }
            let (pad, _, ch) = self.snippets[self.pos];
            self.pos += 1;
            if self.snippets.iter().any(|&(p, l, c)|
                p > pad && p <= pad+1000 && l >= 5 && c == ch
            ) {
                return Some(pad);
            }
        }
    }
}

/// Measure time
fn measure_time<T, F: FnMut() -> T>(mut f: F) -> (T, f64) {
    let start_time = time::precise_time_s();
    let result = f();
    let duration = time::precise_time_s() - start_time;
    (result, duration)
}

fn main() {
    let mut finder = OTPFinder::new("ahsbgdzn", 0);
    let (pad, duration1) = measure_time(|| finder.nth(63).unwrap());
    println!("Index that produces the 64th key (found in {:5.3}s): {}", duration1, pad);
    let mut finder = OTPFinder::new("ahsbgdzn", 2016);
    let (pad, duration2) = measure_time(|| finder.nth(63).unwrap());
    println!("Index that produces the 64th streched key (found in {:5.3}s): {}", duration2, pad);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_digests() {
        let mut finder = HashFinder::new("abc", 0);
        assert_eq!(finder.next(), Some((18, 3, '8')));
        assert_eq!(finder.next(), Some((39, 3, 'e')));
        assert_eq!(finder.skip(6).next(), Some((92, 3, '9')));
    }

    #[test]
    fn finding_stretched_digests() {
        let mut finder = HashFinder::new("abc", 2016);
        assert_eq!(finder.next(), Some((5, 3, '2')));
        assert_eq!(finder.next(), Some((10, 3, 'e')));
    }

    #[test]
    fn finding_otps() {
        let mut finder = OTPFinder::new("abc", 0);
        assert_eq!(finder.next(), Some(39));
        assert_eq!(finder.next(), Some(92));
        assert_eq!(finder.skip(61).next(), Some(22728));
    }

    #[test]
    fn finding_stretched_otps() {
        let mut finder = OTPFinder::new("abc", 2016);
        assert_eq!(finder.next(), Some(10));
        // assert_eq!(finder.skip(62).next(), Some(22551));
    }
}
