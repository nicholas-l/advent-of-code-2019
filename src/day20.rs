use std::{cmp::Reverse, collections::HashMap, io::BufRead};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Teleport(char, char),
}

fn get_teleporter(map: &[Vec<char>], x: usize, y: usize) -> Option<(char, char)> {
    if map[y - 2][x].is_ascii_alphabetic() && map[y - 1][x].is_ascii_alphabetic() {
        return Some((map[y - 2][x], map[y - 1][x]));
    }

    if map[y + 1][x].is_ascii_alphabetic() && map[y + 2][x].is_ascii_alphabetic() {
        return Some((map[y + 1][x], map[y + 2][x]));
    }

    if map[y][x - 2].is_ascii_alphabetic() && map[y][x - 1].is_ascii_alphabetic() {
        return Some((map[y][x - 2], map[y][x - 1]));
    }

    if map[y][x + 1].is_ascii_alphabetic() && map[y][x + 2].is_ascii_alphabetic() {
        return Some((map[y][x + 1], map[y][x + 2]));
    }

    None
}

fn get_map(input: impl BufRead) -> Vec<Vec<Tile>> {
    let raw_map: Vec<Vec<char>> = input
        .lines()
        .map(|v| {
            // println!("{}", &v);
            v.unwrap().chars().collect::<Vec<char>>()
        })
        .collect();
    let raw_height = raw_map.len();
    raw_map
        .iter()
        .enumerate()
        .skip(2)
        .take(raw_height - 4)
        .map(|(y, row)| {
            let raw_width = row.len();
            row.iter()
                .enumerate()
                .skip(2)
                .take(raw_width - 4)
                .map(|(x, c)| {
                    match *c {
                        '.' => {
                            // Check if teleporter
                            get_teleporter(&raw_map, x, y)
                                .map(|(a, b)| Tile::Teleport(a, b))
                                .unwrap_or(Tile::Empty)
                        }
                        '#' => Tile::Wall,
                        'A'..='Z' | 'a'..='z' => Tile::Wall,
                        ' ' => Tile::Wall,
                        c => panic!("{:?} at ({}, {})", c, x, y),
                    }
                })
                .collect()
        })
        .collect()
}

pub fn star_one(input: impl BufRead) -> usize {
    let map = get_map(input);
    let mut teleporter_positions = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // match c {
            //     Tile::Wall => print!("#"),
            //     Tile::Empty => print!("."),
            //     Tile::Teleport(a, _) => print!("{}", a),
            // }
            if let Tile::Teleport(_a, _b) = c {
                teleporter_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
        // println!()
    }
    // println!("{:?}", teleporter_positions);
    let start = teleporter_positions.get(&Tile::Teleport('A', 'A')).unwrap()[0];

    let mut stack = vec![
        ((start.0.wrapping_sub(1), start.1), 1),
        ((start.0 + 1, start.1), 1),
        ((start.0, start.1.wrapping_sub(1)), 1),
        ((start.0, start.1 + 1), 1),
    ];

    let mut costs = vec![vec![usize::MAX; map[0].len()]; map.len()];

    costs[start.1][start.0] = 0;

    while let Some(((x, y), steps)) = stack.pop() {
        // println!("Looking at ({}, {})", x, y);
        match map.get(y).and_then(|row| row.get(x)) {
            None => {}
            Some(Tile::Wall) => {}

            Some(Tile::Empty) => {
                if costs[y][x] > steps {
                    costs[y][x] = steps;
                    // add surrounding tiles
                    stack.push(((x - 1, y), steps + 1));
                    stack.push(((x + 1, y), steps + 1));

                    stack.push(((x, y - 1), steps + 1));
                    stack.push(((x, y + 1), steps + 1));
                }
            }
            Some(Tile::Teleport(a, b)) => {
                if costs[y][x] > steps {
                    costs[y][x] = steps;
                    // add teleported position to stack
                    let new_positions = teleporter_positions.get(&map[y][x]).unwrap();
                    if !(a == &'Z' && b == &'Z') {
                        assert_eq!(new_positions.len(), 2, "Panic'd at {} {}", a, b);
                        let (new_x, new_y) = new_positions.iter().find(|&p| p != &(x, y)).unwrap();
                        costs[*new_y][*new_x] = steps + 1;
                        let x = *new_x;
                        let y = *new_y;
                        stack.push(((x.wrapping_sub(1), y), steps + 2));
                        stack.push(((x + 1, y), steps + 2));

                        stack.push(((x, y.wrapping_sub(1)), steps + 2));
                        stack.push(((x, y + 1), steps + 2));
                    }
                }
            }
        }
    }
    let end = teleporter_positions.get(&Tile::Teleport('Z', 'Z')).unwrap()[0];
    costs[end.1][end.0]
}

pub fn star_two(input: impl BufRead) -> usize {
    let map = get_map(input);
    let mut teleporter_positions = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // match c {
            //     Tile::Wall => print!("#"),
            //     Tile::Empty => print!("."),
            //     Tile::Teleport(a, _) => print!("{}", a),
            // }
            if let Tile::Teleport(_a, _b) = c {
                teleporter_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
        // println!()
    }

    let start = teleporter_positions.get(&Tile::Teleport('A', 'A')).unwrap()[0];

    let mut stack = vec![
        ((start.0.wrapping_sub(1), start.1), 0, 1),
        ((start.0 + 1, start.1), 0, 1),
        ((start.0, start.1.wrapping_sub(1)), 0, 1),
        ((start.0, start.1 + 1), 0, 1),
    ];

    let mut costs = vec![vec![HashMap::<usize, usize>::default(); map[0].len()]; map.len()];

    costs[start.1][start.0].insert(0, 0);

    while let Some(((x, y), level, steps)) = stack.pop() {
        // println!("Looking at ({}, {})", x, y);
        match (level, map.get(y).and_then(|row| row.get(x))) {
            (_, None) => {}
            (_level, Some(Tile::Wall)) => {}

            (level, Some(Tile::Empty)) => {
                if *costs[y][x].get(&level).unwrap_or(&usize::MAX) > steps {
                    costs[y][x].insert(level, steps);
                    // add surrounding tiles
                    stack.push(((x - 1, y), level, steps + 1));
                    stack.push(((x + 1, y), level, steps + 1));

                    stack.push(((x, y - 1), level, steps + 1));
                    stack.push(((x, y + 1), level, steps + 1));
                }
            }
            (0, Some(Tile::Teleport('Z', 'Z'))) => {
                if *costs[y][x].get(&level).unwrap_or(&usize::MAX) > steps {
                    costs[y][x].insert(level, steps);
                    // add surrounding tiles
                    stack.push(((x.wrapping_sub(1), y), level, steps + 1));
                    stack.push(((x + 1, y), level, steps + 1));

                    stack.push(((x, y.wrapping_sub(1)), level, steps + 1));
                    stack.push(((x, y + 1), level, steps + 1));
                }
                break;
            }
            (_, Some(Tile::Teleport('Z', 'Z'))) => {}
            (_, Some(Tile::Teleport('A', 'A'))) => {}
            (level, Some(Tile::Teleport(a, b))) => {
                let is_outer = x == 0 || y == 0 || x == (map[0].len() - 1) || y == (map.len() - 1);
                if *costs[y][x].get(&level).unwrap_or(&usize::MAX) > steps
                    && !(level == 0 && is_outer)
                {
                    costs[y][x].insert(level, steps);
                    // add teleported position to stack
                    let new_positions = teleporter_positions.get(&map[y][x]).unwrap();
                    let new_level = if is_outer { level - 1 } else { level + 1 };
                    assert_eq!(new_positions.len(), 2, "Panic'd at {} {}", a, b);
                    let (new_x, new_y) = new_positions.iter().find(|&p| p != &(x, y)).unwrap();
                    costs[*new_y][*new_x].insert(new_level, steps + 1);
                    let x = *new_x;
                    let y = *new_y;
                    stack.push(((x.wrapping_sub(1), y), new_level, steps + 2));
                    stack.push(((x + 1, y), new_level, steps + 2));

                    stack.push(((x, y.wrapping_sub(1)), new_level, steps + 2));
                    stack.push(((x, y + 1), new_level, steps + 2));
                }
            }
        }
        stack.sort_by_cached_key(|x| Reverse(x.2));
    }
    let end = teleporter_positions.get(&Tile::Teleport('Z', 'Z')).unwrap()[0];
    *costs[end.1][end.0].get(&0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT1: &str = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    const INPUT2: &str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    const INPUT3: &str = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT1)), 23);
        assert_eq!(star_one(Cursor::new(INPUT2)), 58);
    }

    #[test]
    fn test_star_two() {
        // assert_eq!(star_two(Cursor::new(INPUT1)), 23);
        assert_eq!(star_two(Cursor::new(INPUT3)), 396);
    }
}
