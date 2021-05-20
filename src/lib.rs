pub mod day01;
pub mod day02;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::BufRead, path::{Path, PathBuf}};
    use std::io::BufReader;

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
}
