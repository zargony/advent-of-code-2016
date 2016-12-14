extern crate md5;
extern crate onig;
extern crate time;

use std::env;
use onig::Regex;

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
    type Item = (u32, Vec<String>);

    /// Finds the next MD5 hexdigest with 3 or more repeating characters
    /// by appending an ever increasing number to the prefix. Yields a
    /// tuple for every match that contains the appended number and
    /// a vector of strings with 3+ repeated characters.
    fn next(&mut self) -> Option<(u32, Vec<String>)> {
        loop {
            let mut hexdigest = format!("{}{}", self.prefix, self.pad);
            for _ in 0..self.stretch + 1 {
                hexdigest = format!("{:x}", md5::compute(hexdigest));
            }
            self.pad += 1;
            let matches: Vec<String> = self.re.find_iter(&hexdigest).map(|(pos1, pos2)| {
                hexdigest[pos1..pos2].to_owned()
            }).collect();
            if !matches.is_empty() {
                return Some((self.pad - 1, matches));
            }
        }
    }
}

/// The OTP finder yields valid one-time passwords
pub struct OTPFinder<'a> {
    finder: HashFinder<'a>,
    snippets: Vec<(u32, String)>,
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
                match self.finder.next() {
                    Some((pad, ss)) => self.snippets.push((pad, ss.into_iter().nth(0).unwrap())),
                    None => return None,
                }
            }
            let (pad, ref snippet) = self.snippets[self.pos];
            self.pos += 1;
            if self.snippets.iter().any(|&(p, ref s)|
                p > pad && p <= pad+1000 && s.len() >= 5 && s[0..1] == snippet[0..1])
            {
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

    // Don't run the tedious part two on CI
    if !env::var("CI").is_ok() {
        let mut finder = OTPFinder::new("ahsbgdzn", 2016);
        let (pad, duration2) = measure_time(|| finder.nth(63).unwrap());
        println!("Index that produces the 64th streched key (found in {:5.3}s): {}", duration2, pad);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn finding_digests() {
        let mut finder = HashFinder::new("abc", 0);
        assert_eq!(finder.next(), Some((18, vec!["888".to_owned()])));
        assert_eq!(finder.next(), Some((39, vec!["eee".to_owned()])));
        assert_eq!(finder.skip(6).next(), Some((92, vec!["999".to_owned()])));
    }

    #[test]
    fn finding_stretched_digests() {
        let mut finder = HashFinder::new("abc", 2016);
        assert_eq!(finder.next(), Some((5, vec!["222".to_owned()])));
        assert_eq!(finder.next(), Some((10, vec!["eee".to_owned()])));
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
        // Don't run this tedious test on CI
        if !env::var("CI").is_ok() {
            let mut finder = OTPFinder::new("abc", 2016);
            assert_eq!(finder.next(), Some(10));
            // assert_eq!(finder.skip(62).next(), Some(22551));
        }
    }
}
