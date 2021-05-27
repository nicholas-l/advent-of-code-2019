use std::io::BufRead;

use super::process;

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
    let mut input = vec![1];
    let (_index, output, _halted) = process(&mut codes, 0, &mut input, false);
    dbg!(&output);
    output[0] as usize
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
    let mut input = vec![2];
    let (_index, output, _halted) = process(&mut codes, 0, &mut input, false);
    dbg!(&output);
    output[0] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_base() {
        let mut program = vec![109, 19, 204, -34];
        let mut input = vec![];
        let (_, output, _) = process(&mut program, 0, &mut input, true);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn test_star_one() {
        {
            let mut program = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            let mut input = Vec::new();
            let (_index, output, _halted) = process(&mut program, 0, &mut input, false);
            let expected_output = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            assert_eq!(output.len(), expected_output.len());
            assert_eq!(output, expected_output);
        }
        {
            let mut program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
            let mut input = Vec::new();
            let (_index, output, _halted) = process(&mut program, 0, &mut input, false);
            assert!(output[0] > (10_isize.pow(15) - 1));
            assert!(output[0] < 10_isize.pow(16));
        }
        {
            let mut program = vec![104, 1125899906842624, 99];
            let mut input = Vec::new();
            let (_index, output, _halted) = process(&mut program, 0, &mut input, false);
            assert_eq!(output, vec![1125899906842624]);
        }
    }
}
