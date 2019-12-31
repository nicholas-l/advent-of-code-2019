use std::collections::HashMap;
use std::fs;

fn get_shortest_distance(objects: &HashMap<&str,&str>, start: &str, end: &str) -> usize {
    // Go back to COM from START
    let mut path = vec![start];
    while let Some(object) = objects.get(path.last().unwrap()) {
        path.push(object);
    }
    
    let mut path2 = vec![end];
    let mut current = end;
    let mut count = 0;
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
        count += 1;
    }
    panic!();
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
    
    println!("{}", get_shortest_distance(&objects, "YOU", "SAN"));
}
