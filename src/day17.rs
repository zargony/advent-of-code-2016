extern crate md5;

pub fn solve(passcode: &str) -> (String, String) {

    fn _solve<F: FnMut(&str)>(x: usize, y: usize, path: &mut String, ctx: md5::Context, f: &mut F) {
        if x == 3 && y == 3 {
            f(path);
            return;
        }
        let digest = ctx.clone().compute();
        if y > 0 && digest[0] >> 4 > 0xa {      // up
            path.push('U');
            let mut new_ctx = ctx.clone();
            new_ctx.consume("U");
            _solve(x, y - 1, path, new_ctx, f);
            path.pop();
        }
        if y < 3 && digest[0] & 0xf > 0xa {     // down
            path.push('D');
            let mut new_ctx = ctx.clone();
            new_ctx.consume("D");
            _solve(x, y + 1, path, new_ctx, f);
            path.pop();
        }
        if x > 0 && digest[1] >> 4 > 0xa {      // left
            path.push('L');
            let mut new_ctx = ctx.clone();
            new_ctx.consume("L");
            _solve(x - 1, y, path, new_ctx, f);
            path.pop();
        }
        if x < 3 && digest[1] & 0xf > 0xa {     // right
            path.push('R');
            let mut new_ctx = ctx.clone();
            new_ctx.consume("R");
            _solve(x + 1, y, path, new_ctx, f);
            path.pop();
        }
    }

    let mut shortest_path = String::new();
    let mut longest_path = String::new();
    let mut ctx = md5::Context::new();
    ctx.consume(passcode);
    _solve(0, 0, &mut String::new(), ctx, &mut |path| {
        if shortest_path.is_empty() || shortest_path.len() > path.len() {
            shortest_path = path.to_string();
        }
        if longest_path.len() < path.len() {
            longest_path = path.to_string();
        }
    });
    (shortest_path, longest_path)
}

fn main() {
    let input = "yjjvjgan";
    let (shortest_path, longest_path) = solve(input);
    println!("Shortest path for passcode '{}': {}", input, shortest_path);
    println!("Length of longest path for passcode '{}': {}", input, longest_path.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solving() {
        assert_eq!(solve("ihgpwlah").0, "DDRRRD".to_owned());
        assert_eq!(solve("kglvqrro").0, "DDUDRLRRUDRD".to_owned());
        assert_eq!(solve("ulqzkmiv").0, "DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_owned());
    }
}
