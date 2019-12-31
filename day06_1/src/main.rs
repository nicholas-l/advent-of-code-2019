use std::collections::HashMap;
use std::fs;

fn get_distance(objects: &HashMap<&str,&str>, key: &str) -> usize {
    return objects.get(key).map(|k| get_distance(objects, k) + 1).unwrap_or(0);
}

fn main() {
    let input_file = fs::read_to_string("./input.txt")
    .unwrap();
    let objects = input_file
        .lines()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut objects, orbit| {
            objects.insert(orbit[1], orbit[0]);
            objects
        });
    let sum = objects.keys().map(|key| get_distance(&objects, key)).sum::<usize>();
    println!("{}", sum);
}
