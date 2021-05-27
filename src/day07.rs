use std::io::BufRead;

use itertools::Itertools;

use crate::process;

fn find_highest_output(codes: &mut Vec<isize>) -> (isize, isize, isize, isize, isize, isize) {
    let mut highest_output = (-1, -1, -1, -1, -1, 0);
    for a in (0..5).permutations(5) {
        let mut input = vec![a[0], 0];
        let (_, output, _halted) = process(&mut codes.clone(), 0, &mut input, true);
        let mut input = vec![a[1], output[0]];
        let (_, output, _halted) = process(&mut codes.clone(), 0, &mut input, true);
        let mut input = vec![a[2], output[0]];
        let (_, output, _halted) = process(&mut codes.clone(), 0, &mut input, true);
        let mut input = vec![a[3], output[0]];
        let (_, output, _halted) = process(&mut codes.clone(), 0, &mut input, true);
        let mut input = vec![a[4], output[0]];
        let (_, output, _halted) = process(&mut codes.clone(), 0, &mut input, true);
        if output[0] > highest_output.5 {
            highest_output = (a[0], a[1], a[2], a[3], a[4], output[0]);
            // println!("{:?}", highest_output);
        }
    }
    highest_output
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
    let (.., highest_output) = find_highest_output(&mut codes);
    highest_output as usize
}

// // TODO change to getting the input values.
// fn process(
//     codes: &mut [isize],
//     start_index: usize,
//     input: &mut Vec<isize>,
// ) -> Option<(usize, isize)> {
//     let mut index = start_index;
//     let get_parameter = |program: &[isize], op, pos: usize| {
//         // println!("{:?} {} {} {}", program, op, pos, (program[op] / (10*10_isize.pow(pos as u32))) % 10);
//         if (program[op] / (10 * 10_isize.pow(pos as u32))) % 10 == 1 {
//             program[op + pos]
//         } else {
//             program[program[op + pos] as usize]
//         }
//     };
//     while index < codes.len() {
//         match codes[index] % 100 {
//             1 => {
//                 let output_index = codes[index + 3] as usize;
//                 let input1 = get_parameter(&codes, index, 1);
//                 let input2 = get_parameter(&codes, index, 2);
//                 codes[output_index] = input1 + input2;
//                 index += 4;
//             }
//             2 => {
//                 let output_index = codes[index + 3] as usize;
//                 let input1 = get_parameter(&codes, index, 1);
//                 let input2 = get_parameter(&codes, index, 2);
//                 codes[output_index] = input1 * input2;
//                 index += 4;
//             }
//             3 => {
//                 let output_index = codes[index + 1] as usize;
//                 codes[output_index] = input.remove(0);
//                 index += 2
//             }
//             4 => {
//                 let output_parameter = get_parameter(&codes, index, 1);
//                 // output.push(output_parameter);
//                 return Some((index + 2, output_parameter));
//             }
//             //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//             5 => {
//                 let input1 = get_parameter(&codes, index, 1);
//                 let input2 = get_parameter(&codes, index, 2);
//                 if input1 > 0 {
//                     index = input2 as usize
//                 } else {
//                     index += 3
//                 }
//             }
//             // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//             6 => {
//                 let input1 = get_parameter(&codes, index, 1);
//                 let input2 = get_parameter(&codes, index, 2);
//                 if input1 == 0 {
//                     index = input2 as usize
//                 } else {
//                     index += 3
//                 }
//             }
//             // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//             7 => {
//                 let output_index = codes[index + 3] as usize;
//                 let input1 = get_parameter(&codes, index, 1);
//                 let input2 = get_parameter(&codes, index, 2);
//                 if input1 < input2 {
//                     codes[output_index] = 1;
//                 } else {
//                     codes[output_index] = 0;
//                 }
//                 index += 4;
//             }
//             // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//             8 => {
//                 let output_index = codes[index + 3] as usize;
//                 let input1 = get_parameter(&codes, index, 1);
//                 let input2 = get_parameter(&codes, index, 2);
//                 if input1 == input2 {
//                     codes[output_index] = 1;
//                 } else {
//                     codes[output_index] = 0;
//                 }
//                 index += 4
//             }
//             99 => return None,
//             _ => {
//                 println!("{:?} ({})", codes, index);
//                 panic!()
//             }
//         }
//     }
//     panic!();
//     // return output;
// }

fn run_feedback(program: &[isize], settings: &[isize]) -> isize {
    let mut programs = vec![(program.to_owned(), 0); settings.len()]; // (program, index)
    let mut last_output = 0;
    let mut i = 0;
    loop {
        let (ref mut program, index) = &mut programs[i];
        let mut input = vec![last_output];
        if index == &0 {
            input.insert(0, settings[i]);
        }
        let (new_index, output, halted) = process(program, *index, &mut input, true);
        println!("{:?}", output);
        if halted {
            return last_output;
        } else {
            last_output = output[0];
            programs[i].1 = new_index;
            // indexes[i] = new_index;
            // programs[(i + 1) %5].2 = output;
            //programs[(i + 1) %5].2,
        }
        // println!("{}: {:?} {}", i, indexes, last_output);
        i = (i + 1) % settings.len();
    }
}

fn find_highest_output2(
    codes: &mut Vec<isize>,
    min: isize,
    max: isize,
) -> (isize, isize, isize, isize, isize, isize) {
    let mut highest_output = (-1, -1, -1, -1, -1, 0);
    for a in (min..max).permutations(5) {
        let output = run_feedback(codes, &a);
        println!("{:?}: {:?}", a, output);
        if output > highest_output.5 {
            // println!("{:?}", output);
            highest_output = (a[0], a[1], a[2], a[3], a[4], output);
        }
    }
    highest_output
}

pub fn star_two(input: impl BufRead) -> usize {
    // !FIXME
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
    let (.., highest_output) = find_highest_output2(&mut codes, 5, 10);
    highest_output as usize
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_program_1() {
        let mut program = vec![1101, 100, -1, 4, 0];
        let _output = process(&mut program, 0, &mut vec![1], true);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_1_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let _output = process(&mut program, 0, &mut vec![1], true);
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
        let _output = process(&mut program, 0, &mut vec![5], true);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn test_find_highest_output() {
        assert_eq!(
            find_highest_output2(
                &mut vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                0,
                5
            ),
            (4, 3, 2, 1, 0, 43210)
        );
    }

    #[test]
    fn test_feedback() {
        let mut program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let settings = vec![9, 8, 7, 6, 5];
        let output = run_feedback(&mut program, &settings);
        assert_eq!(output, 139629729);
    }

    #[test]
    fn test_find_highest_feedback_output() {
        let mut program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            find_highest_output2(&mut program, 5, 10),
            (9, 8, 7, 6, 5, 139629729)
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_1() {
        let mut program = vec![1101, 100, -1, 4, 0];
        let _output = process(&mut program, 0, &mut vec![1], true);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn test_program_1_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let _output = process(&mut program, 0, &mut vec![1], true);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn test_program_2() {
        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let output = process(&mut program, 0, &mut vec![5], true);
        assert_eq!(output.1[0], 1);
    }

    #[test]
    fn test_program_2_2() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let _output = process(&mut program, 0, &mut vec![5], true);
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
