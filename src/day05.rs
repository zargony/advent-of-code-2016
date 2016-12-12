extern crate md5;
extern crate time;

use std::fmt::Write as fmtWrite;
use std::io::Write as ioWrite;

/// A hash finder uses a brute force approach to find a hash
/// with a starting zeroes for a given prefix
pub struct HashFinder {
    pad: u32,
    ctx: md5::Context,
}

impl HashFinder {
    /// Create new hash finder for the given prefix
    fn new(prefix: &str) -> HashFinder {
        let mut ctx = md5::Context::new();
        ctx.consume(prefix.as_bytes());
        HashFinder {
            pad: 0,
            ctx: ctx,
        }
    }
}

impl Iterator for HashFinder {
    type Item = (u8, u8);

    /// Finds the next MD5 hash with 5 leading nibbles by appending an
    /// ever increasing number to the prefix. Yields the next two
    /// nibbles
    fn next(&mut self) -> Option<(u8, u8)> {
        loop {
            let mut ctx = self.ctx.clone();
            // ctx.consume(format!("{}", self.pad).as_bytes());
            ctx.write_fmt(format_args!("{}", self.pad)).unwrap();
            let hash = ctx.compute();
            self.pad += 1;
            if hash[0..2] == [0; 2] && hash[2] & 0xf0 == 0 {
                return Some((hash[2] & 0x0f, hash[3] >> 4));
            }
        }
    }
}

/// Helper function for displaying a nibble
fn nibble2char(n: u8) -> char {
    match n {
        0...9 => ('0' as u8 + n) as char,
        10...15 => ('a' as u8 + n - 10) as char,
        _ => panic!("nibble must be in range 0..15"),
    }
}

/// Find (simple) password for the given door id
fn find_password(door_id: &str, len: usize) -> String {
    let finder = HashFinder::new(door_id);
    finder.take(len).fold(String::with_capacity(len), |mut s, (b, _)| {
        s.write_char(nibble2char(b)).unwrap();
        s
    })
}

/// Find enhanced password for the given door id
fn find_enhanced_password(door_id: &str, len: usize) -> String {
    let mut password: Vec<u8> = vec![0; len];
    for (pos, b) in HashFinder::new(door_id) {
        let pos = pos as usize;
        if pos < password.len() && password[pos] == 0 {
            password[pos] = nibble2char(b) as u8;
        }
        if password.iter().all(|b| *b != 0) { break; }
    }
    String::from_utf8(password).unwrap()
}

/// Measure time
fn measure_time<T, F: FnMut() -> T>(mut f: F) -> (T, f64) {
    let start_time = time::precise_time_s();
    let result = f();
    let duration = time::precise_time_s() - start_time;
    (result, duration)
}

fn main() {
    let input = "wtnhxymk";
    let (password, duration1) = measure_time(|| find_password(input, 8));
    println!("Password (found in {:5.3}s): {}", duration1, password);
    let (password, duration2) = measure_time(|| find_enhanced_password(input, 8));
    println!("Enhanced password (found in {:5.3}s): {}", duration2, password);
    println!("Total time: {:5.3}s", duration1 + duration2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_next_password_character() {
        let mut finder = HashFinder::new("abc");
        finder.pad = 3200000;   // speed up a bit for testing
        assert_eq!(finder.next(), Some((0x1, 0x5)));
        finder.pad = 5000000;   // speed up a bit for testing
        assert_eq!(finder.next(), Some((0x8, 0xf)));
        assert_eq!(finder.next(), Some((0xf, 0x9)));
        assert_eq!(finder.next(), Some((0x4, 0xe)));
    }
}
