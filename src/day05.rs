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
    use crate::IntCode;

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}

    #[test]
    fn test_program() {
        let input = vec![1];
        let program = vec![1101, 100, -1, 4, 0];
        let mut computer = IntCode::new(program, 0, input);
        computer.run(1);
        assert_eq!(computer.get_program(), &vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_program_2() {
        let input = vec![1];
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = IntCode::new(program, 0, input);
        computer.run(0);
        assert_eq!(computer.get_program(), &vec![1002, 4, 3, 4, 99]);
    }
}
