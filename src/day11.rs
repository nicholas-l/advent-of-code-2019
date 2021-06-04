use std::{collections::HashMap, fmt::Display, io::BufRead};

use crate::IntCode;

#[derive(Debug, Copy, Clone)]
enum Colour {
    Black = 0,
    White = 1,
}

impl From<isize> for Colour {
    fn from(i: isize) -> Self {
        match i {
            0 => Colour::Black,
            1 => Colour::White,
            x => panic!("Unable to convert {} to colour", x),
        }
    }
}

impl Default for Colour {
    fn default() -> Self {
        Colour::Black
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Black => write!(f, "."),
            Colour::White => write!(f, "#"),
        }
    }
}

fn paint(mut codes: Vec<isize>, start_panel: Colour) -> HashMap<(isize, isize), Colour> {
    let mut location = (0, 0);
    let mut direction = 0_isize;
    let mut panel = HashMap::new();
    panel.insert(location, start_panel);

    let input = vec![*panel.get(&location).unwrap_or(&Colour::default()) as isize];
    let mut computer = IntCode::new(&mut codes, 0, input);
    let mut halted = false;
    while !halted {
        let input = vec![*panel.get(&location).unwrap_or(&Colour::default()) as isize];
        computer.set_input(input);
        let data = computer.run(true);
        halted = data.1;
        if halted {
            break;
        }
        panel.insert(location, Colour::from(computer.take_output()[0]));
        let input = vec![*panel.get(&location).unwrap_or(&Colour::default()) as isize];
        computer.set_input(input);
        // Direction
        let data = computer.run(true);
        direction = match computer.take_output()[0] {
            0 => (direction + 90).rem_euclid(360),
            1 => (direction - 90).rem_euclid(360),
            x => panic!("Bad direction: {}", x),
        };
        location = match direction {
            0 => (location.0, location.1 + 1),   // Up
            90 => (location.0 + 1, location.1),  // Left
            180 => (location.0, location.1 - 1), // Down
            270 => (location.0 - 1, location.1), // Right
            x => panic!("Invalid direction: {}", x),
        };
        halted = data.1;
    }
    panel
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
    let panel = paint(codes, Colour::Black);
    panel.len()
}

pub fn star_two(input: impl BufRead) -> String {
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

    let panel = paint(codes, Colour::White);
    let max_x = panel.keys().max_by_key(|k| k.0).unwrap().0;
    let max_y = panel.keys().max_by_key(|k| k.1).unwrap().1;

    let min_x = panel.keys().min_by_key(|k| k.0).unwrap().0;
    let min_y = panel.keys().min_by_key(|k| k.1).unwrap().1;

    (min_y..=max_y)
        .rev()
        .map(|y| {
            (min_x..=max_x)
                .rev()
                .map(|x| format!("{}", panel.get(&(x, y)).unwrap_or(&Colour::Black)))
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
