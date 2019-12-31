use std::fs;
// TODO change to getting the input values.
fn process(codes: &mut Vec<isize>, input: isize) -> Vec<isize> {
    let mut index = 0;
    let mut output = vec!();
    while codes[index] != 99 {
        match codes[index] % 100 {
            1 => {
                let output_index = codes[index + 3] as usize;
                let input1 = if codes[index] / 100 % 10 == 1 { codes[index + 1] } else { codes[codes[index + 1] as usize] };
                let input2 = if (codes[index] / 1000) % 10 == 1 { codes[index + 2] } else { codes[codes[index + 2] as usize] };
                codes[output_index] = input1 + input2;
                index += 4;
            },
            2 => {
                let output_index = codes[index + 3] as usize;
                let input1 = if (codes[index] / 100) % 10 == 1 { codes[index + 1] } else { codes[codes[index + 1] as usize] };
                let input2 = if (codes[index] / 1000) % 10 == 1 { codes[index + 2] } else { codes[codes[index + 2] as usize] };
                codes[output_index] = input1 * input2;
                index += 4;
            },
            3 => {
                let output_index = codes[index + 1] as usize;
                codes[output_index] = input;
                index += 2
            }
            4 => {
                let input1 = if codes[index] / 100 % 10 == 1 { codes[index + 1] } else { codes[codes[index + 1] as usize] };
                output.push(input1);
                index += 2
            }
            _ => {
                println!("{:?} ({})", codes, index);
                panic!()
            }
        }
    }
    return output;
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
    let output = process(&mut codes, 1);
    println!("{:?}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let mut program = vec!(1101,100,-1,4,0);
        let output = process(&mut program);
        assert_eq!(program, vec!(1101,100,-1,4,99));
    }

    #[test]
    fn test_program_2() {
        let mut program = vec!(1002,4,3,4,33);
        let output = process(&mut program);
        assert_eq!(program, vec!(1002,4,3,4,99));
    }
}