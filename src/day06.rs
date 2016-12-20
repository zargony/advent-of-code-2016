use std::collections::HashMap;


/// Build a character frequency map for each column of a text
fn character_column_frequency(text: &str) -> Vec<HashMap<char, u32>> {
    let mut ccf: Vec<HashMap<char, u32>> = vec![];
    for line in text.lines() {
        for (col, ch) in line.chars().enumerate() {
            if ccf.len() <= col { ccf.push(HashMap::new()); }
            assert!(ccf.len() > col);
            *ccf[col].entry(ch).or_insert(0) += 1;
        }
    }
    ccf
}

/// "Error correct" a repeated message by choosing the most often used
/// character for each position
pub fn error_correct_max(msgs: &str) -> String {
    character_column_frequency(msgs).iter().map(|h|
        *h.iter().max_by_key(|&(_, num)| num).unwrap().0
    ).collect()
}

/// "Error correct" a repeated message by choosing the least often used
/// character for each position
pub fn error_correct_min(msgs: &str) -> String {
    character_column_frequency(msgs).iter().map(|h|
        *h.iter().min_by_key(|&(_, num)| num).unwrap().0
    ).collect()
}

fn main() {
    const INPUT: &'static str = include_str!("day06.txt");
    let message = error_correct_max(INPUT);
    println!("Error corrected message: {}", message);
    let message = error_correct_min(INPUT);
    println!("Error corrected (least) message: {}", message);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_correction() {
        const INPUT: &'static str = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!(error_correct_max(INPUT), "easter");
        assert_eq!(error_correct_min(INPUT), "advent");
    }
}
