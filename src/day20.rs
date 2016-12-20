use std::num;


/// Parse multiline-text of ranges into a vector of tuples
pub fn parse(s: &str) -> Result<Vec<(u32, u32)>, num::ParseIntError> {
    s.lines().map(|line| {
        let mut nums = line.split('-').map(|s| s.parse::<u32>().unwrap());
        Ok((nums.next().unwrap(), nums.next().unwrap()))
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

/// Find amount of numbers not covered by a list of ranges
pub fn find_uncovered(ranges: &[(u32, u32)]) -> u32 {
    let mut ranges = ranges.to_owned();
    ranges.sort_by_key(|n| n.0);
    let mut upto = 0;
    let mut count = 0;
    for (from, to) in ranges {
        if from > upto { count += from - upto - 1; }
        if to > upto { upto = to; }
    }
    count
}

fn main() {
    let ranges = parse(include_str!("day20.txt")).unwrap();
    println!("Lowest non-blocked IP: {}", find_lowest(&ranges));
    println!("Number of allowed IPs: {}", find_uncovered(&ranges));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_lowest() {
        let ranges = parse("5-8\n0-2\n4-7").unwrap();
        assert_eq!(find_lowest(&ranges), 3);
    }

    #[test]
    fn finding_uncovered() {
        let ranges = parse("5-8\n0-2\n4-7").unwrap();
        assert_eq!(find_uncovered(&ranges), 1);
    }
}
