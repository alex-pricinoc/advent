use advtools::input;
use advtools::prelude::*;

fn main() {
    let mut first = vec![];
    let mut second = vec![];
    let mut second_count = HashMap::new();

    input::parse_lines().for_each(|nums: (i64, i64)| {
        first.push(nums.0);
        second.push(nums.1);
        *second_count.entry(nums.1).or_default() += 1;
    });

    first.sort();
    second.sort();

    let distance = first
        .iter()
        .zip(&second)
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u64>();

    let score = first
        .iter()
        .map(|i| second_count.get(i).unwrap_or(&0) * i)
        .sum::<i64>();

    println!("part 1: {distance}");
    println!("part 2: {score}");
}
