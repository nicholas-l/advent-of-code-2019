use std::{collections::VecDeque, io::BufRead};

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
    let mut computers: Vec<IntCode> = (0..50)
        .map(|i| IntCode::new(codes.clone(), vec![i]))
        .collect();
    let mut network = VecDeque::new();
    for i in 0..50 {
        let input: Vec<isize> = vec![i, -1];
        network.push_back((i as usize, input));
    }

    loop {
        while let Some((i, input)) = network.pop_front() {
            let computer = &mut computers[i];
            computer.set_input(input);
            match computer.run(3) {
                IntCodeState::Halted(_) => todo!(),
                IntCodeState::Output(output) => {
                    computer.take_output();
                    let (address, x, y) = (output[0], output[1], output[2]);
                    if address == 255 {
                        return y as usize;
                    }
                    network.push_back((address as usize, vec![x, y]));
                }
                IntCodeState::InputNeeded => {}
            }
        }
        network.extend((0..50).map(|address| (address, vec![-1])));
    }
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
    let mut computers: Vec<IntCode> = (0..50)
        .map(|i| IntCode::new(codes.clone(), vec![i]))
        .collect();
    let mut network = VecDeque::new();
    for i in 0..50 {
        let input: Vec<isize> = vec![i];
        network.push_back((i as usize, input));
    }

    let mut nat = None;
    let mut last_nat_packet: Option<(isize, isize)> = None;

    loop {
        while let Some((i, input)) = network.pop_front() {
            let computer = &mut computers[i];
            computer.set_input(input);
            // Weirdly we need to run it after getting out output to get it into an 'idle' state
            // so we loop here then break when it asks for more input.
            loop {
                match computer.run(3) {
                    IntCodeState::Halted(_) => panic!("Computer should never halt."),
                    IntCodeState::Output(output) => {
                        computer.take_output();
                        let (address, x, y) = (output[0], output[1], output[2]);
                        if address == 255 {
                            nat = Some((x, y));
                        } else {
                            network.push_back((address as usize, vec![x, y]));
                        }
                    }
                    IntCodeState::InputNeeded => break,
                }
            }
        }

        if let Some(packet) = nat {
            if last_nat_packet
                .map(|last_packet| last_packet.1 == packet.1)
                .unwrap_or(false)
            {
                return last_nat_packet.unwrap().1 as usize;
            } else {
                last_nat_packet = Some(packet);
                network.push_back((0, vec![packet.0, packet.1]));
            }
        } else {
            network.extend((0..50).map(|address| (address, vec![-1])));
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}
}
