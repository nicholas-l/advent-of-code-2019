use std::cmp::{PartialEq, PartialOrd};
use std::fs;

#[derive(PartialEq)]
enum LineType {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

fn intersection(a: &Line, b: &Line) -> Option<Point> {
    let point = Point { x: a.0.x, y: b.0.y };
    if a.0.y.max(a.1.y) >= point.y
        && a.0.y.min(a.1.y) <= point.y
        && b.0.x.max(b.1.x) >= point.x
        && b.0.x.min(b.1.x) <= point.x
    {
        return Some(point);
    }
    return None;
}

#[derive(Clone, Debug)]
struct Line(Point, Point);

impl Line {
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    fn intersects(&self, other: &Line) -> Option<Point> {
        if self.line_direction() == other.line_direction() {
            return None;
        }
        if other.line_direction() == LineType::Horizontal {
            return intersection(self, other);
        }
        return intersection(other, self);
    }

    fn line_direction(&self) -> LineType {
        if self.0.x == self.1.x {
            return LineType::Vertical;
        } else if self.0.y == self.1.y {
            return LineType::Horizontal;
        }
        unreachable!();
    }
}

#[derive(Debug)]
struct Wire {
    lines: Vec<Line>,
}

fn parse(string: &str) -> Wire {
    let mut lines: Vec<Line> = vec![];
    let mut starting_point = Point { x: 0, y: 0 };
    for instruction in string.split(",") {
        let current_point = match &instruction[0..1] {
            "R" => Point {
                x: starting_point.x + instruction[1..].parse::<isize>().unwrap(),
                y: starting_point.y,
            },
            "L" => Point {
                x: starting_point.x - instruction[1..].parse::<isize>().unwrap(),
                y: starting_point.y,
            },
            "U" => Point {
                x: starting_point.x,
                y: starting_point.y + instruction[1..].parse::<isize>().unwrap(),
            },
            "D" => Point {
                x: starting_point.x,
                y: starting_point.y - instruction[1..].parse::<isize>().unwrap(),
            },
            _ => panic!("{}", instruction),
        };
        lines.push(Line(starting_point.clone(), current_point.clone()));
        starting_point = current_point;
    }
    // println!("{}", string);
    return Wire { lines };
}

fn main() {
    let wires: Vec<Wire> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(parse)
        .collect();
    let mut intersections = vec![];
    // TODO: Change to be not as O(N^2)
    for i in 0..wires.len() {
        for j in (i+1)..wires.len() {
            for segment1 in &wires[i].lines {
                for segment2 in &wires[j].lines {
                    match segment1.intersects(&segment2) {
                        Some(point) => intersections.push(point),
                        None => {}
                    }
                }
            }
        }
    }
    // println!("Wires: {:?}", wires);
    // println!("Intersections: {:?}", intersections);
    match intersections.into_iter().min_by_key(|a| a.x.abs() + a.y.abs()) {
        Some(val) => println!("{:?} {}", val, val.x.abs() + val.y.abs()),
        None => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let a = Line(Point { x: 1, y: 0 }, Point { x: 1, y: 2 });
        let b = Line(Point { x: 0, y: 1 }, Point { x: 2, y: 1 });
        assert_eq!(intersection(&a, &b), Some(Point { x: 1, y: 1 }));
    }
}
