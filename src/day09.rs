pub fn decompress(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let char_count: usize = chars.by_ref().take_while(|ch| *ch != 'x').collect::<String>().parse().unwrap();
            let repeat_count: usize = chars.by_ref().take_while(|ch| *ch != ')').collect::<String>().parse().unwrap();
            let snippet: String = chars.by_ref().take(char_count).collect();
            for _ in 0..repeat_count {
                result.push_str(&snippet);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn main() {
    println!("Length of decompressed file: {}", decompress(include_str!("day09.txt")).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompressing() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }
}
