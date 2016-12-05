extern crate md5;

/// A password finder uses a brute force approach to find
/// the next character of a "password" for a given door id.
pub struct PasswordFinder<'a> {
    door_id: &'a str,
    pad: u32,
    ctx: md5::Context,
}

impl<'a> PasswordFinder<'a> {
    /// Create new password finder for the given prefix
    fn new(door_id: &str) -> PasswordFinder {
        let mut ctx = md5::Context::new();
        ctx.consume(door_id.as_bytes());
        PasswordFinder {
            door_id: door_id,
            pad: 0,
            ctx: ctx,
        }
    }
}

impl<'a> Iterator for PasswordFinder<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            let s = format!("{}", self.pad);
            let mut ctx = self.ctx.clone();
            ctx.consume(s.as_bytes());
            let hash = ctx.compute();
            self.pad += 1;
            if hash[0..2] == [0; 2] && hash[2] & 0xf0 == 0 {
                let res = format!("{:1x}", hash[2] & 0x0f);
                return res.chars().nth(0);
            }
        }
    }
}

fn main() {
    let finder = PasswordFinder::new("wtnhxymk");
    let password: String = finder.take(8).collect();
    println!("Password: {}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_next_password_character() {
        let mut finder = PasswordFinder::new("abc");
        finder.pad = 3200000;   // speed up a bit for testing
        assert_eq!(finder.next(), Some('1'));
        finder.pad = 5000000;   // speed up a bit for testing
        assert_eq!(finder.next(), Some('8'));
        assert_eq!(finder.next(), Some('f'));
    }
}
