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

struct IntCode<'a> {
    program: &'a mut Vec<isize>,
    input: Vec<isize>,
    index: usize,
    output: Vec<isize>,
    // halted: bool,
    relative_base: isize,
}

impl IntCode<'_> {
    fn new(program: &'_ mut Vec<isize>, start_index: usize, input: Vec<isize>) -> IntCode<'_> {
        IntCode {
            program,
            input,
            index: start_index,
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
    fn run(&mut self, output_max: usize) -> (usize, bool) {
        let mut found_99 = false;
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
                    let data = self.input.remove(0);
                    self.write(output_index, data);
                    self.index += 2
                }
                4 => {
                    let output_parameter = self.get_parameter(self.index, 1);
                    self.output.push(output_parameter);
                    self.index += 2;
                    if output_max > 0 && self.output.len() >= output_max {
                        break;
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
                99 => {
                    found_99 = true;
                    break; // return None,
                }
                x => {
                    println!("{:?} [{}]: {}", self.program, self.index, x);
                    panic!()
                }
            }
        }
        (self.index, found_99)
    }
}
// TODO change to getting the input values.
pub fn process(
    codes: &mut Vec<isize>,
    start_index: usize,
    input: &mut Vec<isize>,
    stop_if_output: bool,
) -> (usize, Vec<isize>, bool) {
    let mut computer = IntCode::new(codes, start_index, input.to_vec());

    let (index, found_99) = computer.run(if stop_if_output { 1 } else { 0 });

    (index, computer.take_output(), found_99)
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

        // assert_eq!(day16::star_two(get_data(&filepath)), 326);
    }
}
