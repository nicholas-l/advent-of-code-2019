use std::io::BufRead;

use itertools::Itertools;

use crate::{IntCode, IntCodeState};

fn find_highest_output(codes: &mut [isize]) -> (isize, isize, isize, isize, isize, isize) {
    let mut highest_output = (-1, -1, -1, -1, -1, 0);
    for a in (0..5).permutations(5) {
        let mut last_output = 0;
        for &x in &a {
            let input = vec![x, last_output];
            let mut computer = IntCode::new(codes.to_vec(), input);
            computer.run(1);
            let output = computer.take_output();
            last_output = output[0];
        }

        if last_output > highest_output.5 {
            highest_output = (a[0], a[1], a[2], a[3], a[4], last_output);
            // println!("{:?}", highest_output);
        }
    }
    highest_output
}

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
    let (.., highest_output) = find_highest_output(&mut codes);
    highest_output as usize
}

fn run_feedback(program: Vec<isize>, settings: &[isize]) -> isize {
    let mut programs = vec![(IntCode::new(program, vec![]), false); settings.len()]; // (program, index)
    let mut last_output = 0;
    let mut i = 0;
    loop {
        let (ref mut computer, initialised) = &mut programs[i];
        let mut input = vec![last_output];
        if !(*initialised) {
            input.insert(0, settings[i]);
            *initialised = true;
        }
        computer.set_input(input);
        match computer.run(1) {
            IntCodeState::Halted(_) => {
                return last_output;
            }
            IntCodeState::Output(output) => {
                computer.take_output(); // Clear output in computer since we have cloned it.
                last_output = output[0];
            }
            _ => panic!(),
        }
        // println!("{}: {:?} {}", i, indexes, last_output);
        i = (i + 1) % settings.len();
    }
}

fn find_highest_output2(
    codes: Vec<isize>,
    min: isize,
    max: isize,
) -> (isize, isize, isize, isize, isize, isize) {
    let mut highest_output = (-1, -1, -1, -1, -1, 0);
    for a in (min..max).permutations(5) {
        let output = run_feedback(codes.clone(), &a);
        // println!("{:?}: {:?}", a, output);
        if output > highest_output.5 {
            // println!("{:?}", output);
            highest_output = (a[0], a[1], a[2], a[3], a[4], output);
        }
    }
    highest_output
}

pub fn star_two(input: impl BufRead) -> usize {
    // !FIXME
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
    let (.., highest_output) = find_highest_output2(codes, 5, 10);
    highest_output as usize
}

#[cfg(test)]
mod tests2 {
    use crate::IntCode;

    use super::*;

    #[test]
    fn test_program_1() {
        let program = vec![1101, 100, -1, 4, 0];
        let mut computer = IntCode::new(program, vec![1]);
        computer.run(1);
        assert_eq!(computer.get_program(), vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_program_1_2() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = IntCode::new(program, vec![1]);
        computer.run(1);
        assert_eq!(computer.get_program(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_program_2_2() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = IntCode::new(program, vec![5]);
        computer.run(1);
        assert_eq!(computer.get_program(), vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn test_find_highest_output() {
        assert_eq!(
            find_highest_output2(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                0,
                5
            ),
            (4, 3, 2, 1, 0, 43210)
        );
    }

    #[test]
    fn test_feedback() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let settings = vec![9, 8, 7, 6, 5];
        let output = run_feedback(program, &settings);
        assert_eq!(output, 139629729);
    }

    #[test]
    fn test_find_highest_feedback_output() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            find_highest_output2(program, 5, 10),
            (9, 8, 7, 6, 5, 139629729)
        );
    }
}
#[cfg(test)]
mod tests {
    use crate::IntCode;

    use super::*;

    #[test]
    fn test_program_1() {
        let program = vec![1101, 100, -1, 4, 0];
        let mut computer = IntCode::new(program, vec![1]);
        computer.run(1);
        assert_eq!(computer.get_program(), vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_1_2() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = IntCode::new(program, vec![1]);
        computer.run(1);
        assert_eq!(computer.get_program(), vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn test_program_2() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut computer = IntCode::new(program, vec![5]);
        computer.run(1);
        let output = computer.take_output();
        assert_eq!(output[0], 1);
    }

    #[test]
    fn test_program_2_2() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = IntCode::new(program, vec![5]);
        computer.run(1);
        assert_eq!(computer.get_program(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_find_highest_output() {
        assert_eq!(
            find_highest_output(&mut vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            (4, 3, 2, 1, 0, 43210)
        );
    }
}
