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

pub fn decompressed_size_recursive(s: &str) -> usize {
    let mut result = 0;
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let char_count: usize = chars.by_ref().take_while(|ch| *ch != 'x').collect::<String>().parse().unwrap();
            let repeat_count: usize = chars.by_ref().take_while(|ch| *ch != ')').collect::<String>().parse().unwrap();
            let snippet: String = chars.by_ref().take(char_count).collect();
            result += repeat_count * decompressed_size_recursive(&snippet);
        } else {
            result += 1;
        }
    }
    result
}

fn main() {
    const INPUT: &'static str = include_str!("day09.txt");
    println!("Length of decompressed file: {}", decompress(INPUT).len());
    println!("Length of recursively decompressed file: {}", decompressed_size_recursive(INPUT));
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

    #[test]
    fn decompressing_recursively() {
        assert_eq!(decompressed_size_recursive("(3x3)XYZ"), 9);
        assert_eq!(decompressed_size_recursive("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(decompressed_size_recursive("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(decompressed_size_recursive("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    }
}
