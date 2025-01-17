use advtools::input;
use advtools::prelude::*;

const RX: &str = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";

fn main() {
    let (part1, part2) = solve();

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}

fn solve() -> (u32, u32) {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut enabled = true;

    for m in Regex::new(RX).unwrap().captures_iter(input::string()) {
        if &m[0] == "do()" {
            enabled = true;
        } else if &m[0] == "don't()" {
            enabled = false;
        } else {
            let n = m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap();
            sum1 += n;
            if enabled {
                sum2 += n;
            }
        }
    }

    (sum1, sum2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        input::set(INPUT);

        assert_eq!((161, 48), solve());
    }
}
