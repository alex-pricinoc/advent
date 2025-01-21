use advtools::prelude::*;
use advtools::{digits, input};

fn main() {
    println!("{}", solve());
}

// fn sum(nums: &[usize], ops: &[char]) -> Option<usize> {
//     let total = nums[0];
//     let first = nums[1];
//     let rest = &nums[2..];

//     (0..rest.len())
//         .map(|_| ops)
//         .multi_cartesian_product()
//         .map(|ops| {
//             ops.iter().zip(rest).fold(first, |acc, (op, n)| match op {
//                 '+' => acc + n,
//                 '*' => acc * n,
//                 ' ' => acc * 10usize.pow(digits(*n)) + n,
//                 _ => unimplemented!("{op}"),
//             })
//         })
//         .find(|&s| s == total)
// }

fn sum(nums: &[usize], ops: &[char]) -> Option<usize> {
    fn do_sum(target: usize, acc: usize, nums: &[usize], ops: &[char]) -> Option<usize> {
        if nums.is_empty() {
            return Some(acc);
        }

        ops.iter()
            .filter_map(|op| match op {
                '+' => do_sum(target, acc + nums[0], &nums[1..], ops),
                '*' => do_sum(target, acc * nums[0], &nums[1..], ops),
                ' ' => do_sum(
                    target,
                    acc * 10usize.pow(digits(nums[0])) + nums[0],
                    &nums[1..],
                    ops,
                ),
                _ => unreachable!(),
            })
            .find(|&s| s == target)
    }

    do_sum(nums[0], nums[1], &nums[2..], ops)
}

fn solve() -> Solution<usize, usize> {
    let lines = input::lines().map(|l| l.split(&[':', ' ']).filter_map(|s| s.parse().ok()));
    let lines: Vec<Vec<usize>> = lines.map(|l| l.collect()).collect();

    let part1 = lines.iter().filter_map(|l| sum(l, &['*', '+'])).sum();
    let part2 = lines.iter().filter_map(|l| sum(l, &['*', '+', ' '])).sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_solution() {
        input::set(INPUT);
        assert_eq!(Solution(3749, 11387), solve());
    }
}
