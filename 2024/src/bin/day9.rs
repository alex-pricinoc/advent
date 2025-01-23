use advtools::input;
use advtools::num::Integer;
use advtools::prelude::*;

fn main() {
    println!("{}", solve());
}

#[derive(Debug, Clone, Copy)]
struct Chunk {
    file: Option<usize>,
    len: usize,
}

struct Disk {
    data: Vec<Option<usize>>,
}

impl Disk {
    fn checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.map(|c| (i, c)))
            .fold(0, |acc, (i, c)| acc + i * c)
    }
}

impl From<&Vec<Chunk>> for Disk {
    fn from(value: &Vec<Chunk>) -> Self {
        let data = value
            .iter()
            .flat_map(|c| iter::repeat_n(c.file, c.len))
            .collect();

        Self { data }
    }
}

fn solve() -> Solution<usize, usize> {
    let mut id = 0;
    let chunks = input::string()
        .char_indices()
        .map(|(i, c)| {
            let n = c.to_digit(10).unwrap() as usize;
            if i.is_even() {
                id += 1;
                Chunk {
                    file: Some(id - 1),
                    len: n,
                }
            } else {
                Chunk { file: None, len: n }
            }
        })
        .collect::<Vec<_>>();

    let mut disk = Disk::from(&chunks);
    let mut b = 0;
    while b < disk.data.len() {
        if disk.data[b].is_none() {
            while disk.data.last().unwrap().is_none() {
                disk.data.pop().unwrap();
            }
            disk.data[b] = disk.data.pop().unwrap();
        }
        b += 1;
    }

    let part1 = disk.checksum();

    let mut chunks = chunks;
    for i in (0..chunks.len()).rev() {
        if chunks[i].file.is_none() {
            continue;
        }

        if let Some(j) =
            (0..i).find(|&j| chunks[j].file.is_none() && chunks[i].len <= chunks[j].len)
        {
            let chunk = chunks[i];
            chunks[i].file = None;
            chunks[j].len -= chunk.len;
            chunks.insert(j, chunk);
        }
    }

    let part2 = Disk::from(&chunks).checksum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        input::set("2333133121414131402");

        assert_eq!(solve(), Solution(1928, 2858));
    }
}
