use std::{collections::HashSet, io::BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Space {
    Bug,
    Empty,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '#' => Space::Bug,
            '.' => Space::Empty,
            x => panic!("{}", x),
        }
    }
}

fn parse_map(input: impl BufRead) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|v| {
            // println!("{}", &v);
            v.unwrap().chars().map(Space::from).collect()
        })
        .collect()
}

fn count_surrounding(map: &[Vec<Space>], i: usize, j: usize) -> usize {
    map.get(i.wrapping_sub(1))
        .and_then(|r| r.get(j))
        .map(|x| if matches!(x, Space::Bug) { 1 } else { 0 })
        .unwrap_or(0)
        + map
            .get(i + 1)
            .and_then(|r| r.get(j))
            .map(|x| if matches!(x, Space::Bug) { 1 } else { 0 })
            .unwrap_or(0)
        + map
            .get(i)
            .and_then(|r| r.get(j.wrapping_sub(1)))
            .map(|x| if matches!(x, Space::Bug) { 1 } else { 0 })
            .unwrap_or(0)
        + map
            .get(i)
            .and_then(|r| r.get(j + 1))
            .map(|x| if matches!(x, Space::Bug) { 1 } else { 0 })
            .unwrap_or(0)
}

fn step(map: &[Vec<Space>]) -> Vec<Vec<Space>> {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, x)| match (x, count_surrounding(&map, i, j)) {
                    (Space::Bug, 1) => Space::Bug,
                    (Space::Empty, 1 | 2) => Space::Bug,
                    (Space::Bug, _) => Space::Empty,
                    (x, _) => x.clone(),
                })
                .collect()
        })
        .collect()
}

fn biodiversity(map: &[Vec<Space>]) -> usize {
    map.iter()
        .flatten()
        .enumerate()
        .map(|(i, s)| {
            if matches!(s, Space::Bug) {
                2usize.pow(i as u32)
            } else {
                0
            }
        })
        .sum()
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut map = parse_map(input);
    let mut seen = HashSet::new();
    loop {
        map = step(&map);
        if seen.contains(&map) {
            return biodiversity(&map);
        } else {
            seen.insert(map.clone());
        }
    }
}

pub fn star_two(input: impl BufRead) -> usize {
    let _instructions = parse_map(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT1: &str = "....#
#..#.
#..##
..#..
#....";

    const EXPECTED1: &str = "#..#.
####.
###.#
##.##
.##..";

    #[test]
    fn test_star_one() {
        let input = parse_map(Cursor::new(INPUT1));
        let expected = parse_map(Cursor::new(EXPECTED1));
        let output = step(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_star_two() {}
}
