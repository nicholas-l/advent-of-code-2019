use std::cmp::Reverse;
use std::{
    collections::{HashMap, VecDeque},
    f64::consts::PI,
    io::BufRead,
};

enum Space {
    Asteroid,
    Empty,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '#' => Space::Asteroid,
            '.' => Space::Empty,
            x => panic!("Unable to map char {} to space", x),
        }
    }
}

fn convert_map(data: impl BufRead) -> Vec<Vec<Space>> {
    data.lines()
        .map(|line| line.unwrap().chars().map(Space::from).collect())
        .collect()
}

fn calculate_angle(origin_x: usize, origin_y: usize, point_x: usize, point_y: usize) -> isize {
    ((((point_y as f64 - origin_y as f64).atan2(point_x as f64 - origin_x as f64)
        + 2.5 * PI as f64)
        % (2.0 * PI)
        * 1000.0)
        .floor()) as isize
}

fn get_visible_from(
    map: &[Vec<Space>],
    i: usize,
    j: usize,
) -> HashMap<isize, Vec<(usize, usize, usize)>> {
    let mut angles = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if !(y == j && x == i) {
                match cell {
                    Space::Asteroid => {
                        let angle = calculate_angle(i, j, x, y);
                        // assuming it just for ordering not actual distance calcs
                        let distance = ((x as isize - i as isize).pow(2)
                            + (y as isize - j as isize).pow(2))
                            as usize;
                        let distances = angles.entry(angle).or_insert_with(Vec::new);
                        distances.push((x, y, distance));
                    }
                    Space::Empty => {}
                }
            }
        }
    }
    angles
}

fn get_max_visible(map: &[Vec<Space>]) -> Option<(usize, usize, usize)> {
    map.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, cell)| match cell {
                    Space::Asteroid => Some((x, y, get_visible_from(map, x, y).len())),
                    Space::Empty => None,
                })
                .max_by_key(|x| x.2)
        })
        .max_by_key(|x| x.2)
}

fn spin(map: &[Vec<Space>], x: usize, y: usize) -> Vec<(usize, usize, usize)> {
    let mut angles: VecDeque<_> = get_visible_from(map, x, y).into_iter().collect();
    angles.make_contiguous().sort_by_key(|x| x.0);
    // dbg!(&angles);
    let mut destroyed = Vec::new();
    while let Some((angle, mut asteroids)) = angles.pop_front() {
        // dbg!(&angle);
        asteroids.sort_by_key(|x| Reverse(x.2));
        destroyed.push(asteroids.pop().unwrap());
        if !asteroids.is_empty() {
            angles.push_back((angle, asteroids));
        }
    }
    destroyed
}

pub fn star_one(input: impl BufRead) -> usize {
    let map = convert_map(input);
    get_max_visible(&map).unwrap().2
}

pub fn star_two(input: impl BufRead) -> usize {
    let map = convert_map(input);
    let (x, y, _visible) = get_max_visible(&map).unwrap();
    spin(&map, x, y)
        .get(199)
        .map(|c| {
            dbg!(&c);
            let (x, y, _d) = c;
            100 * x + y
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const DATA3: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn test_position_8() {
        let data = ".#..#
.....
#####
....#
...##";
        let map = convert_map(Cursor::new(data));
        assert_eq!(get_visible_from(&map, 3, 4).len(), 8)
    }

    #[test]
    fn test_star_one() {
        let data: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let res = star_one(Cursor::new(data));
        assert_eq!(res, 33);
    }

    #[test]
    fn test_spin() {
        let map = convert_map(Cursor::new(DATA3));
        let destroyed = spin(&map, 11, 13);
        let (x, y, _d) = destroyed[0];
        assert_eq!((x, y), (11, 12));
        let (x, y, _d) = destroyed[1];
        assert_eq!((x, y), (12, 1));
        let (x, y, _d) = destroyed[2];
        assert_eq!((x, y), (12, 2));
        let (x, y, _d) = destroyed[9];
        assert_eq!((x, y), (12, 8));
        let (x, y, _d) = destroyed[99];
        assert_eq!((x, y), (10, 16));
        let (x, y, _d) = destroyed[199];
        assert_eq!((x, y), (8, 2));
    }

    #[test]
    fn test_spin2() {
        let data = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let map = convert_map(Cursor::new(data));
        let destroyed = spin(&map, 8, 3);
        for i in &destroyed {
            println!("{:?}", i);
        }
        let expected = vec![
            (8, 1),
            (9, 0),
            (9, 1),
            (10, 0),
            (9, 2),
            (11, 1),
            (12, 1),
            (11, 2),
        ];
        for (i, e) in expected.into_iter().enumerate() {
            let (x, y, _d) = destroyed[i];
            assert_eq!((x, y), e);
        }
        let (x, y, _d) = destroyed[17];
        assert_eq!((x, y), (4, 4));
        let (x, y, _d) = destroyed[18];
        assert_eq!((x, y), (2, 4));
        let (x, y, _d) = destroyed[19];
        assert_eq!((x, y), (2, 3));
        let (x, y, _d) = destroyed[20];
        assert_eq!((x, y), (0, 2));
        let (x, y, _d) = destroyed[26];
        assert_eq!((x, y), (5, 1));

        let (x, y, _d) = destroyed[29];
        assert_eq!((x, y), (7, 0));
        let (x, y, _d) = destroyed[30];
        assert_eq!((x, y), (8, 0));
    }

    #[test]
    fn test_star_two() {
        let res = star_two(Cursor::new(DATA3));
        assert_eq!(res, 802);
    }
}
