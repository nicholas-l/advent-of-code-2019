use std::io::BufRead;

// TODO change to getting the input values.
fn process(codes: &mut [isize], input: isize) -> Vec<isize> {
    let mut index = 0;
    let mut output = vec![];
    let get_parameter = |program: &[isize], op, pos: usize| {
        // println!("{:?} {} {} {}", program, op, pos, (program[op] / (10*10_isize.pow(pos as u32))) % 10);
        if (program[op] / (10 * 10_isize.pow(pos as u32))) % 10 == 1 {
            program[op + pos]
        } else {
            program[program[op + pos] as usize]
        }
    };
    while codes[index] != 99 {
        match codes[index] % 100 {
            1 => {
                let output_index = codes[index + 3] as usize;
                let input1 = get_parameter(&codes, index, 1);
                let input2 = get_parameter(&codes, index, 2);
                codes[output_index] = input1 + input2;
                index += 4;
            }
            2 => {
                let output_index = codes[index + 3] as usize;
                let input1 = get_parameter(&codes, index, 1);
                let input2 = get_parameter(&codes, index, 2);
                codes[output_index] = input1 * input2;
                index += 4;
            }
            3 => {
                let output_index = codes[index + 1] as usize;
                codes[output_index] = input;
                index += 2
            }
            4 => {
                let input1 = get_parameter(&codes, index, 1);
                output.push(input1);
                index += 2
            }
            //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            5 => {
                let input1 = get_parameter(&codes, index, 1);
                let input2 = get_parameter(&codes, index, 2);
                if input1 > 0 {
                    index = input2 as usize
                } else {
                    index += 3
                }
            }
            // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            6 => {
                let input1 = get_parameter(&codes, index, 1);
                let input2 = get_parameter(&codes, index, 2);
                if input1 == 0 {
                    index = input2 as usize
                } else {
                    index += 3
                }
            }
            // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            7 => {
                let output_index = codes[index + 3] as usize;
                let input1 = get_parameter(&codes, index, 1);
                let input2 = get_parameter(&codes, index, 2);
                if input1 < input2 {
                    codes[output_index] = 1;
                } else {
                    codes[output_index] = 0;
                }
                index += 4;
            }
            // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            8 => {
                let output_index = codes[index + 3] as usize;
                let input1 = get_parameter(&codes, index, 1);
                let input2 = get_parameter(&codes, index, 2);
                if input1 == input2 {
                    codes[output_index] = 1;
                } else {
                    codes[output_index] = 0;
                }
                index += 4
            }
            _ => {
                println!("{:?} ({})", codes, index);
                panic!()
            }
        }
    }
    output
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
    let output = process(&mut codes, 1);
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
    let output = process(&mut codes, 5);
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
        let mut program = vec![1101, 100, -1, 4, 0];
        let _output = process(&mut program, 1);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let _output = process(&mut program, 1);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }
}
