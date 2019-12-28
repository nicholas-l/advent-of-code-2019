use std::fs;

fn process(mass: usize) -> usize {
    return mass / 3 - 2;
}

fn main() {
    println!("{}", process(12));
    println!("{}", process(14));
    println!("{}", process(1969));
    println!("{}", process(100756));
    let sum: usize = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(process)
        .sum();
    println!("{}", sum);
}
