use std::cmp::Ordering;

use advtools::input;
use advtools::prelude::*;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let (rules, updates) = input::string().split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| (l[0..2].parse().unwrap(), l[3..].parse().unwrap()))
        .collect::<HashSet<(usize, usize)>>();

    let updates = updates.lines().map(|u| {
        u.split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
    });

    let compare = |x: &usize, y: &usize| {
        if rules.contains(&(*x, *y)) {
            Ordering::Less
        } else if rules.contains(&(*y, *x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    for mut update in updates {
        if update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
            part1 += update[update.len() / 2];
        } else {
            update.sort_by(compare);
            part2 += update[update.len() / 2];
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
