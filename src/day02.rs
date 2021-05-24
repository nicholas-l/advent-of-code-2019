use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> usize {
    let mut codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    let mut input = Vec::new();
    let (_index, _output, _halted) = super::process(&mut codes, 0, &mut input, true);
    codes[0] as usize
}

pub fn star_two(input: impl BufRead) -> usize {
    let codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    // println!("{:?}", codes);
    for noun in 1..100 {
        for verb in 1..100 {
            let mut program = codes.clone();
            let mut input = Vec::new();
            program[1] = noun;
            program[2] = verb;
            let (_index, _output, _halted) = super::process(&mut program, 0, &mut input, true);

            if program[0] == 19690720 {
                println!("{} {}", noun, verb);
                return (100 * noun + verb) as usize;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_star_one() {
        {
            let input = b"1,9,10,3,2,3,11,0,99,30,40,50";
            assert_eq!(star_one(Cursor::new(input)), 3500);
        }

        {
            let input = b"1,0,0,0,99";
            assert_eq!(star_one(Cursor::new(input)), 2);
        }

        {
            let input = b"1,1,1,4,99,5,6,0,99";
            assert_eq!(star_one(Cursor::new(input)), 30);
        }
    }

    // #[test]
    // fn test_star_two() {
    //     let input = b"1,9,10,3,2,3,11,0,99,30,40,50";
    //     assert_eq!(star_one(Cursor::new(input)), 67384529);
    // }
}
