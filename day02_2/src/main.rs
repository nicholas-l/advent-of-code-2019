use std::fs;

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
    return codes[0];
}

fn main() {
    let codes: Vec<usize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
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
                return;
            }
        }
    }
    println!("{:?}", codes);
}
