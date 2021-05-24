use super::process;
use std::io::BufRead;

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
    output.into_iter().find(|&x| x != 0).unwrap() as usize
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
    let mut input = vec![5];
    let (_index, output, _halted) = process(&mut codes, 0, &mut input, false);
    output.into_iter().find(|&x| x != 0).unwrap() as usize
}

#[cfg(test)]
mod tests {
    // use std::io::Cursor;

    use super::*;

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}

    #[test]
    fn test_program() {
        let mut input = vec![1];
        let mut program = vec![1101, 100, -1, 4, 0];
        let _output = process(&mut program, 0, &mut input, true);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_2() {
        let mut input = vec![1];
        let mut program = vec![1002, 4, 3, 4, 33];
        let _output = process(&mut program, 0, &mut input, false);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }
}
