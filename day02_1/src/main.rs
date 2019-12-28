use std::fs;

fn main() {
    let mut codes: Vec<usize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut index = 0;
    while codes[index] != 99 {
        let input1_index = codes[index + 1];
        let input2_index = codes[index + 2];
        let output_index = codes[index + 3];
        match codes[index]  {
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
    println!("{:?}", codes);
}
