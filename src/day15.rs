use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use crate::IntCode;

#[derive(Debug)]
enum Space {
    Unknown,
    Nothing,
    Wall,
    Oxygen,
}

type Position = (isize, isize);

//north (1), south (2), west (3), and east (4)
fn get_next_direction(
    map: &HashMap<Position, Space>,
    position: &Position,
) -> Option<(isize, Position)> {
    if let Space::Unknown = map
        .get(&(position.0, position.1 - 1))
        .unwrap_or(&Space::Unknown)
    {
        return Some((1, (position.0, position.1 - 1)));
    };
    if let Space::Unknown = map
        .get(&(position.0, position.1 + 1))
        .unwrap_or(&Space::Unknown)
    {
        return Some((2, (position.0, position.1 + 1)));
    };
    if let Space::Unknown = map
        .get(&(position.0 - 1, position.1))
        .unwrap_or(&Space::Unknown)
    {
        return Some((3, (position.0 - 1, position.1)));
    };
    if let Space::Unknown = map
        .get(&(position.0 + 1, position.1))
        .unwrap_or(&Space::Unknown)
    {
        return Some((4, (position.0 + 1, position.1)));
    };
    None
}

fn get_back(current_position: Position, proposed_position: Position) -> isize {
    match (
        current_position.0 - proposed_position.0,
        current_position.1 - proposed_position.1,
    ) {
        (0, 1) => 1,
        (0, -1) => 2,
        (1, 0) => 3,
        (-1, 0) => 4,
        _ => panic!(),
    }
}

/*
Explore the whole map and create a HashMap of the environment.
 */
fn get_map(mut computer: IntCode) -> (HashMap<Position, Space>, Position) {
    let mut path = vec![(0, 0)];
    let mut map = HashMap::new();
    map.insert((0, 0), Space::Nothing);
    let mut position = (0, 0);
    let mut oxygen = None;
    loop {
        if let Some((proposed_dir, new_position)) = get_next_direction(&map, &position) {
            let input = vec![proposed_dir];
            computer.set_input(input);
            computer.run(1);
            let output = computer.take_output();
            position = match output[0] {
                // Wall
                0 => {
                    map.insert(new_position, Space::Wall);
                    position
                }
                // Moved
                1 => {
                    map.insert(new_position, Space::Nothing);
                    path.push(new_position);
                    new_position
                }
                // Oxygen
                2 => {
                    map.insert(new_position, Space::Oxygen);
                    path.push(new_position);
                    oxygen.replace(new_position);
                    new_position
                }
                _ => panic!(),
            }
        } else if path.len() > 1 {
            // Backtrack
            let _current_position = path.pop();
            let proposed_position = *path.last().unwrap();
            let proposed_dir = get_back(position, proposed_position);
            let input = vec![proposed_dir];
            computer.set_input(input);
            computer.run(1);
            let output = computer.take_output();
            assert_eq!(output, vec![1]);
            position = proposed_position;
        } else {
            break;
        }
    }
    (map, oxygen.unwrap())
}

pub fn star_one(input: impl BufRead) -> usize {
    let codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            // println!("{}", &v);
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    let computer = IntCode::new(codes, vec![]);

    let (map, oxygen) = get_map(computer);
    /*
    0: The repair droid hit a wall. Its position has not changed.
    1: The repair droid has moved one step in the requested direction.
    2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
    */

    // get shortest path back
    let mut visited = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back((oxygen, 0));

    // Start from the oxygen supply and do a breadth first search of the map to get back to the start (0,0).
    // TODO: Change to BFS.
    while let Some((position, steps)) = stack.pop_front() {
        if !visited.contains(&position) {
            visited.insert(position);
            if position == (0, 0) {
                return steps;
            }
            match map.get(&position).unwrap() {
                Space::Unknown => todo!(),
                Space::Nothing | Space::Oxygen => {
                    stack.push_back(((position.0, position.1 + 1), steps + 1));
                    stack.push_back(((position.0, position.1 - 1), steps + 1));
                    stack.push_back(((position.0 + 1, position.1), steps + 1));
                    stack.push_back(((position.0 - 1, position.1), steps + 1));
                }
                Space::Wall => {}
            }
        }
    }
    // println!("{:?}", map);
    unreachable!()
}

pub fn star_two(input: impl BufRead) -> usize {
    let codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            // println!("{}", &v);
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    let computer = IntCode::new(codes, vec![]);

    let (map, oxygen) = get_map(computer);

    // Flood fill from oxygen.
    let mut stack = Vec::new();
    let mut flood_map = HashMap::new();
    flood_map.insert(oxygen, 0);
    stack.push((oxygen.0, oxygen.1 + 1));
    stack.push((oxygen.0, oxygen.1 - 1));
    stack.push((oxygen.0 + 1, oxygen.1));
    stack.push((oxygen.0 - 1, oxygen.1));
    let mut max_steps = 0;

    while let Some(p) = stack.pop() {
        let lowest_neighbour = *flood_map
            .get(&(p.0, p.1 + 1))
            .unwrap_or(&usize::MAX)
            .min(flood_map.get(&(p.0, p.1 - 1)).unwrap_or(&usize::MAX))
            .min(flood_map.get(&(p.0 + 1, p.1)).unwrap_or(&usize::MAX))
            .min(flood_map.get(&(p.0 - 1, p.1)).unwrap_or(&usize::MAX));
        let f = flood_map.entry(p);
        match map.get(&p).unwrap_or(&Space::Unknown) {
            Space::Unknown => todo!(),
            Space::Nothing => {
                match f {
                    std::collections::hash_map::Entry::Occupied(x) => {
                        assert!(x.get() <= &(lowest_neighbour + 1));
                    }
                    std::collections::hash_map::Entry::Vacant(x) => {
                        x.insert(lowest_neighbour + 1);
                        max_steps = max_steps.max(lowest_neighbour + 1);
                        stack.push((p.0, p.1 + 1));
                        stack.push((p.0, p.1 - 1));
                        stack.push((p.0 + 1, p.1));
                        stack.push((p.0 - 1, p.1));
                    }
                };
            }
            Space::Wall => {}
            Space::Oxygen => {}
        };
    }

    max_steps
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_star_one() {}
}
