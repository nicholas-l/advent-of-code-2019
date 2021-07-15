use std::{
    io::{stdin, BufRead},
    iter::once,
};

use crate::{IntCode, IntCodeState};

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

    let mut commands = vec![
        "east",                     // Hull breach
        "take antenna",             // Holodeck
        "north",                    // Holodeck
        "north",                    // Hallway
        "take asterisk",            // Gift Wrapping Center
        "south",                    // Gift Wrapping Center
        "west",                     // Hallway
        "south",                    // Science Lab
        "take hologram",            // Engineering
        "north",                    // Engineering
        "west",                     // Science Lab
        "take astronaut ice cream", // Corridor
        "east",                     // Corridor
        "east",                     // Science Lab
        "south",                    // Hallway
        "east",                     // Holodeck
        "take ornament",            // Crew Quarters
        "north",                    // Crew Quarters
        "west",                     // Kitchen
        "take fixed point",         // Observatory
        "east",                     // Observatory
        "south",                    // Kitchen
        "west",                     // Crew Quarters
        "west",                     // Holodeck
        "south",                    // Hull Breach
        "south",                    // Sick Bay
        "south",                    // Storage
        "take dark matter",         // Arcade
        "north",                    // Arcade
        "west",                     // Storage
        "north",                    // Stables
        "take monolith",            // Navgiation
        "north",                    // Navigation
        "north",                    // Passages
        "east",                     // Security Checkpoint
        "drop monolith",
        "drop antenna",
        "drop hologram",
        "drop dark matter",
        "east",
    ]
    .into_iter()
    .rev()
    .collect::<Vec<_>>();
    // println!("{}", vec![10, 10, 10, 61, 61, 32, 80, 114, 101, 115, 115, 117, 114, 101, 45, 83, 101, 110, 115, 105, 116, 105, 118, 101, 32, 70, 108, 111, 111, 114, 32, 61, 61, 10, 65, 110, 97, 108, 121, 122, 105, 110, 103, 46, 46, 46, 10, 10, 68, 111, 111, 114, 115, 32, 104, 101, 114, 101, 32, 116, 108, 121, 46, 10, 34, 79, 104, 44, 32, 104, 101, 108, 108, 111, 33, 32, 89, 111, 117, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 97, 98, 108, 101, 32, 116, 111, 32, 103, 101, 116, 32, 105, 110, 32, 98, 121, 32, 116, 121, 112, 105, 110, 103, 32, 49, 51, 52, 50, 50, 55, 52, 53, 54, 32, 111, 110, 32, 116, 104, 101, 32, 107, 101, 121, 112, 97, 100, 32, 97, 116, 32, 116, 104, 101, 32, 109, 97, 105, 110, 32, 97, 105, 114, 108, 111, 99, 107, 46, 34, 10].into_iter().map(|x| x as u8 as char).collect::<String>());
    let mut computer = IntCode::new(codes, vec![]);
    loop {
        match computer.run(0) {
            IntCodeState::Halted(output) => {
                let output_string = output.iter().map(|&x| x as u8 as char).collect::<String>();
                println!("{}", output_string);
                let pin: String = output_string.matches(char::is_numeric).collect();
                let number = pin.parse::<usize>().unwrap();
                return number;
            }
            IntCodeState::Output(_) => panic!("We should never reach here"),
            IntCodeState::InputNeeded => {
                // println!("Input needed");
                let output = computer
                    .take_output()
                    .iter()
                    .map(|&c| c as u8 as char)
                    .collect::<String>();
                println!("{}", output);
                if let Some(c) = commands.pop() {
                    let new_input = c
                        .trim()
                        .chars()
                        .map(|c| c as isize)
                        .chain(once(10))
                        .collect();
                    computer.set_input(new_input);
                } else {
                    let mut input = String::new();
                    match stdin().read_line(&mut input) {
                        Ok(_) => {
                            // let selected = options[input.trim().parse::<usize>().unwrap()];
                            let new_input = input
                                .trim()
                                .chars()
                                .map(|c| c as isize)
                                .chain(once(10))
                                .collect();
                            // println!("{:?}", new_input);
                            computer.set_input(new_input);
                        }
                        Err(_) => todo!(),
                    }
                }
            }
        }
    }
}

pub fn star_two(_input: impl BufRead) -> usize {
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}
}
