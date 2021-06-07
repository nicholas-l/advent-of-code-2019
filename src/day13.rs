use std::{collections::HashMap, io::BufRead};

use crate::IntCode;

pub fn star_one(input: impl BufRead) -> usize {
    let mut codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            // println!("{}", &v);
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    let mut computer = IntCode::new(&mut codes, 0, vec![]);
    let mut halted = false;
    let mut screen = HashMap::new();
    while !halted {
        let data = computer.run(3);
        halted = data.1;
        if halted {
            break;
        }
        let output = computer.take_output();
        screen.insert((output[0], output[1]), output[2]);
    }
    screen.values().filter(|&&x| x == 2).count()
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            // println!("{}", &v);
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    // Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
    codes[0] = 2;
    let mut computer = IntCode::new(&mut codes, 0, vec![]);
    let mut halted = false;
    let mut screen = HashMap::new();
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    while !halted {
        // Move paddle to be under the ball.
        let input = vec![ball_x - paddle_x];
        computer.set_input(input);
        let data = computer.run(3);
        halted = data.1;
        if halted {
            break;
        }
        let output = computer.take_output();
        match (output[0], output[1], output[2]) {
            (-1, 0, s) => {
                score = s;
            }
            (x, y, v) => {
                screen.insert((x, y), v);
                /*
                0 is an empty tile. No game object appears in this tile.
                1 is a wall tile. Walls are indestructible barriers.
                2 is a block tile. Blocks can be broken by the ball.
                3 is a horizontal paddle tile. The paddle is indestructible.
                4 is a ball tile. The ball moves diagonally and bounces off objects
                */
                match v {
                    3 => {
                        paddle_x = x;
                    }
                    4 => {
                        ball_x = x;
                    }
                    _ => {}
                }
            }
        }
    }
    score as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_star_one() {}
}
