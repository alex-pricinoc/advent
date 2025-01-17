use advtools::input;
use advtools::prelude::*;

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn is_safe(report: &[i64], skip: Option<usize>) -> bool {
    let (min, max) = report
        .iter()
        .enumerate()
        .filter(|&(i, _)| skip != Some(i))
        .tuple_windows()
        .map(|((_, a), (_, b))| b - a)
        .minmax()
        .into_option()
        .unwrap();

    (min >= -3 && max <= -1) || (min >= 1 && max <= 3)
}

fn part_1() -> usize {
    input::parse_lines()
        .filter(|r: &Vec<i64>| is_safe(r, None))
        .count()
}

fn part_2() -> usize {
    input::parse_lines()
        .filter(|r: &Vec<i64>| is_safe(r, None) || (0..r.len()).any(|i| is_safe(r, Some(i))))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part_1() {
        input::set(INPUT);

        assert_eq!(part_1(), 2);
    }

    #[test]
    fn test_part_2() {
        input::set(INPUT);

        assert_eq!(part_2(), 4);
    }
}
