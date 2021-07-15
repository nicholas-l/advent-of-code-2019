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
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

enum IntCodeState {
    Halted(Vec<isize>),
    Output(Vec<isize>),
    InputNeeded,
}

#[derive(Debug, Clone)]
struct IntCode {
    program: Vec<isize>,
    input: Vec<isize>,
    index: usize,
    output: Vec<isize>,
    // halted: bool,
    relative_base: isize,
}

impl IntCode {
    fn new(program: Vec<isize>, input: Vec<isize>) -> IntCode {
        IntCode {
            program,
            input,
            index: 0,
            output: Vec::new(),
            // halted: false,
            relative_base: 0,
        }
    }

    // fn input(&self) {
    //     self.input
    // }

    fn set_input(&mut self, input: Vec<isize>) {
        self.input = input;
    }
    fn len(&self) -> usize {
        self.program.len()
    }

    fn read(&self, index: usize) -> isize {
        *self.program.get(index).unwrap_or(&0)
    }

    #[allow(dead_code)]
    fn get_program(&self) -> &[isize] {
        &self.program
    }

    fn write(&mut self, index: usize, data: isize) {
        if index >= self.program.len() {
            self.program.resize(index + 1, 0)
        }
        self.program[index] = data;
    }

    fn get_index(&self, op: usize, pos: usize) -> usize {
        let mode = (self.program[op] / 10_isize.pow((pos + 1).try_into().unwrap())) % 10;
        match mode {
            0 => self.program[op + pos] as usize,
            1 => op + pos,
            2 => (self.relative_base + self.program[op + pos]) as usize,
            _ => panic!("Mode {} not supported", mode),
        }
    }
    fn take_output(&mut self) -> Vec<isize> {
        self.output.drain(..).collect()
    }
    fn get_parameter(&self, op: usize, pos: usize) -> isize {
        self.read(self.get_index(op, pos))
    }
    fn run(&mut self, output_max: usize) -> IntCodeState {
        while self.index < self.len() {
            match self.program[self.index] % 100 {
                1 => {
                    let input1 = self.get_parameter(self.index, 1);
                    let input2 = self.get_parameter(self.index, 2);
                    let output_index = self.get_index(self.index, 3);
                    // assert!(input1 >= 0);
                    // assert!(input2 >= 0);
                    self.write(output_index, input1 + input2);
                    self.index += 4;
                }
                2 => {
                    let input1 = self.get_parameter(self.index, 1);
                    let input2 = self.get_parameter(self.index, 2);
                    let output_index = self.get_index(self.index, 3);
                    self.write(output_index, input1 * input2);
                    self.index += 4;
                }
                3 => {
                    let output_index = self.get_index(self.index, 1);
                    if self.input.is_empty() {
                        return IntCodeState::InputNeeded;
                    }
                    let data = self.input.remove(0);
                    self.write(output_index, data);
                    self.index += 2
                }
                4 => {
                    let output_parameter = self.get_parameter(self.index, 1);
                    self.output.push(output_parameter);
                    self.index += 2;
                    if output_max > 0 && self.output.len() >= output_max {
                        return IntCodeState::Output(self.output.clone());
                    }
                }
                //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                5 => {
                    let input1 = self.get_parameter(self.index, 1);
                    let input2 = self.get_parameter(self.index, 2);
                    if input1 > 0 {
                        self.index = input2 as usize
                    } else {
                        self.index += 3
                    }
                }
                // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                6 => {
                    let input1 = self.get_parameter(self.index, 1);
                    let input2 = self.get_parameter(self.index, 2);
                    if input1 == 0 {
                        self.index = input2 as usize
                    } else {
                        self.index += 3
                    }
                }
                // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                7 => {
                    let input1 = self.get_parameter(self.index, 1);
                    let input2 = self.get_parameter(self.index, 2);
                    let output_index = self.get_index(self.index, 3);
                    if input1 < input2 {
                        self.write(output_index, 1);
                    } else {
                        self.write(output_index, 0);
                    }
                    self.index += 4;
                }
                // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                8 => {
                    let input1 = self.get_parameter(self.index, 1);
                    let input2 = self.get_parameter(self.index, 2);
                    let output_index = self.get_index(self.index, 3);

                    if input1 == input2 {
                        self.write(output_index, 1);
                    } else {
                        self.write(output_index, 0);
                    }
                    self.index += 4
                }
                9 => {
                    let input1 = self.get_parameter(self.index, 1);
                    self.relative_base += input1;
                    self.index += 2
                }
                // Halt program
                99 => return IntCodeState::Halted(self.output.clone()),
                x => {
                    println!("{:?} [{}]: {}", self.program, self.index, x);
                    panic!()
                }
            }
        }
        unreachable!()
    }
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
        assert_eq!(day10::star_one(get_data(&filepath)), 319);

        assert_eq!(day10::star_two(get_data(&filepath)), 517);
    }

    #[test]
    fn day11_complete() {
        let filepath = Path::new("data").join("day11.txt");
        assert_eq!(day11::star_one(get_data(&filepath)), 2392);

        let expected = "\
        .####..##..###..#..#.#....####.#..#.####...\n\
        .#....#..#.#..#.#..#.#....#....#..#.#......\n\
        .###..#....###..####.#....###..#..#.###....\n\
        .#....#.##.#..#.#..#.#....#....#..#.#......\n\
        .#....#..#.#..#.#..#.#....#....#..#.#......\n\
        .####..###.###..#..#.####.####..##..####...";
        assert_eq!(day11::star_two(get_data(&filepath)), expected);
    }

    #[test]
    fn day12_complete() {
        let filepath = Path::new("data").join("day12.txt");
        assert_eq!(day12::star_one(get_data(&filepath)), 8742);

        assert_eq!(day12::star_two(get_data(&filepath)), 325433763467176);
    }

    #[test]
    fn day13_complete() {
        let filepath = Path::new("data").join("day13.txt");
        assert_eq!(day13::star_one(get_data(&filepath)), 335);

        assert_eq!(day13::star_two(get_data(&filepath)), 15706);
    }

    #[test]
    fn day14_complete() {
        let filepath = Path::new("data").join("day14.txt");
        assert_eq!(day14::star_one(get_data(&filepath)), 220019);

        assert_eq!(day14::star_two(get_data(&filepath)), 5650230);
    }

    #[test]
    fn day15_complete() {
        let filepath = Path::new("data").join("day15.txt");
        assert_eq!(day15::star_one(get_data(&filepath)), 216);

        assert_eq!(day15::star_two(get_data(&filepath)), 326);
    }

    #[test]
    fn day16_complete() {
        let filepath = Path::new("data").join("day16.txt");
        assert_eq!(day16::star_one(get_data(&filepath)), 84487724);

        assert_eq!(day16::star_two(get_data(&filepath)), 84692524);
    }

    #[test]
    fn day17_complete() {
        let filepath = Path::new("data").join("day17.txt");
        assert_eq!(day17::star_one(get_data(&filepath)), 6000);

        assert_eq!(day17::star_two(get_data(&filepath)), 807320);
    }

    #[test]
    fn day18_complete() {
        let filepath = Path::new("data").join("day18.txt");
        assert_eq!(day18::star_one(get_data(&filepath)), 4590);

        assert_eq!(day18::star_two(get_data(&filepath)), 2086);
    }

    #[test]
    fn day19_complete() {
        let filepath = Path::new("data").join("day19.txt");
        assert_eq!(day19::star_one(get_data(&filepath)), 211);

        assert_eq!(day19::star_two(get_data(&filepath)), 8071006);
    }

    #[test]
    fn day20_complete() {
        let filepath = Path::new("data").join("day20.txt");
        assert_eq!(day20::star_one(get_data(&filepath)), 668);

        assert_eq!(day20::star_two(get_data(&filepath)), 7778);
    }

    #[test]
    fn day21_complete() {
        let filepath = Path::new("data").join("day21.txt");
        assert_eq!(day21::star_one(get_data(&filepath)), 19348404);

        assert_eq!(day21::star_two(get_data(&filepath)), 1139206699);
    }

    #[test]
    fn day22_complete() {
        let filepath = Path::new("data").join("day22.txt");
        assert_eq!(day22::star_one(get_data(&filepath)), 7096);

        assert_eq!(day22::star_two(get_data(&filepath)), 27697279941366);
    }

    #[test]
    fn day23_complete() {
        let filepath = Path::new("data").join("day23.txt");
        assert_eq!(day23::star_one(get_data(&filepath)), 20665);

        assert_eq!(day23::star_two(get_data(&filepath)), 13358);
    }

    #[test]
    fn day24_complete() {
        let filepath = Path::new("data").join("day24.txt");
        assert_eq!(day24::star_one(get_data(&filepath)), 18350099);

        assert_eq!(day24::star_two(get_data(&filepath)), 2037);
    }

    #[test]
    fn day25_complete() {
        let filepath = Path::new("data").join("day25.txt");
        assert_eq!(day25::star_one(get_data(&filepath)), 134227456);

        assert_eq!(day25::star_two(get_data(&filepath)), 0);
    }
}
