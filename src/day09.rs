use std::io::BufRead;

use crate::IntCode;

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
    let input = vec![1];
    let mut computer = IntCode::new(codes, input);
    computer.run(0);
    let output = computer.take_output();
    dbg!(&output);
    output[0] as usize
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
    let input = vec![2];
    let mut computer = IntCode::new(codes, input);
    computer.run(0);
    let output = computer.take_output();
    dbg!(&output);
    output[0] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_base() {
        let program = vec![109, 19, 204, -34];
        let input = vec![];
        let mut computer = IntCode::new(program, input);
        computer.run(1);
        let output = computer.take_output();
        assert_eq!(output[0], 0);
    }

    #[test]
    fn test_star_one() {
        {
            let program = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            let input = Vec::new();
            let mut computer = IntCode::new(program, input);
            computer.run(0);
            let output = computer.take_output();
            let expected_output = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            assert_eq!(output.len(), expected_output.len());
            assert_eq!(output, expected_output);
        }
        {
            let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
            let input = Vec::new();
            let mut computer = IntCode::new(program, input);
            computer.run(0);
            let output = computer.take_output();
            assert!(output[0] > (10_isize.pow(15) - 1));
            assert!(output[0] < 10_isize.pow(16));
        }
        {
            let program = vec![104, 1125899906842624, 99];
            let input = Vec::new();
            let mut computer = IntCode::new(program, input);
            computer.run(0);
            let output = computer.take_output();
            assert_eq!(output, vec![1125899906842624]);
        }
    }
}
