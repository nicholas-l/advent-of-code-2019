use nom::character::complete::anychar;
use nom::character::complete::char;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::delimited,
    sequence::tuple,
    IResult,
};
use num::Integer;
use std::io::BufRead;
use std::ops::Add;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

fn parse_position(s: &str) -> IResult<&str, Position> {
    let parse_decimal_str = recognize(tuple((opt(tag("-")), many1(one_of("0123456789")))));
    let decimal = map_res(parse_decimal_str, |s: &str| s.parse::<isize>());
    let coord = separated_pair(anychar, char('='), decimal);
    let coords = separated_list1(tag(", "), coord);
    let (_rest, data) = delimited(char('<'), coords, char('>'))(s)?;
    let (_k, x) = data[0];
    let (_k, y) = data[1];
    let (_k, z) = data[2];
    Ok(("", Position { x, y, z }))
}

impl Position {
    fn get_velocity_delta(&self, other: &Position) -> Velocity {
        let x = match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let y = match self.y.cmp(&other.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let z = match self.z.cmp(&other.z) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        Velocity { x, y, z }
    }

    fn energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

impl Velocity {
    fn energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<'a, 'b> Add<&'a Velocity> for &'b Velocity {
    type Output = Velocity;

    fn add(self, rhs: &Velocity) -> Self::Output {
        Velocity {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Velocity> for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Self) -> Self::Output {
        Velocity {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add<&Velocity> for &Position {
    type Output = Position;

    fn add(self, rhs: &Velocity) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct System {
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
}

impl System {
    fn step(&mut self) {
        self.velocities =
            self.positions
                .iter()
                .zip(self.velocities.iter())
                .enumerate()
                .map(|(i, (position, velocity))| {
                    let v = self.positions.iter().enumerate().fold(
                        Velocity::default(),
                        |x, (j, other)| {
                            if i != j {
                                let velocity_delta = position.get_velocity_delta(&other);
                                x + velocity_delta
                            } else {
                                x
                            }
                        },
                    );
                    &v + velocity
                })
                .collect();
        self.positions = self
            .positions
            .iter()
            .zip(self.velocities.iter())
            .map(|(position, velocity)| position + velocity)
            .collect();
    }

    fn energy(&self) -> isize {
        self.positions
            .iter()
            .zip(self.velocities.iter())
            .map(|(position, velocity)| position.energy() * velocity.energy())
            .sum()
    }
}

impl FromStr for System {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<Position> = s
            .lines()
            .map(|line| {
                // println!("{}", &v);
                let (_, position) = parse_position(line.trim()).unwrap();
                position
            })
            .collect();
        let velocities = positions
            .iter()
            .map(|_x| Velocity { x: 0, y: 0, z: 0 })
            .collect();
        Ok(System {
            positions,
            velocities,
        })
    }
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_text = String::new();
    input.read_to_string(&mut input_text).unwrap();
    let mut system = input_text.parse::<System>().unwrap();
    for _x in 0..1000 {
        system.step();
    }
    system.energy() as usize
}

fn get_axis(velocity: &Velocity, axis: usize) -> isize {
    match axis {
        0 => velocity.x,
        1 => velocity.y,
        2 => velocity.z,
        _ => panic!("Invlaid axis"),
    }
}

fn get_axis_position(position: &Position, axis: usize) -> isize {
    match axis {
        0 => position.x,
        1 => position.y,
        2 => position.z,
        _ => panic!("Invlaid axis"),
    }
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_text = String::new();
    input.read_to_string(&mut input_text).unwrap();
    let mut system = input_text.parse::<System>().unwrap();
    let starting_system = system.clone();
    // Period of complete change of each axis.
    let mut periods = vec![None; 3];
    let mut steps = 0;
    while periods.iter().any(|x| x.is_none()) {
        system.step();
        steps += 1;

        for (i, p) in periods.iter_mut().enumerate() {
            if p.is_none() // We havent found the period of this axis.
            // is all velocities in axis equal to 0
            && system.velocities.iter().all(|v| get_axis(v, i) == 0)
            && system.positions
            .iter()
            .zip(starting_system.positions.iter())
            .all(|(position, starting_position)| get_axis_position(position, i) == get_axis_position(starting_position,i))
            {
                p.replace(steps);
            }
        }
    }

    let first = periods[0].unwrap();
    periods
        .into_iter()
        .fold(first, |lcm, p| lcm.lcm(&p.unwrap()))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const INPUT: &str = "\
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    const INPUT2: &str = "\
    <x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    #[test]
    fn test_parse_position() {
        let input = "<x=-1, y=0, z=2>";
        let expected = Ok(("", Position { x: -1, y: 0, z: 2 }));
        assert_eq!(parse_position(input), expected);
    }

    #[test]
    fn test_velocity_delta() {
        let ganymede = Position { x: 3, y: 0, z: 0 };
        let callisto = Position { x: 5, y: 0, z: 0 };
        let expected = Velocity { x: 1, y: 0, z: 0 };
        assert_eq!(ganymede.get_velocity_delta(&callisto), expected);
    }

    #[test]
    fn test_star_one_steps() {
        let mut system = INPUT.parse::<System>().unwrap();

        assert_eq!(system.positions[0], Position { x: -1, y: 0, z: 2 },);
        assert_eq!(system.velocities[0], Velocity { x: 0, y: 0, z: 0 },);

        system.step();
        // let expected = Moon {
        //     position: Position { x: 2, y: -1, z: 1 },
        //     velocity: Velocity { x: 3, y: -1, z: -1 },
        // };
        // assert_eq!(system.moons[0], expected);

        // system.step();
        // let expected = Moon {
        //     position: Position { x: 5, y: -3, z: -1 },
        //     velocity: Velocity { x: 3, y: -2, z: -2 },
        // };
        // assert_eq!(system.moons[0], expected);

        // system.step();
        // let expected = Moon {
        //     position: Position { x: 5, y: -6, z: -1 },
        //     velocity: Velocity { x: 0, y: -3, z: 0 },
        // };
        // assert_eq!(system.moons[0], expected);
    }

    #[test]
    fn test_star_one_10_steps_end() {
        let mut system = INPUT.parse::<System>().unwrap();
        for _x in 0..10 {
            system.step();
        }

        assert_eq!(system.positions[0], Position { x: 2, y: 1, z: -3 },);
        assert_eq!(system.velocities[0], Velocity { x: -3, y: -2, z: 1 });

        // assert_eq!(
        //     system.moons[1],
        //     Moon {
        //         position: Position { x: 1, y: -8, z: 0 },
        //         velocity: Velocity { x: -1, y: 1, z: 3 }
        //     }
        // );
        // assert_eq!(system.moons[0].energy(), 36);
        assert_eq!(system.energy(), 179);
    }

    #[ignore = "reason"]
    #[test]
    fn test_star_one_step_end() {
        let mut system = INPUT2.parse::<System>().unwrap();
        let mut expected = vec![
            (
                90,
                Position {
                    x: -25,
                    y: -1,
                    z: 4,
                },
            ),
            // (80, Position { x: 30, y: -8, z: 3 }),
        ];
        let mut current_i = 0;
        while let Some((i, e)) = expected.pop() {
            while current_i < i {
                current_i += 1;
                system.step();
            }
            assert_eq!(system.positions[0], e);
        }
        while current_i < 100 {
            current_i += 1;
            system.step();
        }

        assert_eq!(
            system.positions[0],
            Position {
                x: 8,
                y: -12,
                z: -9
            }
        );

        // assert_eq!(system.moons[0].energy(), 290);
        assert_eq!(system.energy(), 1940);
    }

    #[test]
    fn test_star_one() {}

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 2772);
    }
}
