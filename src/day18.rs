use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::BufRead,
};

static DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// Use BTreeSet for keys as HashSet does not implement hash

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    Door(char),
    Key(char),
    Entrance,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '#' => Space::Wall,
            d @ 'A'..='Z' => Space::Door(d),
            k @ 'a'..='z' => Space::Key(k.to_ascii_uppercase()),
            '@' => Space::Entrance,
            x => panic!("Could not parse {} to Space", x),
        }
    }
}

type Position = (usize, usize);

fn get_adjacent(position: &'_ Position) -> impl Iterator<Item = Position> + '_ {
    DIRS.iter()
        .map(move |&x| (position.0 as isize + x.0, position.1 as isize + x.1))
        .filter(|x| x.0 >= 0 && x.1 >= 0)
        .map(|x| (x.0 as usize, x.1 as usize))
}

fn get_acessible(
    map: &[Vec<Space>],
    position: &Position,
    current_keys: &HashSet<char>,
    open_doors: &HashSet<char>,
) -> Vec<(Space, Position, usize)> {
    let mut locations = vec![];

    let mut stack = VecDeque::new();
    stack.push_front((*position, 0));

    let mut visited = HashSet::new();

    while let Some((pos, steps)) = stack.pop_front() {
        if !visited.contains(&pos) {
            visited.insert(pos);
            match map[pos.1][pos.0] {
                Space::Empty | Space::Entrance => {
                    stack.extend(get_adjacent(&pos).map(|x| (x, steps + 1)));
                }
                Space::Wall => {}
                Space::Door(d) => {
                    if current_keys.contains(&d) {
                        locations.push((Space::Door(d), pos, steps));
                    } else if open_doors.contains(&d) {
                        stack.extend(get_adjacent(&pos).map(|x| (x, steps + 1)));
                    }
                }
                Space::Key(k) => {
                    if !current_keys.contains(&k) {
                        locations.push((Space::Key(k), pos, steps))
                    } else {
                        stack.extend(get_adjacent(&pos).map(|x| (x, steps + 1)));
                    }
                }
            }
        }
    }

    locations
}

fn find_entrance(map: &[Vec<Space>]) -> Position {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Space::Entrance = c {
                return (x, y);
            }
        }
    }
    unreachable!()
}

#[derive(Debug, PartialEq, Eq)]
struct Data(
    Position,
    HashSet<char>,
    HashSet<char>,
    usize,
    Vec<(Space, usize)>,
);

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.3.partial_cmp(&self.3)
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.3.cmp(&self.3)
    }
}

fn get_acessible_2(map: &[Vec<Space>], position: &Position) -> Vec<(Position, usize)> {
    let mut locations = vec![];

    let mut stack = VecDeque::new();
    stack.push_front((*position, 0));

    let mut visited = HashSet::new();
    visited.insert(*position);

    while let Some((pos, steps)) = stack.pop_front() {
        for x in get_adjacent(&pos) {
            if !visited.contains(&x) {
                visited.insert(x);
                match map[x.1][x.0] {
                    Space::Empty | Space::Entrance => {
                        stack.push_back((x, steps + 1));
                    }
                    Space::Wall => {}
                    Space::Door(_d) => {
                        locations.push((x, steps + 1));
                    }
                    Space::Key(_k) => {
                        locations.push((x, steps + 1));
                    }
                }
            }
        }
    }

    locations
}

type Cache = HashMap<(Position, BTreeSet<char>), Vec<(Position, usize)>>;

fn get_shortest(map: &[Vec<Space>], position: &Position, needed_keys: usize) -> Option<usize> {
    let mut stack = Vec::new();

    let mut costs: HashMap<(Position, BTreeSet<char>), usize> = HashMap::new();
    costs.insert((*position, BTreeSet::new()), 0);

    stack.push((0, *position, BTreeSet::new()));

    // Cache the result of dijstra method using the current position and the current keys as a cache key.
    let mut cache: Cache = HashMap::new();

    while let Some((cost, position, keys)) = stack.pop() {
        if keys.len() == needed_keys {
            return Some(cost);
        }

        if let Some(&lowest_cost) = costs.get(&(position, keys.clone())) {
            if cost > lowest_cost {
                continue;
            }
        }

        let cache_key = (position, keys.clone());

        let cached_entry = cache
            .entry(cache_key)
            .or_insert_with(|| dijkstra(map, &position, &keys));

        // dbg!(&symbol);
        // dbg!(&position);
        // dbg!(&cached_entry);

        for (next_position, next_cost) in cached_entry.iter() {
            let mut next_keys = keys.clone();
            let next_symbol = &map[next_position.1][next_position.0];
            if let Space::Key(k) = next_symbol {
                next_keys.insert(*k);
            }
            let next_steps = cost + next_cost;

            let distances_entry = costs
                .entry((*next_position, next_keys.clone()))
                .or_insert(usize::MAX);

            if next_steps < *distances_entry {
                *distances_entry = next_steps;

                let next_state = (cost + next_cost, *next_position, next_keys);

                stack.push(next_state);
            }
        }
        // ToDo Change to better structure
        stack.sort_by_cached_key(|x| Reverse(x.0));
        // dbg!(&stack);
    }

    None
}

fn dijkstra(
    data: &[Vec<Space>],
    position: &Position,
    keys: &BTreeSet<char>,
) -> Vec<(Position, usize)> {
    // println!("Running dijkstra with: {:?}, {:?}", position, keys);
    let mut costs = HashMap::new();
    costs.insert(*position, 0);

    let mut stack = vec![(0, *position)];

    let mut accessible_keys = HashSet::new();

    while let Some((cost, position)) = stack.pop() {
        let symbol = &data[position.1][position.0];
        if let Space::Key(k) = symbol {
            if !keys.contains(&k) {
                accessible_keys.insert((position, cost));
                continue;
            }
        }

        if cost <= *costs.get(&position).unwrap_or(&usize::MAX) {
            let locations = get_acessible_2(&data, &position) // TODO: Maybe change to preprocessing this, HashMap?
                .into_iter()
                .filter(|(position, _)| {
                    let next_symbol = &data[position.1][position.0];
                    if let Space::Door(d) = next_symbol {
                        return keys.contains(&d);
                    }
                    true
                });
            for (next_position, next_cost) in locations {
                // dbg!(&next_position);
                let new_cost = next_cost + cost;

                if new_cost < *costs.get(&next_position).unwrap_or(&usize::MAX) {
                    costs.insert(next_position, new_cost);
                    stack.push((new_cost, next_position));
                }
            }
        }
        stack.sort_by_cached_key(|x| x.0)
    }
    accessible_keys
        .into_iter()
        .map(|(k, _)| (k, costs[&k]))
        .collect()
}

pub fn star_one(input: impl BufRead) -> usize {
    let data: Vec<Vec<Space>> = input
        .lines()
        .map(|v| {
            // println!("{:?}", v);
            v.unwrap().chars().map(|c| c.into()).collect()
        })
        .collect();
    // get all keys
    let entrance = find_entrance(&data);
    let keys = HashSet::new();
    let doors = HashSet::new();
    let key_symbols: HashSet<char> = data
        .iter()
        .flat_map(|row| {
            row.iter().filter_map(|c| {
                if let Space::Key(v) = c {
                    Some(*v)
                } else {
                    None
                }
            })
        })
        .collect();
    println!(
        "Access: {:?}",
        get_acessible(&data, &entrance, &keys, &doors)
    );
    get_shortest(&data, &entrance, key_symbols.len()).unwrap()
}

pub fn star_two(input: impl BufRead) -> usize {
    let _data: Vec<Vec<Space>> = input
        .lines()
        .map(|v| {
            // println!("{:?}", v);
            v.unwrap().chars().map(|c| c.into()).collect()
        })
        .collect();
    todo!()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const INPUT2: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const INPUT3: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const INPUT4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const INPUT5: &str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT2)), 86);
        assert_eq!(star_one(Cursor::new(INPUT3)), 132);
        assert_eq!(star_one(Cursor::new(INPUT4)), 136);
        assert_eq!(star_one(Cursor::new(INPUT5)), 81);
    }

    #[test]
    fn test_star_two() {}
}
