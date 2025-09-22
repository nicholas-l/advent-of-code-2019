use rayon::prelude::*;
use std::{io::BufRead, iter::repeat_n};

fn run(codes: Vec<isize>, loops: usize) -> Vec<isize> {
    let l = codes.len();
    (0..loops).fold(codes, |phase, _i| {
        (1..=l)
            .into_par_iter()
            .map(|d| {
                let multiplier = repeat_n(0, d)
                    .chain(repeat_n(1, d))
                    .chain(repeat_n(0, d))
                    .chain(repeat_n(-1, d))
                    .cycle()
                    .skip(1);
                phase
                    .iter()
                    .zip(multiplier)
                    .filter(|x| x.1 != 0)
                    .map(|(a, b)| a * b)
                    // .inspect(|d| println!("{}", d))
                    .sum::<isize>()
                    .abs()
                    % 10
            })
            .collect()
    })
}

fn convert_number(output: &[isize]) -> usize {
    output.iter().fold(0, |total, &x| total * 10 + x) as usize
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _input_file = input.read_to_string(&mut buf);
    let codes: Vec<isize> = buf
        .chars()
        .map(|b| b.to_digit(10).unwrap() as isize)
        .collect();

    let output = run(codes, 100);
    println!("{:?}", &output[..8]);
    convert_number(&output[..8])
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _input_file = input.read_to_string(&mut buf);
    let codes: Vec<isize> = buf
        .chars()
        .map(|b| b.to_digit(10).unwrap() as isize)
        .collect();
    let new_length = 10000 * codes.len();
    let input_phase: Vec<isize> = codes.iter().cycle().take(new_length).copied().collect();

    let offset = convert_number(&input_phase[..7]);
    let mut data = input_phase[offset..].to_vec();
    for _i in 0..100 {
        data = data
            .iter()
            .rev()
            .scan(0, |state, x| {
                *state += x;
                Some((*state) % 10)
            })
            .collect();
        data.reverse();
    }
    convert_number(&data[..8])
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            run(vec![1, 2, 3, 4, 5, 6, 7, 8], 1)[..],
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            run(vec![1, 2, 3, 4, 5, 6, 7, 8], 2)[..],
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );
        assert_eq!(
            run(vec![1, 2, 3, 4, 5, 6, 7, 8], 3)[..],
            vec![0, 3, 4, 1, 5, 5, 1, 8]
        );
        assert_eq!(
            run(vec![1, 2, 3, 4, 5, 6, 7, 8], 4)[..],
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        );
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(b"80871224585914546619083218645595")),
            24176176
        );
        assert_eq!(
            star_one(Cursor::new("19617804207202209144916044189917")),
            73745418
        );
        assert_eq!(
            star_one(Cursor::new("69317163492948606335995924319873")),
            52432133
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(b"03036732577212944063491565474664")),
            84462026
        );
        assert_eq!(
            star_two(Cursor::new("02935109699940807407585447034323")),
            78725270
        );
        assert_eq!(
            star_two(Cursor::new("03081770884921959731165446850517")),
            53553731
        );
    }
}
