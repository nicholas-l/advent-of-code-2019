use std::{io::BufRead, iter::once};

use crate::IntCode;

/*
Manually solve this day by writing out the path and dividing into three like parts.
 */
fn print_image(image: &[Vec<isize>]) {
    for row in image {
        for x in row {
            let c = *x as u8 as char;
            print!("{}", c);
        }
        println!()
    }
}

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
    let mut computer = IntCode::new(codes, 0, vec![]);
    computer.run(0);
    let output = computer.take_output();

    let image: Vec<Vec<isize>> = output
        .split(|&x| x == 10)
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect();
    print_image(&image);
    let mut intersections = Vec::new();
    for y in 1..(image.len() - 1) {
        for x in 1..(image[y].len() - 1) {
            // print!("{} ", image[y][x]);
            if image[y][x] == b'#' as isize
                && image[y - 1][x] == b'#' as isize
                && image[y + 1][x] == b'#' as isize
                && image[y][x - 1] == b'#' as isize
                && image[y][x + 1] == b'#' as isize
            {
                intersections.push((x, y));
            }
        }
        // println!()
    }
    intersections.iter().map(|(x, y)| x * y).sum()
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
    let program = codes.clone();
    let mut computer = IntCode::new(program, 0, vec![]);
    computer.run(0);
    let output = computer.take_output();

    let _image: Vec<Vec<isize>> = output
        .split(|&x| x == 10)
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect();
    // print_image(&image);

    println!("Setting code[0] to 2");

    let mut program = codes;
    program[0] = 2;
    // let input = "\
    let commands = "B,C,B,C,B,A,C,A,B,A";
    let a = "L,8,L,8,R,10,R,4";
    let b = "R,4,L,10,L,10";
    let c = "L,8,R,12,R,10,R,4";

    let new_commands: Vec<isize> = commands
        .chars()
        .map(|c| c as isize)
        .chain(once('\n' as isize))
        .chain(a.chars().map(|c| c as isize))
        .chain(once('\n' as isize))
        .chain(b.chars().map(|c| c as isize))
        .chain(once('\n' as isize))
        .chain(c.chars().map(|c| c as isize))
        .chain(once('\n' as isize))
        .chain(once('n' as isize))
        .chain(once('\n' as isize))
        .collect();
    // println!("{}", new_commands.iter().map(|&c| c as u8 as char).collect::<String>());

    let mut computer = IntCode::new(program, 0, new_commands);
    computer.run(0);
    let output = computer.take_output();

    let _image: Vec<Vec<isize>> = output
        .split(|&x| x == 10)
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect();
    // print_image(&image);

    *output.last().unwrap() as usize
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}
}
