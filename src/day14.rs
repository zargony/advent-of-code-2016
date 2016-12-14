extern crate md5;
extern crate onig;
extern crate time;

use std::io::Write;
use onig::Regex;

/// A hash finder uses a brute force approach to find a MD5 digest
/// for a given prefix that has 3 or more repeating characters in its
/// hex representation
pub struct HashFinder {
    pad: u32,
    ctx: md5::Context,
    re: onig::Regex,
}

impl HashFinder {
    /// Create new hash finder for the given prefix
    fn new(prefix: &str) -> HashFinder {
        let mut ctx = md5::Context::new();
        ctx.consume(prefix.as_bytes());
        HashFinder {
            pad: 0,
            ctx: ctx,
            re: Regex::new("([a-z0-9])\\1\\1+").unwrap(),
        }
    }
}

impl Iterator for HashFinder {
    type Item = (u32, Vec<String>);

    /// Finds the next MD5 hexdigest with 3 or more repeating characters
    /// by appending an ever increasing number to the prefix. Yields a
    /// tuple for every match that contains the appended number and
    /// a vector of strings with 3+ repeated characters.
    fn next(&mut self) -> Option<(u32, Vec<String>)> {
        loop {
            let mut ctx = self.ctx.clone();
            ctx.write_fmt(format_args!("{}", self.pad)).unwrap();
            let digest = ctx.compute();
            self.pad += 1;
            // OPTIMIZE: String allocation and regex searching is slow
            let hexdigest = format!("{:x}", digest);
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
pub struct OTPFinder {
    finder: HashFinder,
    snippets: Vec<(u32, String)>,
    pos: usize,
}

impl OTPFinder {
    /// Create new OTP finder for the given seed
    fn new(seed: &str) -> OTPFinder {
        OTPFinder { finder: HashFinder::new(seed), snippets: Vec::new(), pos: 0 }
    }
}

impl Iterator for OTPFinder {
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
    let mut finder = OTPFinder::new("ahsbgdzn");
    let (pad, duration) = measure_time(|| finder.nth(63).unwrap());
    println!("Index that produces the 64th key (found in {:5.3}s): {}", duration, pad);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_digests() {
        let mut finder = HashFinder::new("abc");
        assert_eq!(finder.next(), Some((18, vec!["888".to_owned()])));
        assert_eq!(finder.next(), Some((39, vec!["eee".to_owned()])));
        assert_eq!(finder.skip(6).next(), Some((92, vec!["999".to_owned()])));
    }

    #[test]
    fn finding_otps() {
        let mut finder = OTPFinder::new("abc");
        assert_eq!(finder.next(), Some(39));
        assert_eq!(finder.next(), Some(92));
        assert_eq!(finder.skip(61).next(), Some(22728));
    }
}
