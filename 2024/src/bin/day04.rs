use advtools::grid::*;
use advtools::input;
use advtools::prelude::*;

fn main() {
    let (part1, part2) = solve();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn solve() -> (usize, usize) {
    let grid = Grid::new(input::lines().map(|line| line.chars()));

    let mut part1 = 0;
    let mut part2 = 0;

    for pos in grid.positions() {
        let get = |dir| grid.get(&(pos + dir)?);

        if grid[pos] == 'X' {
            for dir in DIRECTIONS {
                let word = (1..4).map_while(|i| get(dir * i)).collect_array::<3>();

                if let Some(['M', 'A', 'S']) = word {
                    part1 += 1;
                }
            }
        }

        if grid[pos] == 'A' {
            if let (
                (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')),
                (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')),
            ) = ((get(UL), get(DR)), (get(UR), get(DL)))
            {
                part2 += 1;
            }
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test() {
        input::set(INPUT);

        let (part1, part2) = solve();

        assert_eq!(part1, 18);
        assert_eq!(part2, 9);
    }
}
