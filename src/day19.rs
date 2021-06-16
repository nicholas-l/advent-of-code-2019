use std::{collections::HashSet, io::BufRead};

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
    (0..50)
        .map(|y| {
            (0..50)
                .filter_map(|x| {
                    let mut program = codes.clone();
                    let mut computer = IntCode::new(&mut program, 0, vec![x, y]);
                    computer.run(2);
                    let mut output = computer.take_output();
                    // assert_eq!(output.len(), 1);
                    output.pop()
                })
                .filter(|&x| x == 1)
                .count()
        })
        .sum()
}

fn get_point(codes: &[isize], x: usize, y: usize) -> isize {
    let mut program = codes.to_vec();
    let mut computer = IntCode::new(&mut program, 0, vec![x as isize, y as isize]);
    computer.run(1);
    let output = computer.take_output();
    // assert_eq!(output.len(), 1);
    output[0]
}

fn get_square(
    map: &HashSet<(usize, usize)>,
    map_width: usize,
    map_height: usize,
    width: usize,
) -> bool {
    let last_y = (0..map_height)
        .find(|&y| map.contains(&(map_width, y)))
        .unwrap();
    (0..width).all(|x| map.contains(&(map_width - x, last_y)))
        && (0..width).all(|y| map.contains(&(map_width - width, last_y + y)))
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
    let sample_x = 500;
    let upper = (0..)
        .map(|y| {
            let mut program = codes.clone();
            let mut computer = IntCode::new(&mut program, 0, vec![sample_x, y]);
            computer.run(2);
            let output = computer.take_output();
            // assert_eq!(output.len(), 1);
            if output[0] == 1 {
                '#'
            } else {
                '.'
            }
        })
        .enumerate()
        .find(|&(_, x)| x == '#')
        .map(|x| x.0)
        .unwrap();
    let lower = (upper..)
        .map(|y| {
            let mut program = codes.clone();
            let mut computer = IntCode::new(&mut program, 0, vec![sample_x, y as isize]);
            computer.run(2);
            let output = computer.take_output();
            // assert_eq!(output.len(), 1);
            if output[0] == 1 {
                '#'
            } else {
                '.'
            }
        })
        .enumerate()
        .find(|&(_, x)| x == '.')
        .map(|x| x.0)
        .unwrap();

    let lower_angle = sample_x as f64 / (upper - 2) as f64;
    let upper_angle = sample_x as f64 / (lower  + 2) as f64;

    println!("{} - {}", upper_angle, lower_angle);

    let mut max_width = 1500;
    let mut max_height = 1500;
    let mut map: HashSet<(usize, usize)> = (0..=max_height)
        .flat_map(|y| {
          let lower_x = f64::floor(lower_angle * (y as f64)) as usize;
          let upper_x = f64::ceil(upper_angle * (y as f64)) as usize;
          // println!("{} - {}", lower_x, upper_x);
            (lower_x..=upper_x)
                .map(move |x| (x, y))
                .filter(|pos| get_point(&codes, pos.0, pos.1) == 1)
        })
        .collect();
    println!("Starting checks");
    while !get_square(&map, max_width, max_height, 100) {
        map.extend(
            (0..=max_height)
                .map(|y| (max_width, y))
                .filter(|pos| get_point(&codes, pos.0, pos.1) == 1),
        );
        max_width += 1;

        // add row

        map.extend(
            (0..=max_width)
                .map(|x| (x, max_width))
                .filter(|pos| get_point(&codes, pos.0, pos.1) == 1),
        );

        max_height += 1;
        if max_width % 10 == 0 {
            println!("{}", max_width);
        }
    }
    // for row in map {
    //     for x in row {
    //         match x {
    //             0 => print!("."),
    //             1 => print!("#"),
    //             _ => panic!(),
    //         }
    //     }
    //     println!();
    // }
    // let mut program = codes.clone();
    // let mut computer = IntCode::new(&mut program, 0, vec![]);
    // computer.run(0);
    // let output = computer.take_output();
    // (map.last().unwrap().len() - 100) * 1000 +
    map.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}
}
