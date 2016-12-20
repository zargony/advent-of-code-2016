use std::str::FromStr;

/// Parse multiline-text of ranges into a vector of tuples
pub fn parse(s: &str) -> Vec<(u32, u32)> {
    s.lines().map(|line| {
        let mut nums = line.split('-').map(|s| u32::from_str(s).unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).collect()
}

/// Find lowest number not covered by a list of ranges
pub fn find_lowest(ranges: &[(u32, u32)]) -> u32 {
    let mut ranges = ranges.to_owned();
    ranges.sort_by_key(|n| n.0);
    let mut n = 0;
    for (from, to) in ranges {
        if from > n { break; }
        if to >= n { n = to + 1; }
    }
    n
}

fn main() {
    let ranges = parse(include_str!("day20.txt"));
    println!("Lowest non-blocked IP: {}", find_lowest(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_lowest() {
        let ranges = parse("5-8\n0-2\n4-7");
        assert_eq!(find_lowest(&ranges), 3);
    }
}
