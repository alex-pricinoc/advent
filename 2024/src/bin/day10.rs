use advtools::grid::{Grid, Pos};
use advtools::input;
use advtools::prelude::*;

fn main() {
    println!("{}", solve());
}

fn walk(grid: &Grid<u32>, from: Pos) -> Vec<Pos> {
    let mut trails = vec![];

    for n in grid.neighbours(from).filter(|&p| grid[p] == grid[from] + 1) {
        if grid[n] == 9 {
            trails.push(n);
        } else {
            trails.extend(walk(grid, n));
        }
    }

    trails
}

fn solve() -> Solution<usize, usize> {
    let grid = Grid::new(input::lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap())));

    let trails = grid
        .positions()
        .filter(|&p| grid[p] == 0)
        .map(|s| walk(&grid, s));

    let mut part1 = 0;
    let mut part2 = 0;

    for t in trails {
        part1 += t.iter().unique().count();
        part2 += t.len();
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_trailheads() {
        input::set(
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );

        assert_eq!(solve(), Solution(36, 81));
    }
}
