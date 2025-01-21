use std::fmt::Display;

use advtools::grid::*;
use advtools::input;
use advtools::prelude::*;

fn main() {
    println!("{}", solve());
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Cell {
    Antenna(char),
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Antenna(c) => write!(f, "{c}"),
            Cell::Empty => write!(f, "."),
        }
    }
}

fn solve() -> Solution<usize, usize> {
    let grid = Grid::new(input::lines().map(|l| {
        l.chars().map(|c| match c {
            '.' | '#' => Cell::Empty,
            c => Cell::Antenna(c),
        })
    }));

    let frequencies = grid
        .positions()
        .filter(|&c| grid[c] != Cell::Empty)
        .map(|pos| (grid[pos], pos))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert_with(Vec::new).push(v);
            acc
        });

    let mut antinodes = HashSet::new();
    let mut antinodes_harmonics = HashSet::new();

    let combinations = frequencies
        .values()
        .flat_map(|p| p.iter().cartesian_product(p.iter()))
        .filter(|&(a, b)| a != b);

    for (a, b) in combinations {
        let diff = a - b;

        let next = |l: &Pos, r: Pos<isize>| (l + r).take_if(|p| grid.get(p).is_some());

        if let Some(pos) = next(b, diff) {
            antinodes.insert(pos);
        }

        let mut f = 1;

        while let Some(pos) = next(a, diff * f) {
            antinodes_harmonics.insert(pos);
            f += 1;
        }
    }

    let part1 = antinodes.len();
    let part2 = antinodes_harmonics.len();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name() {
        input::set(
            "\
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
",
        );

        assert_eq!(Solution(14, 34), solve());
    }
}
