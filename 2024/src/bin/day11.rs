use advtools::num::Integer;
use advtools::prelude::*;
use advtools::{digits, input, Solution};

fn main() {
    println!("{}", solve(25, 75));
}

fn solve(one: u8, two: u8) -> Solution<usize, usize> {
    let list = input::string()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = list.iter().cloned().map(|s| count_stones(s, one)).sum();
    let part2 = list.iter().cloned().map(|s| count_stones(s, two)).sum();

    Solution(part1, part2)
}

#[memoize]
fn count_stones(stone: usize, depth: u8) -> usize {
    if depth == 0 {
        1
    } else if stone == 0 {
        count_stones(1, depth - 1)
    } else if digits(stone).is_odd() {
        count_stones(stone * 2024, depth - 1)
    } else {
        let (a, b) = split(stone);
        count_stones(a, depth - 1) + count_stones(b, depth - 1)
    }
}

fn split(number: usize) -> (usize, usize) {
    number.div_rem(&10_usize.pow(digits(number) / 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        input::set("125 17");

        assert_eq!(solve(3, 25), Solution(5, 55312));
    }
}
