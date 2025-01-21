use advtools::grid::*;
use advtools::input;
use advtools::prelude::*;

#[derive(Debug, Clone)]
struct Day {
    grid: Grid<char>,
}

impl Day {
    fn new() -> Self {
        Self {
            grid: Grid::new(input::lines().map(|l| l.chars())),
        }
    }

    fn find_guard(&self) -> Pos {
        self.grid.find_pos(|p| *p == '^').unwrap()
    }

    fn iter_from(&self, dir: Direction, pos: Pos) -> Iter<'_> {
        Iter {
            pos,
            dir,
            grid: &self.grid,
        }
    }

    fn iter(&self) -> Iter<'_> {
        Iter {
            pos: self.find_guard(),
            dir: Direction::Up,
            grid: &self.grid,
        }
    }
}

#[derive(Debug, Clone)]
struct Iter<'a> {
    pos: Pos,
    dir: Direction,
    grid: &'a Grid<char>,
}

impl Iter<'_> {
    fn is_loop(&mut self) -> bool {
        let mut visited = HashSet::new();

        let mut prev = self.pos;

        for d in self {
            if prev == d.1 && !visited.insert(d) {
                return true;
            }
            prev = d.1;
        }

        false
    }
}

impl Iterator for Iter<'_> {
    type Item = (Direction, Pos);

    fn next(&mut self) -> Option<Self::Item> {
        let new_pos = (self.pos + self.dir.to_pos())?;

        if *self.grid.get(&new_pos)? == '#' {
            self.dir.turn_right();
        } else {
            self.pos = new_pos;
        }

        Some((self.dir, self.pos))
    }
}

impl<'a> IntoIterator for &'a Day {
    type Item = (Direction, Pos);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn solve() -> Solution<usize, usize> {
    let day = Day::new();

    let part1 = day.iter().unique_by(|p| p.1).count();

    let mut part2 = 0;

    let mut visited = HashSet::new();

    let it = day.iter();
    visited.insert(it.pos);

    let mut prev = (it.dir, it.pos);
    let mut day = day.clone();

    for (dir, pos) in it {
        if !visited.contains(&pos) {
            day.grid[pos] = '#';

            let mut it = day.iter_from(prev.0, prev.1);

            if it.is_loop() {
                part2 += 1;
            }

            day.grid[pos] = '.';
        }

        visited.insert(pos);
        prev = (dir, pos);
    }

    Solution(part1, part2)
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_day() {
        input::set(INPUT);
        assert_eq!(Solution(41, 6), solve());
    }
}
