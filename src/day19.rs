pub fn solve(n: usize) -> usize {
    (2..n+1).fold(0, |winner, nn| (winner + 2) % nn) + 1
}

pub fn solve2(n: usize) -> usize {
    (2..n+1).fold(0, |winner, nn| {
        if nn / 2 - 1 > winner {
            (winner + 1) % nn
        } else {
            (winner + 2) % nn
        }
    }) + 1
}

fn main() {
    println!("Elf that gets all the presents: {}", solve(3005290));
    println!("Elf that gets all the presents (new rules): {}", solve2(3005290));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solving() {
        assert_eq!(solve(4), 1);
        assert_eq!(solve(5), 3);
        assert_eq!(solve(6), 5);
        assert_eq!(solve(7), 7);
        assert_eq!(solve(8), 1);
        assert_eq!(solve(9), 3);
    }

    #[test]
    fn solving2() {
        assert_eq!(solve2(5), 2);
    }
}
