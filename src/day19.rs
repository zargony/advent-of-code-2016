use std::env;
use std::collections::VecDeque;

pub fn solve(n: usize) -> usize {
    let mut elves: Vec<_> = (1..n+1).collect();
    let mut flag = false;
    while elves.len() > 1 {
        elves.retain(|_| {
            flag = !flag;
            flag
        });
    }
    elves[0]
}

pub fn solve2(n: usize) -> usize {
    let mut elves: VecDeque<_> = (1..n+1).collect();
    let mut i = 0;
    while elves.len() > 1 {
        let target = (i + elves.len() / 2) % elves.len();
        elves.remove(target);
        if target < i { i -= 1; }
        i = (i + 1) % elves.len();
    }
    elves[0]
}

fn main() {
    println!("Elf that gets all the presents: {}", solve(3005290));

    // Don't run the tedious part two on CI
    if !env::var("CI").is_ok() {
        println!("Elf that gets all the presents (new rules): {}", solve2(3005290));
    }
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
