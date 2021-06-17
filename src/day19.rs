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
                .map(|x| get_point(codes.clone(), x, y))
                .filter(|&x| x == 1)
                .count()
        })
        .sum()
}

fn get_point(mut program: Vec<isize>, x: usize, y: usize) -> isize {
    let mut computer = IntCode::new(&mut program, 0, vec![x as isize, y as isize]);
    computer.run(1);
    let output = computer.take_output();
    // assert_eq!(output.len(), 1);
    output[0]
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
    let sample_x = 40;
    let upper = dbg!((0..)
        .map(|y| {
            if get_point(codes.clone(), sample_x, y) == 1 {
                '#'
            } else {
                '.'
            }
        })
        .enumerate()
        .find(|&(_, x)| x == '#')
        .map(|x| x.0)
        .unwrap());

    let lower = dbg!(
        upper
            + (upper..)
                .map(|y| {
                    if get_point(codes.clone(), sample_x, y) == 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .enumerate()
                .find(|&(_, x)| x == '.')
                .map(|x| x.0)
                .unwrap()
    );

    let upper_angle = sample_x as f64 / (upper - 2) as f64;
    let lower_angle = sample_x as f64 / (lower + 2) as f64;

    println!("{} - {}", upper_angle, lower_angle);

    let max_width = 1200;
    let max_height = 1200;
    println!("{}", lower_angle * 100f64 / (upper_angle - lower_angle));
    let map: HashSet<(usize, usize)> = (0..=max_height)
        .flat_map(|y| {
            let lower_x = f64::floor(lower_angle * (y as f64)) as usize;
            let upper_x = f64::ceil(upper_angle * (y as f64)) as usize;
            // println!("{} - {}", lower_x, upper_x);
            (lower_x..=upper_x)
                .map(move |x| (x, y))
                .filter(|pos| get_point(codes.clone(), pos.0, pos.1) == 1)
        })
        .collect();

    println!("Starting checks");
    for y in 0..max_height {
        for x in 0..max_width {
            if map.contains(&(x, y)) && map.contains(&(x + 99, y)) && map.contains(&(x, y + 99)) {
                return x * 10000 + y;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}
}
