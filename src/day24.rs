use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

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
        .map(|x| usize::from(matches!(x, Space::Bug)))
        .unwrap_or(0)
        + map
            .get(i + 1)
            .and_then(|r| r.get(j))
            .map(|x| usize::from(matches!(x, Space::Bug)))
            .unwrap_or(0)
        + map
            .get(i)
            .and_then(|r| r.get(j.wrapping_sub(1)))
            .map(|x| usize::from(matches!(x, Space::Bug)))
            .unwrap_or(0)
        + map
            .get(i)
            .and_then(|r| r.get(j + 1))
            .map(|x| usize::from(matches!(x, Space::Bug)))
            .unwrap_or(0)
}

fn step(map: &[Vec<Space>]) -> Vec<Vec<Space>> {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, x)| match (x, count_surrounding(map, i, j)) {
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

fn get_adjacent(
    pos: &(usize, usize, isize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize, isize)> {
    let mut v = Vec::new();

    if pos.0 == height / 2 && pos.1 == width / 2 {
        panic!("Bad position");
    }

    // Above
    if pos.0 == height / 2 + 1 && pos.1 == width / 2 {
        v.extend((0..width).map(|w| (height - 1, w, pos.2 + 1)));
    } else if pos.0 == 0 {
        v.push((height / 2 - 1, width / 2, pos.2 - 1));
    } else {
        v.push((pos.0 - 1, pos.1, pos.2));
    }

    // Below
    if pos.0 == height / 2 - 1 && pos.1 == width / 2 {
        v.extend((0..width).map(|w| (0, w, pos.2 + 1)));
    } else if pos.0 == height - 1 {
        v.push((height / 2 + 1, width / 2, pos.2 - 1));
    } else {
        v.push((pos.0 + 1, pos.1, pos.2));
    }

    // Left
    if pos.0 == height / 2 && pos.1 == width / 2 + 1 {
        v.extend((0..height).map(|h| (h, width - 1, pos.2 + 1)));
    } else if pos.1 == 0 {
        v.push((height / 2, width / 2 - 1, pos.2 - 1));
    } else {
        v.push((pos.0, pos.1 - 1, pos.2));
    }

    // Right
    if pos.0 == height / 2 && pos.1 == width / 2 - 1 {
        v.extend((0..height).map(|h| (h, 0, pos.2 + 1)));
    } else if pos.1 == width - 1 {
        v.push((height / 2, width / 2 + 1, pos.2 - 1));
    } else {
        v.push((pos.0, pos.1 + 1, pos.2));
    }

    v
}

fn step2(
    bug_positions: HashSet<(usize, usize, isize)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize, isize)> {
    let mut map_count = HashMap::new();

    for pos in &bug_positions {
        for new_pos in get_adjacent(pos, width, height) {
            *map_count.entry(new_pos).or_insert(0) += 1;
        }
    }

    let mut new_map = HashSet::new();

    for pos in &bug_positions {
        if map_count.get(pos).unwrap_or(&0) == &1 {
            new_map.insert(*pos);
        }
    }

    for (pos, count) in map_count {
        match (bug_positions.contains(&pos), count) {
            (true, 1) => {
                new_map.insert(pos);
            }
            (false, 1 | 2) => {
                new_map.insert(pos);
            }
            (_, _) => {}
        }
    }
    new_map
}

pub fn star_two(input: impl BufRead) -> usize {
    let map = parse_map(input);
    let mut bug_positions: HashSet<(usize, usize, isize)> = map
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_j, s)| s == &Space::Bug)
                .map(move |(j, _s)| (i, j, 0))
        })
        .collect();
    for _i in 0..200 {
        bug_positions = step2(bug_positions, 5, 5);
    }
    bug_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Cursor, iter::FromIterator};

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
    fn test_get_adjacent() {
        assert_eq!(
            HashSet::<(usize, usize, isize)>::from_iter(get_adjacent(&(3, 3, 0), 5, 5).into_iter()),
            HashSet::from_iter(vec![(2, 3, 0), (3, 2, 0), (4, 3, 0), (3, 4, 0)])
        );

        assert_eq!(
            get_adjacent(&(0, 3, 0), 5, 5),
            vec![(1, 2, -1), (1, 3, 0), (0, 2, 0), (0, 4, 0)]
        );

        assert_eq!(
            get_adjacent(&(0, 4, 0), 5, 5),
            vec![(1, 2, -1), (1, 4, 0), (0, 3, 0), (2, 3, -1)]
        );

        assert_eq!(
            get_adjacent(&(2, 3, 0), 5, 5),
            vec![
                (1, 3, 0),
                (3, 3, 0),
                (0, 4, 1),
                (1, 4, 1),
                (2, 4, 1),
                (3, 4, 1),
                (4, 4, 1),
                (2, 4, 0)
            ]
        );

        assert_eq!(
            get_adjacent(&(2, 3, 0), 5, 5),
            vec![
                (1, 3, 0),
                (3, 3, 0),
                (0, 4, 1),
                (1, 4, 1),
                (2, 4, 1),
                (3, 4, 1),
                (4, 4, 1),
                (2, 4, 0)
            ]
        );
    }

    #[test]
    fn test_star_two() {}
}
