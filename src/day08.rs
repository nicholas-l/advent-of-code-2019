use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct Layer(Vec<u8>);

impl Layer {
    fn get_hash(&self) -> usize {
        let (ones, twos) = self.0.iter().fold((0, 0), |c, x| match x {
            1 => (c.0 + 1, c.1),
            2 => (c.0, c.1 + 1),
            _ => c,
        });
        ones * twos
    }
}

struct Picture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Picture {
    fn layers<'a>(&'a self) -> impl Iterator<Item = Layer> + 'a {
        self.data
            .chunks(self.width * self.height)
            .map(|v| Layer(v.to_owned()))
    }

    fn get_pixel(&self, y: usize, x: usize) -> char {
        for l in self.layers() {
            match l.0[self.width * y + x] {
                0 => return '0',
                1 => return '1',
                _ => {}
            }
        }
        unreachable!()
    }

    fn get_picture(&self) -> String {
        let mut v = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                v.push(self.get_pixel(y, x))
            }
            v.push('\n');
        }
        v.into_iter().collect()
    }
}

fn to_layers<'a>(input: &'a [u8], width: usize, height: usize) -> impl Iterator<Item = Layer> + 'a {
    input.chunks(width * height).map(|v| Layer(v.to_owned()))
}

fn process(data: &[u8], width: usize, height: usize) -> usize {
    let layer = to_layers(data, width, height)
        .min_by_key(|l| l.0.iter().filter(|&x| x == &0).count())
        .unwrap();

    layer.get_hash()
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_text = String::new();
    input.read_to_string(&mut input_text);
    let data: Vec<u8> = input_text
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    process(&data, 25, 6)
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut input_text = String::new();
    input.read_to_string(&mut input_text);
    let data: Vec<u8> = input_text
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let p = Picture {
        data,
        width: 25,
        height: 6,
    };
    println!("{}", p.get_picture());
    "CFLUL".into()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_star_one() {
        let data: Vec<u8> = "123456789012"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        assert_eq!(process(&data, 3, 2), 1);
    }

    #[test]
    fn test_star_two() {
        let data: Vec<u8> = "0222112222120000"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let p = Picture {
            data,
            width: 2,
            height: 2,
        };
        assert_eq!(p.get_picture().trim(), "01\n10");
    }
}
