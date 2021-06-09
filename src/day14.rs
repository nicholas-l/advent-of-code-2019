use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    iter::once,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, newline},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

type Statement<'a> = (Vec<(&'a str, usize)>, (&'a str, usize));
type Equations<'a> = HashMap<&'a str, (usize, &'a Vec<(&'a str, usize)>)>;

fn parse_section(s: &str) -> IResult<&str, (&str, usize)> {
    let (input, (multiplier, symbol)) = delimited(
        many0(char(' ')),
        separated_pair(digit1, tag(" "), alpha1),
        many0(char(' ')),
    )(s)?;
    let m = multiplier.parse::<usize>().unwrap();
    Ok((input, (symbol, m)))
}

fn parse_statement(s: &str) -> IResult<&str, Vec<(&str, usize)>> {
    let (input, sections) = separated_list1(tag(","), preceded(many0(tag(" ")), parse_section))(s)?;
    Ok((input, sections))
}

fn parse_line(s: &str) -> IResult<&str, Statement> {
    let (input, (left, right)) = separated_pair(parse_statement, tag("=>"), parse_statement)(s)?;
    assert_eq!(right.len(), 1);
    Ok((input, (left, right[0])))
}

fn parse_statements(s: &str) -> (Vec<&str>, Vec<Statement>) {
    let (input, statements) = separated_list1(newline, parse_line)(s).unwrap();
    assert_eq!(input.len(), 0);
    // get all possible symbols
    let symbols: HashSet<_> = statements
        .iter()
        .flat_map(|(left, right)| left.iter().chain(once(right)).map(|s| s.0))
        .collect();
    let symbols: Vec<&str> = symbols.into_iter().collect();
    (symbols, statements)
}

fn react<'a>(
    mapping: &'a Equations,
    current_state: &mut HashMap<&'a str, usize>,
    chemical: &'a str,
    amount: usize,
) -> usize {
    if chemical == "ORE" {
        return amount;
    }
    // println!("Trying to produce {} of {}", amount, chemical);
    let (output_count, inputs) = mapping.get(chemical).unwrap();
    let current_amount = *current_state.get(chemical).unwrap_or(&0);
    let amount_needed = if current_amount >= amount {
        current_state.insert(chemical, current_amount - amount);
        return 0;
    } else {
        current_state.insert(chemical, 0);
        amount - current_amount
    };
    let reacts_needed = (amount_needed as f64 / *output_count as f64).ceil() as usize;

    let ore_used = inputs
        .iter()
        .map(|(x, a)| react(mapping, current_state, x, *a * reacts_needed))
        .sum::<usize>();
    *current_state.entry(chemical).or_insert(0) += output_count * reacts_needed - amount_needed;
    // println!("{}: {} => {} ({}, {}) {:?}", chemical, &ore_used, amount, reacts_needed, output_count, current_state);
    ore_used
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _input_file = input.read_to_string(&mut buf);
    let (_symbols, statements) = parse_statements(&buf);
    let mapping = statements
        .iter()
        .map(|(left, right)| (right.0, (right.1, left)))
        .collect();
    let mut current_state = HashMap::new();
    react(&mapping, &mut current_state, "FUEL", 1)
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _input_file = input.read_to_string(&mut buf);
    let (_symbols, statements) = parse_statements(&buf);
    let mapping = statements
        .iter()
        .map(|(left, right)| (right.0, (right.1, left)))
        .collect();
    let target = 1_000_000_000_000;
    let mut lower = 0;
    let mut upper = target;
    let mut max_fuel = 0;

    // Binary search
    while lower < upper {
        let fuel = lower + (upper - lower) / 2;
        let mut current_state = HashMap::new();
        let ores = react(&mapping, &mut current_state, "FUEL", fuel);
        if ores <= target {
            max_fuel = max_fuel.max(fuel);
        }
        if ores > target {
            upper = fuel;
        } else {
            lower = fuel + 1;
        }
    }
    // dbg!(symbols);
    max_fuel
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT1: &str = "\
    10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL";

    const INPUT2: &str = "\
    9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";

    const INPUT3: &str = "\
    157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const INPUT4: &str = "\
    2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";

    const INPUT5: &str = "\
    171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_parsing() {
        let (_symbols, statements) = parse_statements(INPUT1);
        assert_eq!(
            statements,
            vec![
                (vec![("ORE", 10)], ("A", 10)),
                (vec![("ORE", 1)], ("B", 1)),
                (vec![("A", 7), ("B", 1)], ("C", 1)),
                (vec![("A", 7), ("C", 1)], ("D", 1)),
                (vec![("A", 7), ("D", 1)], ("E", 1)),
                (vec![("A", 7), ("E", 1)], ("FUEL", 1))
            ]
        );
    }

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT1)), 31);
        assert_eq!(star_one(Cursor::new(INPUT2)), 165);
        assert_eq!(star_one(Cursor::new(INPUT3)), 13312);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT3)), 82892753);
        assert_eq!(star_two(Cursor::new(INPUT4)), 5586022);
        assert_eq!(star_two(Cursor::new(INPUT5)), 460664);
    }
}
