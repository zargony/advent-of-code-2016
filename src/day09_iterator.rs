use std::str;

pub struct Repeat<'a> {
    count: usize,
    chars: str::Chars<'a>,
    iter: str::Chars<'a>,
}

impl<'a> Iterator for Repeat<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.count == 0 {
            return None;
        }
        match self.iter.next() {
            Some(ch) => Some(ch),
            None => {
                self.count -= 1;
                self.iter = self.chars.clone();
                self.next()
            }
        }
    }
}

impl<'a> Repeat<'a> {
    fn new(data: &str, count: usize) -> Repeat {
        Repeat { count: count, chars: data.chars(), iter: data.chars() }
    }

    fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }
}

pub struct Decompressor<'a> {
    iters: Vec<Repeat<'a>>,
    recursive: bool,
}

impl<'a> Iterator for Decompressor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.iters.is_empty() {
            return None;
        }
        match self.iters.last_mut().unwrap().next() {
            Some('(') if self.iters.len() == 1 || self.recursive => {
                let repeat_len: usize = self.iters.last_mut().unwrap().by_ref().take_while(|ch| *ch != 'x').collect::<String>().parse().unwrap();
                let repeat_cnt: usize = self.iters.last_mut().unwrap().by_ref().take_while(|ch| *ch != ')').collect::<String>().parse().unwrap();
                let repeat_str = &self.iters.last_mut().unwrap().as_str()[..repeat_len];
                for _ in 0..repeat_len { self.iters.last_mut().unwrap().next().unwrap(); }
                self.iters.push(Repeat::new(repeat_str, repeat_cnt));
                self.next()
            },
            Some(ch) => Some(ch),
            None => {
                self.iters.pop();
                self.next()
            }
        }
    }
}

impl<'a> Decompressor<'a> {
    fn new(data: &str, recursive: bool) -> Decompressor {
        Decompressor { iters: vec![Repeat::new(data, 1)], recursive: recursive }
    }
}

fn main() {
    let input = include_str!("day09.txt");
    println!("Length of decompressed file: {}", Decompressor::new(input, false).count());
    println!("Length of recursively decompressed file: {}", Decompressor::new(input, true).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeating() {
        assert_eq!(Repeat::new("A", 0).collect::<String>(), "");
        assert_eq!(Repeat::new("A", 5).collect::<String>(), "AAAAA");
        assert_eq!(Repeat::new("ABC", 3).collect::<String>(), "ABCABCABC");
    }

    #[test]
    fn decompressing() {
        assert_eq!(Decompressor::new("ADVENT", false).collect::<String>(), "ADVENT");
        assert_eq!(Decompressor::new("A(1x5)BC", false).collect::<String>(), "ABBBBBC");
        assert_eq!(Decompressor::new("(3x3)XYZ", false).collect::<String>(), "XYZXYZXYZ");
        assert_eq!(Decompressor::new("A(2x2)BCD(2x2)EFG", false).collect::<String>(), "ABCBCDEFEFG");
        assert_eq!(Decompressor::new("(6x1)(1x3)A", false).collect::<String>(), "(1x3)A");
        assert_eq!(Decompressor::new("X(8x2)(3x3)ABCY", false).collect::<String>(), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn decompressing_recursively() {
        assert_eq!(Decompressor::new("(3x3)XYZ", true).count(), 9);
        assert_eq!(Decompressor::new("X(8x2)(3x3)ABCY", true).count(), 20);
        assert_eq!(Decompressor::new("(27x12)(20x12)(13x14)(7x10)(1x12)A", true).count(), 241920);
        assert_eq!(Decompressor::new("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", true).count(), 445);
    }
}
