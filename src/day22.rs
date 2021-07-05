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

fn star_one_stack(instructions: &[Instruction], mut stack: Vec<isize>) -> Vec<isize> {
    for instruction in instructions {
        stack = match instruction {
            Instruction::NewStack => stack.into_iter().rev().collect(),
            Instruction::Cut(x) => {
                if x > &0 {
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
                    pos = (pos + *x as usize) % table.len();
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
    let output = star_one_stack(&instructions, (0..stack_size).collect());
    output.into_iter().position(|x| x == 2019).unwrap()
}

// adapted from https://github.com/simon-andrews/rust-modinverse/blob/master/src/lib.rs
pub fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, a_new, b_new) = egcd(b % a, a);
        (g, b_new - (b / a) * a_new, a_new)
    }
}

pub fn modinverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

fn get_multiplier(instructions: &[Instruction], stack_size: i128) -> i128 {
    instructions
        .iter()
        .rev()
        .fold(1i128, |multiplier, instruction| match instruction {
            Instruction::NewStack => (multiplier * (stack_size - 1)) % stack_size,
            Instruction::Cut(_x) => multiplier,
            Instruction::Increment(x) => {
                let x = modinverse(*x as i128, stack_size as i128).unwrap();
                (multiplier * x as i128) % stack_size
            }
        })
}

fn get_addition(instructions: &[Instruction], stack_size: i128) -> i128 {
    instructions
        .iter()
        .rev()
        .fold(0i128, |addition, instruction| match instruction {
            Instruction::NewStack => ((addition + 1) * (stack_size - 1)) % stack_size,
            Instruction::Cut(x) => {
                let x = if x < &0 {
                    stack_size + *x as i128
                } else {
                    *x as i128
                };
                (addition + x) % stack_size
            }
            Instruction::Increment(x) => {
                let x = modinverse(*x as i128, stack_size as i128).unwrap();
                (addition * x) % stack_size
            }
        })
}

// Adapted from https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

pub fn star_two(input: impl BufRead) -> usize {
    let instructions = parse_instructions(input);
    let stack_size = 119315717514047;
    let repeats = 101741582076661;
    let requested_position = 2020;

    let multiplier = get_multiplier(&instructions, stack_size);
    let addition = get_addition(&instructions, stack_size);

    let mx = mod_pow(multiplier, repeats, stack_size);
    let pmx = (requested_position * mx) % stack_size;
    let amx = (addition * mx) % stack_size;
    let inv = modinverse(multiplier - 1, stack_size).unwrap();
    let res = (pmx + (amx - addition) * inv) % stack_size;
    if res >= 0 {
        res as usize
    } else {
        (res + stack_size) as usize
    }
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
            star_one_stack(&input, (0..10).collect()),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        );
        let input = parse_instructions(Cursor::new(INPUT2));
        assert_eq!(
            star_one_stack(&input, (0..10).collect()),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        );
        let input = parse_instructions(Cursor::new(INPUT3));
        assert_eq!(
            star_one_stack(&input, (0..10).collect()),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        );

        let input = parse_instructions(Cursor::new(INPUT4));
        assert_eq!(
            star_one_stack(&input, (0..10).collect()),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
        );
    }

    #[test]
    fn test_star_two() {}
}
