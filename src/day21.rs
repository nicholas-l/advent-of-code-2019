use std::io::BufRead;

use crate::IntCode;

pub fn star_one(input: impl BufRead) -> usize {
    let codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            // println!("{}", &v);
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    let commands = "NOT A T
NOT C J
OR T J
AND D J
WALK
";
    let input = commands.chars().map(|c| c as isize).collect();
    let mut computer = IntCode::new(codes, 0, input);
    computer.run(0);
    let output = computer.take_output();
    // for &o in &output {
    //     if o < 300 {
    //         print!("{}", char::from_u32(o as u32).unwrap());
    //     }
    // }
    *output.last().unwrap() as usize
}

pub fn star_two(input: impl BufRead) -> usize {
    let codes: Vec<isize> = input
        .split(b',')
        .map(|v| {
            // println!("{}", &v);
            String::from_utf8(v.unwrap())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();
    let commands = "NOT C J 
AND D J 
AND H J
NOT B T 
AND D T 
OR T J
NOT A T 
OR T J
RUN
";
    let input = commands.chars().map(|c| c as isize).collect();
    // println!("{:?}", input);
    let mut computer = IntCode::new(codes, 0, input);
    computer.run(0);
    let output = computer.take_output();
    // for &o in &output {
    //     if o < 300 {
    //         print!("{}", char::from_u32(o as u32).unwrap());
    //     }
    // }
    *output.last().unwrap() as usize
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}
}
