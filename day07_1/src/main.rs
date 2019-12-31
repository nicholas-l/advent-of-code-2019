use itertools::Itertools;
use itertools::Permutations;
use std::fs;

// TODO change to getting the input values.
fn process(codes: &mut [isize], input: &mut Vec<isize>) -> Vec<isize> {
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
                codes[output_index] = input.remove(0);
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
    return output;
}

fn find_highest_output(codes: &mut Vec<isize>) -> (isize, isize, isize, isize, isize, isize) {
    let mut highest_output = (-1, -1, -1, -1, -1, 0);
    for a in (0..5).permutations(5) {
        let mut input = vec![a[0], 0];
        let output = process(&mut codes.clone(), &mut input);
        let mut input = vec![a[1], output[0]];
        let output = process(&mut codes.clone(), &mut input);
        let mut input = vec![a[2], output[0]];
        let output = process(&mut codes.clone(), &mut input);
        let mut input = vec![a[3], output[0]];
        let output = process(&mut codes.clone(), &mut input);
        let mut input = vec![a[4], output[0]];
        let output = process(&mut codes.clone(), &mut input);
        if output[0] > highest_output.5 {
            highest_output = (a[0], a[1], a[2], a[3], a[4], output[0]);
            println!("{:?}", highest_output);
        }
    }
    return highest_output;
}

fn main() {
    let mut codes: Vec<isize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(",")
        .map(|v| {
            // println!("{}", &v);
            v.parse::<isize>().unwrap()
        })
        .collect();
    let (highest_output, ..) = find_highest_output(&mut codes);
    println!("{:?}", highest_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_1() {
        let mut program = vec![1101, 100, -1, 4, 0];
        let output = process(&mut program, &mut vec![1]);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_1_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let output = process(&mut program, &mut vec![1]);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn test_program_2() {
        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let output = process(&mut program, &mut vec![5]);
        assert_eq!(output.last(), Some(&1));
    }

    #[test]
    fn test_program_2_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let output = process(&mut program, &mut vec![5]);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
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
