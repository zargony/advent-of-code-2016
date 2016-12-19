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

fn main() {
    println!("Elf that gets all the presents: {}", solve(3005290));
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
}
