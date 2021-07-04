use std::{io::BufRead, str::FromStr};

enum Instruction {
    NewStack,
    Cut(isize),
    Increment(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.rsplitn(2, ' ').collect();
        match parts[1] {
            "deal into new" => Ok(Instruction::NewStack),
            "cut" => Ok(Instruction::Cut(
                parts[0].parse().unwrap_or_else(|_| panic!("{}", s)),
            )),
            "deal with increment" => Ok(Instruction::Increment(parts[0].parse().unwrap())),
            x => panic!("{}: {}", s, x),
        }
    }
}

fn star_one_stack(instructions: Vec<Instruction>, mut stack: Vec<isize>) -> Vec<isize> {
    for instruction in instructions {
        stack = match instruction {
            Instruction::NewStack => stack.into_iter().rev().collect(),
            Instruction::Cut(x) => {
                if x > 0 {
                    stack.rotate_left(x.abs() as usize);
                } else {
                    stack.rotate_right(x.abs() as usize);
                }
                stack
            }
            Instruction::Increment(x) => {
                let mut table = stack.clone();
                let mut pos = 0;
                for i in stack {
                    table[pos] = i;
                    pos = (pos + x as usize) % table.len();
                }
                table
            }
        }
    }
    stack
}

fn parse_instructions(input: impl BufRead) -> Vec<Instruction> {
    input
        .lines()
        .map(|v| {
            // println!("{}", &v);
            Instruction::from_str(&v.unwrap()).unwrap()
        })
        .collect()
}

pub fn star_one(input: impl BufRead) -> usize {
    let instructions = parse_instructions(input);
    let stack_size = 10007;
    let output = star_one_stack(instructions, (0..stack_size).collect());
    output.into_iter().position(|x| x == 2019).unwrap()
}

pub fn star_two(_input: impl BufRead) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT1: &str = "deal with increment 7
deal into new stack
deal into new stack";

    const INPUT2: &str = "cut 6
deal with increment 7
deal into new stack";

    const INPUT3: &str = "deal with increment 7
deal with increment 9
cut -2";

    const INPUT4: &str = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

    #[test]
    fn test_star_one() {
        let input = parse_instructions(Cursor::new(INPUT1));
        assert_eq!(
            star_one_stack(input, (0..10).collect()),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        );
        let input = parse_instructions(Cursor::new(INPUT2));
        assert_eq!(
            star_one_stack(input, (0..10).collect()),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        );
        let input = parse_instructions(Cursor::new(INPUT3));
        assert_eq!(
            star_one_stack(input, (0..10).collect()),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        );

        let input = parse_instructions(Cursor::new(INPUT4));
        assert_eq!(
            star_one_stack(input, (0..10).collect()),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
        );
    }

    #[test]
    fn test_star_two() {}
}
