extern crate onig;

use onig::Regex;

pub fn count_tls_ips(input: &str) -> usize {
    let re1 = Regex::new("\\[[a-z]*([a-z])(?!\\1)([a-z])\\2\\1").unwrap();
    let re2 = Regex::new("([a-z])(?!\\1)([a-z])\\2\\1").unwrap();
    input.lines().filter(|ip| {
        re1.find(ip).is_none() && re2.find(ip).is_some()
    }).count()
}

fn main() {
    let input = include_str!("day07.txt");
    println!("Number of IPs with TLS support: {}", count_tls_ips(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_support_detection() {
        assert_eq!(count_tls_ips("abba[mnop]qrst"), 1);
        assert_eq!(count_tls_ips("abcd[bddb]xyyx"), 0);
        assert_eq!(count_tls_ips("aaaa[qwer]tyui"), 0);
        assert_eq!(count_tls_ips("ioxxoj[asdfgh]zxcvbn"), 1);
    }
}
