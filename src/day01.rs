use std::io::BufRead;

fn process(mass: usize) -> usize {
    mass / 3 - 2
}

fn process2(mass: isize) -> isize {
    let fuel = mass / 3 - 2;
    match fuel {
        f if f <= 0 => 0,
        f => f + process2(f),
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .map(process)
        .sum()
}

pub fn star_two(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .map(process2)
        .sum::<isize>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(process(12), 2);
        assert_eq!(process(14), 2);
        assert_eq!(process(1969), 654);
        assert_eq!(process(100756), 33583);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(process2(12), 2);
        assert_eq!(process2(14), 2);
        assert_eq!(process2(1969), 966);
        assert_eq!(process2(100756), 50346);
    }
}
