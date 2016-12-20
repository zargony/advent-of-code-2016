extern crate onig;

use onig::Regex;


pub fn count_tls_ips(input: &str) -> usize {
    let re1 = Regex::new("\\[[a-z]*([a-z])(?!\\1)([a-z])\\2\\1").unwrap();
    let re2 = Regex::new("([a-z])(?!\\1)([a-z])\\2\\1").unwrap();
    input.lines().filter(|ip| {
        re1.find(ip).is_none() && re2.find(ip).is_some()
    }).count()
}

pub fn count_ssl_ips(input: &str) -> usize {
    let re1 = Regex::new("(?:^|\\])[a-z]*([a-z])(?!\\1)([a-z])\\1.*\\[[a-z]*\\2\\1\\2").unwrap();
    let re2 = Regex::new("\\[[a-z]*([a-z])(?!\\1)([a-z])\\1.*\\][a-z]*\\2\\1\\2").unwrap();
    input.lines().filter(|ip| {
        re1.find(ip).is_some() || re2.find(ip).is_some()
    }).count()
}

fn main() {
    const INPUT: &'static str = include_str!("day07.txt");
    println!("Number of IPs with TLS support: {}", count_tls_ips(INPUT));
    println!("Number of IPs with SSL support: {}", count_ssl_ips(INPUT));
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

    #[test]
    fn ssl_support_detection() {
        assert_eq!(count_ssl_ips("aba[bab]xyz"), 1);
        assert_eq!(count_ssl_ips("xyx[xyx]xyx"), 0);
        assert_eq!(count_ssl_ips("aaa[kek]eke"), 1);
        assert_eq!(count_ssl_ips("zazbz[bzb]cdb"), 1);
    }
}
