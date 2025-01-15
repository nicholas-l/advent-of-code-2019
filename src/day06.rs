use std::{collections::HashMap, io::BufRead};

fn get_distance(objects: &HashMap<&str, &str>, key: &str) -> usize {
    objects
        .get(key)
        .map(|k| get_distance(objects, k) + 1)
        .unwrap_or(0)
}

pub fn star_one(mut input: impl BufRead) -> usize {
    // FIXME Change to not reading into a string.
    let mut buf = String::new();
    let _input_file = input.read_to_string(&mut buf);
    let objects = buf
        .lines()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut objects, orbit| {
            objects.insert(orbit[1], orbit[0]);
            objects
        });
    objects
        .keys()
        .map(|key| get_distance(&objects, key))
        .sum::<usize>()
}

fn get_shortest_distance(objects: &HashMap<&str, &str>, start: &str, end: &str) -> usize {
    // Go back to COM from START
    let mut path = vec![start];
    while let Some(object) = objects.get(path.last().unwrap()) {
        path.push(object);
    }

    let mut path2 = vec![end];
    let mut current = end;
    // GO back to COM from end while checking if we have already been there from start
    while let Some(object) = objects.get(current) {
        path2.push(object);
        if let Some(index) = path.iter().position(|&r| r == *object) {
            let path3 = &path[..index];
            path2.reverse();
            println!("{:?}", path3);
            println!("{:?}", path2);
            return path3.len() + path2.len() - 3;
        }
        current = object;
    }
    panic!();
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _input_file = input.read_to_string(&mut buf);
    let objects = buf
        .lines()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut objects, orbit| {
            objects.insert(orbit[1], orbit[0]);
            objects
        });

    get_shortest_distance(&objects, "YOU", "SAN")
}
