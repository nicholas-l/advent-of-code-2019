use std::io::BufRead;

const MAX_VALUE: i32 = 824795;
const MIN_VALUE: i32 = 278384;

pub fn star_one(_input: impl BufRead) -> usize {
    (MIN_VALUE..=MAX_VALUE)
        // It is a six-digit number.
        .filter(|d| d.to_string().len() == 6)
        // Two adjacent digits are the same (like 22 in 122345).
        .filter(|d| {
            d.to_string()
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .any(|a| a[0] == a[1])
        })
        .filter(|d| {
            d.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
                .windows(2)
                .all(|a| a[0] <= a[1])
        })
        .count()
}

fn check(d: &i32) -> bool {
    let v = d
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let adjacent = v.windows(2).any(|a| a[0] == a[1]);
    let increasing = v.windows(2).all(|a| a[0] <= a[1]);
    let ok = v
        .iter()
        .fold(vec![0; 10], |mut acc, a| {
            acc[*a as usize] += 1;
            acc
        })
        .into_iter()
        .any(|a| a == 2);
    adjacent && increasing && ok
}

// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

pub fn star_two(_input: impl BufRead) -> usize {
    (MIN_VALUE..=MAX_VALUE)
        // It is a six-digit number.
        .filter(|d| d.to_string().len() == 6)
        // Two adjacent digits are the same (like 22 in 122345).
        .filter(check)
        .count()
}

#[cfg(test)]
mod tests {
    // use std::io::Cursor;

    use super::*;

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {}

    #[test]
    fn test_check() {
        assert_eq!(check(&112233), true);
        assert_eq!(check(&123444), false);
        assert_eq!(check(&111122), true);
        assert_eq!(check(&111111), false);
        assert_eq!(check(&223450), false);
        assert_eq!(check(&123789), false);
    }
}
