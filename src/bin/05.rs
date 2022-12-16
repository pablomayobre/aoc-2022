use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, digit1, space0},
    combinator::{map, map_res, opt},
    error::{FromExternalError, ParseError},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

type Structure = Vec<Vec<char>>;
trait CrateStructure {
    fn top_items(&self) -> String;
}
impl CrateStructure for Structure {
    fn top_items(&self) -> String {
        self.iter()
            .map(|column| column.last().unwrap())
            .collect::<String>()
    }
}

fn transpose<T>(mut v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return Vec::new();
    }

    for inner in &mut v {
        // Reverse each row (right to left) so that when we pop we get the leftmost item first
        inner.reverse();
    }

    let size = v.iter().map(|i| i.len()).max().unwrap();

    (0..size)
        .map(|_| {
            // Iterate over the rows in reverse order (bottom to top)
            v.iter_mut()
                .rev()
                // Filter out all the None values, this removes holes and trims the Nones from the edges of the rows
                .filter_map(|inner| {
                    inner
                        // Pop one item from each row
                        .pop()
                        // If this row was shorter, or the item is a hole, we flatten it into a None
                        .flatten()
                })
                // Collect the items from each row, into columns
                .collect::<Vec<T>>()
        })
        // Then collect all the columns for the matrix
        .collect()
}

fn parse_box<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Option<char>, E> {
    alt((
        map(delimited(char('['), alpha1, char(']')), |char: &str| {
            char.chars().next()
        }),
        map(tag("   "), |_| None),
    ))
    .parse(input)
}

fn parse_structure<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Structure, E> {
    map(
        separated_list0(char('\n'), many1(preceded(opt(char(' ')), parse_box))),
        |structure| transpose(structure),
    )
    .parse(input)
}

#[derive(Debug)]
struct Instruction {
    crates: usize,
    from: usize,
    to: usize,
}

fn parse_number<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, usize, E> {
    map_res(digit1, |n: &'a str| n.parse::<usize>()).parse(input)
}

fn parse_instruction<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, Instruction, E> {
    map(
        tuple((
            preceded(tuple((tag("move"), space0)), parse_number),
            preceded(tuple((space0, tag("from"), space0)), parse_number),
            preceded(tuple((space0, tag("to"), space0)), parse_number),
        )),
        |(crates, from, to)| Instruction {
            crates,
            from: from - 1,
            to: to - 1,
        },
    )
    .parse(input)
}

fn parse_input<'a, E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>>(
    input: &'a str,
) -> IResult<&'a str, (Structure, Vec<Instruction>), E> {
    tuple((
        parse_structure,
        preceded(
            take_until("move"),
            separated_list0(char('\n'), parse_instruction),
        ),
    ))
    .parse(input)
}

trait CrateInstuction {
    type Error;
    fn execute(&self, crates: Structure) -> Result<Structure, Self::Error>;
    fn better_execute(&self, crates: Structure) -> Result<Structure, Self::Error>;
}

impl CrateInstuction for Instruction {
    type Error = ();

    fn execute(&self, mut crates: Structure) -> Result<Structure, Self::Error> {
        if self.from > crates.len() {
            return Err(());
        }
        if self.to > crates.len() {
            return Err(());
        }

        for _i in 0..self.crates {
            match crates[self.from].pop() {
                Some(popped) => crates[self.to].push(popped),
                None => return Err(()),
            }
        }

        Ok(crates)
    }

    fn better_execute(&self, mut crates: Structure) -> Result<Structure, Self::Error> {
        if self.from > crates.len() {
            return Err(());
        }
        if self.to > crates.len() {
            return Err(());
        }

        let len = crates[self.from].len() - self.crates;
        let items = crates[self.from].split_off(len);

        for i in items.iter() {
            crates[self.to].push(*i)
        }

        Ok(crates)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, (structure, instructions)) = parse_input::<()>.parse(input).unwrap();

    let s = instructions
        .iter()
        .fold(structure, |st, inst| inst.execute(st).unwrap());

    Some(s.top_items())
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, (structure, instructions)) = parse_input::<()>.parse(input).unwrap();

    let s = instructions
        .iter()
        .fold(structure, |st, inst| inst.better_execute(st).unwrap());

    Some(s.top_items())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
