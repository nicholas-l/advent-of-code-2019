use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Teleport {
    Inner(char, char),
    Outer(char, char),
}

impl Teleport {
    fn get_chars(&self) -> (char, char) {
        match self {
            Teleport::Inner(a, b) => (*a, *b),
            Teleport::Outer(a, b) => (*a, *b),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Teleport(Teleport),
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

fn get_map(
    input: impl BufRead,
    map_teleport: &impl Fn((char, char), (usize, usize), (usize, usize)) -> Tile,
) -> Vec<Vec<Tile>> {
    let raw_map: Vec<Vec<char>> = input
        .lines()
        .map(|v| {
            // println!("{}", &v);
            v.unwrap().chars().collect::<Vec<char>>()
        })
        .collect();
    let raw_height = raw_map.len();
    let map_height = raw_height - 4;
    raw_map
        .iter()
        .enumerate()
        .skip(2)
        .take(raw_height - 4)
        .map(|(y, row)| {
            let raw_width = row.len();
            let map_width = raw_width - 4;

            row.iter()
                .enumerate()
                .skip(2)
                .take(raw_width - 4)
                .map(|(x, c)| {
                    match *c {
                        '.' => {
                            // Check if teleporter
                            get_teleporter(&raw_map, x, y)
                                .map(|(a, b)| {
                                    map_teleport((a, b), (x - 2, y - 2), (map_width, map_height))
                                })
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

fn get_graph(map: &[Vec<Tile>]) -> HashMap<&Teleport, HashMap<&Teleport, usize>> {
    let mut teleporters = Vec::new();

    let deltas = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // match c {
            //     Tile::Wall => print!("#"),
            //     Tile::Empty => print!("."),
            //     Tile::Teleport(a, _) => print!("{}", a),
            // }
            if let Tile::Teleport(teleport) = c {
                teleporters.push((teleport, (x, y)));
            }
        }
        // println!()
    }

    // dbg!(&teleporters);

    let mut graph = HashMap::new();

    for (from_tile, start) in &teleporters {
        let mut costs = HashMap::new();

        let mut stack = VecDeque::new();
        stack.push_back((*start, 0));
        // stack.push_back(((start.0.wrapping_sub(1), start.1), 1));
        // stack.push_back(((start.0 + 1, start.1), 1));
        // stack.push_back(((start.0, start.1.wrapping_sub(1)), 1));
        // stack.push_back(((start.0, start.1 + 1), 1));

        let mut visited = HashSet::new();

        while let Some(((x, y), steps)) = stack.pop_front() {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));
                for (d_x, d_y) in &deltas {
                    // println!("Looking at ({}, {})", x, y);
                    let n_y = (y as isize + d_y) as usize;
                    let n_x = (x as isize + d_x) as usize;
                    match map.get(n_y).and_then(|row| row.get(n_x)) {
                        None => {}
                        Some(tile) => {
                            match tile {
                                Tile::Wall => {}
                                Tile::Empty => {
                                    // add surrounding tiles
                                    stack.push_back(((n_x, n_y), steps + 1));
                                }

                                Tile::Teleport(to_tile) => {
                                    // This should be the first time we have seend it so it should be the quickest path.
                                    if !visited.contains(&(n_x, n_y))
                                        && costs.get(to_tile).unwrap_or(&usize::MAX) > &steps
                                    {
                                        costs.insert(to_tile, steps + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let to_tile = match from_tile {
            Teleport::Inner(a, b) => teleporters.iter().find(|t| t.0 == &Teleport::Outer(*a, *b)),
            Teleport::Outer(a, b) => teleporters.iter().find(|t| t.0 == &Teleport::Inner(*a, *b)),
        };
        if let Some(to_tile) = to_tile {
            costs.insert(to_tile.0, 1);
        }

        graph.insert(*from_tile, costs);
    }
    graph
}

pub fn star_one(input: impl BufRead) -> usize {
    let map_teleport = |(a, b), _, _| Tile::Teleport(Teleport::Outer(a, b));
    let map = get_map(input, &map_teleport);
    let mut teleporter_positions = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // match c {
            //     Tile::Wall => print!("#"),
            //     Tile::Empty => print!("."),
            //     Tile::Teleport(a, _) => print!("{}", a),
            // }
            if let Tile::Teleport(_teleport) = c {
                teleporter_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
        // println!()
    }
    // println!("{:?}", teleporter_positions);
    let start = teleporter_positions
        .get(&Tile::Teleport(Teleport::Outer('A', 'A')))
        .unwrap()[0];

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
            Some(Tile::Teleport(_teleport)) => {
                if costs[y][x] > steps {
                    costs[y][x] = steps;
                    // add teleported position to stack
                    let new_positions = teleporter_positions.get(&map[y][x]).unwrap();
                    if new_positions.len() > 1 {
                        assert_eq!(new_positions.len(), 2, "Panic'd at {} {}", x, y);
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
    let end = teleporter_positions
        .get(&Tile::Teleport(Teleport::Outer('Z', 'Z')))
        .unwrap()[0];
    costs[end.1][end.0]
}

pub fn star_two(input: impl BufRead) -> usize {
    let map_teleport = |(a, b), (x, y), (width, height)| {
        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            Tile::Teleport(Teleport::Outer(a, b))
        } else {
            Tile::Teleport(Teleport::Inner(a, b))
        }
    };
    let map = get_map(input, &map_teleport);
    let mut teleporter_positions = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // match c {
            //     Tile::Wall => print!("#"),
            //     Tile::Empty => print!("."),
            //     Tile::Teleport(a, _) => print!("{}", a),
            // }
            if let Tile::Teleport(_teleport) = c {
                teleporter_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
        // println!()
    }
    let graph = get_graph(&map);
    dbg!(&graph);

    let start = Teleport::Outer('A', 'A');

    let mut costs = HashMap::new();

    let mut stack = vec![(&start, 0, 0, vec![(&start, 0, 0)])];

    while let Some((current_teleport, current_depth, current_cost, path)) = stack.pop() {
        // dbg!((current_teleport, current_depth));
        if current_teleport == &Teleport::Outer('Z', 'Z') && current_depth == 0 {
            for (t, d, c) in path {
                let (a, b) = t.get_chars();
                println!("({}, {}) {} - {}", a, b, d, c);
            }
            return current_cost;
        }
        let cost = *costs
            .get(&(current_depth, current_teleport))
            .unwrap_or(&usize::MAX);
        if cost >= current_cost {
            costs.insert((current_depth, current_teleport), current_cost);
            // dbg!(graph.get(&current_teleport));
            for (next_teleport, &delta_cost) in graph.get(&current_teleport).unwrap() {
                // dbg!(next_teleport);
                // dbg!(matches!(current_teleport, Teleport::Outer(_, _)));
                // dbg!(current_depth == 0);
                // dbg!(delta_cost == 1);
                if !(matches!(current_teleport, Teleport::Outer(_, _))
                    && current_depth == 0
                    && delta_cost == 1)
                {
                    let next_depth = if delta_cost == 1 {
                        match next_teleport {
                            Teleport::Outer(_, _) => current_depth + 1,
                            Teleport::Inner(_, _) => current_depth - 1,
                        }
                    } else {
                        current_depth
                    };
                    let new_cost = current_cost + delta_cost;
                    let mut new_path = path.clone();
                    new_path.push((next_teleport, next_depth, new_cost));
                    stack.push((next_teleport, next_depth, new_cost, new_path));
                }
            }
        }
        stack.sort_by_cached_key(|x| Reverse(x.2));
    }

    unreachable!();
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
