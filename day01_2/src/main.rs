use std::fs;

fn process(mass: isize) -> isize {
    let fuel = mass / 3 - 2;
    match fuel {
        f if f <= 0 => 0,
        f => f + process(f),
    }
}

fn main() {
    println!("{}", process(12));
    println!("{}", process(14));
    println!("{}", process(1969));
    println!("{}", process(100756));
    let sum: isize = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .map(process)
        .sum();
    println!("{}", sum);
}
