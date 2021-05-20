use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> usize {
    let mut codes: Vec<usize> = input
        .split(b',')
        .map(|v| {
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect();
    let mut index = 0;
    while codes[index] != 99 {
        let input1_index = codes[index + 1];
        let input2_index = codes[index + 2];
        let output_index = codes[index + 3];
        match codes[index] {
            1 => {
                codes[output_index] = codes[input1_index] + codes[input2_index];
            }
            2 => {
                codes[output_index] = codes[input1_index] * codes[input2_index];
            }
            _ => {
                todo!()
            }
        }
        index += 4;
    }
    codes[0]
}

fn process(codes: &mut Vec<usize>) -> usize {
    let mut index = 0;
    while codes[index] != 99 {
        let output_index = codes[index + 3];
        match codes[index] {
            1 => codes[output_index] = codes[codes[index + 1]] + codes[codes[index + 2]],
            2 => codes[output_index] = codes[codes[index + 1]] * codes[codes[index + 2]],
            99 => break,
            _ => panic!(),
        }
        index += 4;
    }
    codes[0]
}

pub fn star_two(input: impl BufRead) -> usize {
    let codes: Vec<usize> = input
        .split(b',')
        .map(|v| {
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect();
    // println!("{:?}", codes);
    for noun in 1..100 {
        for verb in 1..100 {
            let mut code = codes.clone();
            code[1] = noun;
            code[2] = verb;
            let output = process(&mut code);

            if output == 19690720 {
                println!("{} {}", noun, verb);
                return 100 * noun + verb;
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
