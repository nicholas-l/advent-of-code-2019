pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::{
        fs,
        io::BufRead,
        path::{Path, PathBuf},
    };

    fn get_data(filepath: &PathBuf) -> Box<dyn BufRead> {
        let f = fs::File::open(filepath).unwrap();
        let input = BufReader::new(f);
        Box::new(input)
    }

    #[test]
    fn day01_complete() {
        let filepath = Path::new("data").join("day01.txt");
        assert_eq!(day01::star_one(get_data(&filepath)), 3512133);

        assert_eq!(day01::star_two(get_data(&filepath)), 5265294);
    }

    #[test]
    fn day02_complete() {
        let filepath = Path::new("data").join("day02.txt");
        assert_eq!(day02::star_one(get_data(&filepath)), 3516593);

        assert_eq!(day02::star_two(get_data(&filepath)), 7749);
    }

    #[test]
    fn day03_complete() {
        let filepath = Path::new("data").join("day03.txt");
        assert_eq!(day03::star_one(get_data(&filepath)), 1211);

        assert_eq!(day03::star_two(get_data(&filepath)), 101386);
    }

    #[test]
    fn day04_complete() {
        let filepath = Path::new("data").join("day04.txt");
        assert_eq!(day04::star_one(get_data(&filepath)), 921);

        assert_eq!(day04::star_two(get_data(&filepath)), 603);
    }

    #[test]
    fn day05_complete() {
        let filepath = Path::new("data").join("day05.txt");
        assert_eq!(day05::star_one(get_data(&filepath)), 9961446);

        assert_eq!(day05::star_two(get_data(&filepath)), 742621);
    }

    #[test]
    fn day06_complete() {
        let filepath = Path::new("data").join("day06.txt");
        assert_eq!(day06::star_one(get_data(&filepath)), 273985);

        assert_eq!(day06::star_two(get_data(&filepath)), 460);
    }

    #[test]
    fn day07_complete() {
        let filepath = Path::new("data").join("day07.txt");
        assert_eq!(day07::star_one(get_data(&filepath)), 46014);

        assert_eq!(day07::star_two(get_data(&filepath)), 19581200);
    }
}
