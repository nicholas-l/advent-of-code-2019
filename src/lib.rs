use std::convert::TryInto;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

fn write(program: &mut Vec<isize>, index: usize, data: isize) {
    if index >= program.len() {
        program.resize(index + 1, 0)
    }
    program[index] = data;
}

fn read(program: &[isize], index: usize) -> isize {
    *program.get(index).unwrap_or(&0)
}

// TODO change to getting the input values.
pub fn process(
    codes: &mut Vec<isize>,
    start_index: usize,
    input: &mut Vec<isize>,
    stop_if_output: bool,
) -> (usize, Vec<isize>, bool) {
    let mut index = start_index;
    let mut output = vec![];
    let mut found_99 = false;
    let mut relative_base: isize = 0;

    let get_index = |program: &[isize], op, pos: usize, relative_base| {
        // println!("{:?} {} {} {}", program, op, pos, (program[op] / (10*10_isize.pow(pos as u32))) % 10);
        let mode = (program[op] / 10_isize.pow((pos + 1).try_into().unwrap())) % 10;
        match mode {
            0 => program[op + pos] as usize,
            1 => op + pos,
            2 => (relative_base + program[op + pos]) as usize,
            _ => panic!("Mode {} not supported", mode),
        }
    };
    let get_parameter = |program: &[isize], op, pos: usize, relative_base| {
        // println!("{:?} {} {} {}", program, op, pos, (program[op] / (10*10_isize.pow(pos as u32))) % 10);
        read(program, get_index(program, op, pos, relative_base))
    };
    while index < codes.len() {
        match codes[index] % 100 {
            1 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                let input2 = get_parameter(codes, index, 2, relative_base);
                let output_index = get_index(codes, index, 3, relative_base);
                // assert!(input1 >= 0);
                // assert!(input2 >= 0);
                write(codes, output_index, input1 + input2);
                index += 4;
            }
            2 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                let input2 = get_parameter(codes, index, 2, relative_base);
                let output_index = get_index(codes, index, 3, relative_base);
                write(codes, output_index, input1 * input2);
                index += 4;
            }
            3 => {
                let output_index = get_index(codes, index, 1, relative_base);
                codes[output_index] = input.remove(0);
                index += 2
            }
            4 => {
                let output_parameter = get_parameter(codes, index, 1, relative_base);
                output.push(output_parameter);
                index += 2;
                if stop_if_output {
                    break;
                }
            }
            //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            5 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                let input2 = get_parameter(codes, index, 2, relative_base);
                if input1 > 0 {
                    index = input2 as usize
                } else {
                    index += 3
                }
            }
            // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            6 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                let input2 = get_parameter(codes, index, 2, relative_base);
                if input1 == 0 {
                    index = input2 as usize
                } else {
                    index += 3
                }
            }
            // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            7 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                let input2 = get_parameter(codes, index, 2, relative_base);
                let output_index = get_index(codes, index, 3, relative_base);
                if input1 < input2 {
                    codes[output_index] = 1;
                } else {
                    codes[output_index] = 0;
                }
                index += 4;
            }
            // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            8 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                let input2 = get_parameter(codes, index, 2, relative_base);
                let output_index = get_index(codes, index, 3, relative_base);

                if input1 == input2 {
                    write(codes, output_index, 1);
                } else {
                    write(codes, output_index, 0);
                }
                index += 4
            }
            9 => {
                let input1 = get_parameter(codes, index, 1, relative_base);
                relative_base += input1;
                index += 2
            }
            99 => {
                found_99 = true;
                break; // return None,
            }
            x => {
                println!("{:?} [{}]: {}", codes, index, x);
                panic!()
            }
        }
    }
    (index, output, found_99)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::{
        fs,
        io::BufRead,
        path::{Path, PathBuf},
    };

    fn get_data(filepath: &PathBuf) -> Box<dyn BufRead> {
        let f = fs::File::open(filepath).unwrap();
        let input = BufReader::new(f);
        Box::new(input)
    }

    #[test]
    fn day01_complete() {
        let filepath = Path::new("data").join("day01.txt");
        assert_eq!(day01::star_one(get_data(&filepath)), 3512133);

        assert_eq!(day01::star_two(get_data(&filepath)), 5265294);
    }

    #[test]
    fn day02_complete() {
        let filepath = Path::new("data").join("day02.txt");
        assert_eq!(day02::star_one(get_data(&filepath)), 3516593);

        assert_eq!(day02::star_two(get_data(&filepath)), 7749);
    }

    #[test]
    fn day03_complete() {
        let filepath = Path::new("data").join("day03.txt");
        assert_eq!(day03::star_one(get_data(&filepath)), 1211);

        assert_eq!(day03::star_two(get_data(&filepath)), 101386);
    }

    #[test]
    fn day04_complete() {
        let filepath = Path::new("data").join("day04.txt");
        assert_eq!(day04::star_one(get_data(&filepath)), 921);

        assert_eq!(day04::star_two(get_data(&filepath)), 603);
    }

    #[test]
    fn day05_complete() {
        let filepath = Path::new("data").join("day05.txt");
        assert_eq!(day05::star_one(get_data(&filepath)), 9961446);

        assert_eq!(day05::star_two(get_data(&filepath)), 742621);
    }

    #[test]
    fn day06_complete() {
        let filepath = Path::new("data").join("day06.txt");
        assert_eq!(day06::star_one(get_data(&filepath)), 273985);

        assert_eq!(day06::star_two(get_data(&filepath)), 460);
    }

    #[test]
    fn day07_complete() {
        let filepath = Path::new("data").join("day07.txt");
        assert_eq!(day07::star_one(get_data(&filepath)), 46014);
        println!("Completed 1");
        assert_eq!(day07::star_two(get_data(&filepath)), 19581200);
    }

    #[test]
    fn day08_complete() {
        let filepath = Path::new("data").join("day08.txt");
        assert_eq!(day08::star_one(get_data(&filepath)), 1935);

        assert_eq!(day08::star_two(get_data(&filepath)), "CFLUL");
    }
    #[test]
    fn day09_complete() {
        let filepath = Path::new("data").join("day09.txt");
        assert_eq!(day09::star_one(get_data(&filepath)), 2171728567);

        assert_eq!(day09::star_two(get_data(&filepath)), 49815);
    }

    #[test]
    fn day10_complete() {
        let filepath = Path::new("data").join("day10.txt");
        assert_eq!(day10::star_one(get_data(&filepath)), 517);

        assert_eq!(day10::star_two(get_data(&filepath)), 49815);
    }
}
