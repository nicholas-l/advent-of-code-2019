use std::{cmp::PartialEq, io::BufRead};

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

impl Point {
    fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
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
    None
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
        intersection(other, self)
    }

    fn line_direction(&self) -> LineType {
        if self.0.x == self.1.x {
            return LineType::Vertical;
        } else if self.0.y == self.1.y {
            return LineType::Horizontal;
        }
        unreachable!();
    }

    fn length(&self) -> isize {
        (self.0.x - self.1.x).abs() + (self.0.y - self.1.y).abs()
    }
}

#[derive(Debug)]
struct Wire {
    lines: Vec<Line>,
}

fn parse(string: &str) -> Wire {
    let mut lines: Vec<Line> = vec![];
    let mut starting_point = Point { x: 0, y: 0 };
    for instruction in string.split(',') {
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
    Wire { lines }
}

pub fn star_one(input: impl BufRead) -> usize {
    let wires: Vec<Wire> = input
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .collect();
    let mut intersections = vec![];
    // TODO: Change to be not as O(N^2)
    for i in 0..wires.len() {
        for j in (i + 1)..wires.len() {
            for segment1 in &wires[i].lines {
                for segment2 in &wires[j].lines {
                    if let Some(point) = segment1.intersects(&segment2) {
                        intersections.push(point);
                    }
                }
            }
        }
    }
    // println!("Wires: {:?}", wires);
    // println!("Intersections: {:?}", intersections);
    match intersections
        .into_iter()
        .min_by_key(|a| a.x.abs() + a.y.abs())
    {
        Some(val) => {
            println!("{:?} {}", val, val.x.abs() + val.y.abs());
            (val.x.abs() + val.y.abs()) as usize
        }
        None => unreachable!(),
    }
}

pub fn star_two(input: impl BufRead) -> usize {
    let wires: Vec<Wire> = input
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .collect();
    let mut intersections = vec![];
    // TODO: Change to be not as O(N^2)
    for i in 0..wires.len() {
        for j in (i + 1)..wires.len() {
            let mut line1_current_length = 0;
            for segment1 in &wires[i].lines {
                let mut line2_current_length = 0;
                for segment2 in &wires[j].lines {
                    if let Some(point) = segment1.intersects(&segment2) {
                        intersections.push((
                            point.clone(),
                            line1_current_length + segment1.0.manhattan_distance(&point),
                            line2_current_length + segment2.0.manhattan_distance(&point),
                        ))
                    }
                    line2_current_length += segment2.length();
                }
                line1_current_length += segment1.length();
            }
        }
    }
    // println!("Wires: {:?}", wires);
    // println!("Intersections: {:?}", intersections);
    let (_point, x, y) = intersections.into_iter().min_by_key(|a| a.1 + a.2).unwrap();
    (x.abs() + y.abs()) as usize
}

#[cfg(test)]
mod tests {
    // use std::io::Cursor;

    use super::*;

    #[test]
    fn test_intersection() {
        let a = Line(Point { x: 1, y: 0 }, Point { x: 1, y: 2 });
        let b = Line(Point { x: 0, y: 1 }, Point { x: 2, y: 1 });
        assert_eq!(intersection(&a, &b), Some(Point { x: 1, y: 1 }));
    }

    #[test]
    fn test_star_one() {
        // {
        //     let input = b"R8,U5,L5,D3\nU7,R6,D4,L4";
        //     assert_eq!(star_one(Cursor::new(input)), 6);
        // }

        //         {
        //             let input = b"R75,D30,R83,U83,L12,D49,R71,U7,L72
        // U62,R66,U55,R34,D71,R55,D58,R83";
        //             assert_eq!(star_one(Cursor::new(input)), 159);
        //         }

        //         {
        //             let input = b"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        // U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        //             assert_eq!(star_one(Cursor::new(input)), 135);
        //         }
    }

    #[test]
    fn test_star_two() {
        //       {
        //         let input = b"R8,U5,L5,D3
        // U7,R6,D4,L4";
        //         assert_eq!(star_two(Cursor::new(input)), 30);
        //       }
        //       {
        //         let input = b"R75,D30,R83,U83,L12,D49,R71,U7,L72
        // U62,R66,U55,R34,D71,R55,D58,R83";
        //         assert_eq!(star_two(Cursor::new(input)), 610);
        //       }
        //       {
        //         let input = b"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        // U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        //         assert_eq!(star_two(Cursor::new(input)), 410);
        //       }
    }
}
