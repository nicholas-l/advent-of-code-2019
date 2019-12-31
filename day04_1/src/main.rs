
const MAX_VALUE: i32 = 824795;
const MIN_VALUE: i32 = 278384;

// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

fn main() {
    let values: Vec<i32> = (MIN_VALUE..=MAX_VALUE)
        // It is a six-digit number.
        .filter(|d| d.to_string().len() == 6)
        // Two adjacent digits are the same (like 22 in 122345).
        .filter(|d| d.to_string().chars().collect::<Vec<char>>().windows(2).any(|a| a[0] == a[1]))
        .filter(|d| d.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>().windows(2).all(|a| a[0] <= a[1]))
        .collect::<Vec<i32>>();
    println!("{}", values.len());
}
