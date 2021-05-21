use itertools::Itertools;
use std::fs;

// TODO change to getting the input values.
fn process(
    codes: &mut [isize],
    start_index: usize,
    input: &mut Vec<isize>,
) -> Option<(usize, isize)> {
    let mut index = start_index;
    let get_parameter = |program: &[isize], op, pos: usize| {
        // println!("{:?} {} {} {}", program, op, pos, (program[op] / (10*10_isize.pow(pos as u32))) % 10);
        if (program[op] / (10 * 10_isize.pow(pos as u32))) % 10 == 1 {
            program[op + pos]
        } else {
            program[program[op + pos] as usize]
        }
    };
    while index < codes.len() {
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
                let output_parameter = get_parameter(&codes, index, 1);
                // output.push(output_parameter);
                return Some((index + 2, output_parameter));
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
            99 => return None,
            _ => {
                println!("{:?} ({})", codes, index);
                panic!()
            }
        }
    }
    panic!();
    // return output;
}

fn run_feedback(program: &mut Vec<isize>, settings: &[isize]) -> isize {
    let mut programs = vec![(program.clone(), 0); settings.len()]; // (program, index)
    let mut indexes = vec![0; settings.len()];
    let mut last_output = 0;
    let mut i = 0;
    loop {
        let (ref mut program, index) = &mut programs[i];
        let mut input = vec![last_output];
        if index == 0 {
            input.insert(0, settings[i]);
        }
        match process(program, index, &mut input) {
            Some((index, output)) => {
                last_output = output;
                programs[i].1 = index;
                indexes[i] = index;
                // programs[(i + 1) %5].2 = output;
            }
            None => return last_output, //programs[(i + 1) %5].2,
        }
        // println!("{}: {:?} {}", i, indexes, last_output);
        i = (i + 1) % 5;
    }
}

fn find_highest_output(
    codes: &mut Vec<isize>,
    min: isize,
    max: isize,
) -> (isize, isize, isize, isize, isize, isize) {
    let mut highest_output = (-1, -1, -1, -1, -1, 0);
    for a in (min..=max).permutations(5) {
        let output = run_feedback(codes, &a);
        // println!("{:?}: {:?}", a, output);
        if output > highest_output.5 {
            // println!("{:?}", output);
            highest_output = (a[0], a[1], a[2], a[3], a[4], output);
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
    let (.., highest_output) = find_highest_output(&mut codes, 5, 9);
    println!("{:?}", highest_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_1() {
        let mut program = vec![1101, 100, -1, 4, 0];
        let output = process(&mut program, 0, &mut vec![1]);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_1_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let output = process(&mut program, 0, &mut vec![1]);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }

    // #[test]
    // fn test_program_2() {
    //     let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    //     let output = process(&mut program, &mut vec![5]);
    //     assert_eq!(output.last(), Some(&1));
    // }

    #[test]
    fn test_program_2_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let output = process(&mut program, 0, &mut vec![5]);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn test_find_highest_output() {
        assert_eq!(
            find_highest_output(
                &mut vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                0,
                4
            ),
            (4, 3, 2, 1, 0, 43210)
        );
    }

    #[test]
    fn test_find_highest_feedback_output() {
        let mut program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            find_highest_output(&mut program, 5, 9),
            (9, 8, 7, 6, 5, 139629729)
        );
    }
}
