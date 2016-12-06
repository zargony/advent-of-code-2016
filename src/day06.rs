use std::collections::HashMap;

/// "Error correct" a repeated message by choosing the most often used
/// character for each position
pub fn error_correct(msgs: &str) -> String {
    let mut chars: Vec<HashMap<char, u32>> = vec![];
    for msg in msgs.lines() {
        for (i, ch) in msg.chars().enumerate() {
            if chars.len() <= i { chars.push(HashMap::new()); }
            assert!(chars.len() > i);
            *chars[i].entry(ch).or_insert(0) += 1;
        }
    }
    chars.iter().map(|h|
        *h.iter().max_by_key(|&(_, num)| num).unwrap().0
    ).collect()
}

fn main() {
    let message = error_correct(include_str!("day06.txt"));
    println!("Error corrected message: {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_correction() {
        assert_eq!(error_correct("eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar"), "easter");
    }
}
