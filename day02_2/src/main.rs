use std::fs;

fn process(mut codes: Vec<usize>) -> Vec<usize> {
    let mut index = 0;
    while codes[index] != 99 {
        let input1_index = codes[index + 1];
        let input2_index = codes[index + 2];
        let output_index = codes[index + 3];
        match codes[index] {
            1 => {
                codes[output_index] = codes[input1_index] + codes[input2_index];
            }
            2 => {
                codes[output_index] = codes[input1_index] * codes[input2_index];
            }
            _ => todo!(),
        }
        index += 4;
    }
    return codes;
}

fn main() {
    let codes: Vec<usize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", codes);
    for noun in 1..100 {
        for verb in 1..100 {
            let mut code = codes.clone();
            code[1] = noun;
            code[2] = verb;
            let code2 = process(code);

            if code2[0] == 19690720 {
                println!("{} {}", noun, verb);
                return;
            }
        }
    }
    println!("{:?}", codes);
}
