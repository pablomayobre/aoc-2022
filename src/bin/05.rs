use std::str::FromStr;

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
    // Our crate structure has a method to get the box at the top of each stack
    fn top_items(&self) -> String {
        self.iter() // For that we need to iterate through the stacks
            .map(|column| column.last().unwrap()) // Get the last box and unwrap it into a char
            .collect::<String>() // Then we can collect all the chars into a String
    }
}

// Transpose our parsed structure that contains holes and is in rows instead of columns into the desired shape (list of stacks/columns)
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

// This function can parse a single box with a character inside (Some(char)) or a hole in the structure (None)
fn parse_box<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Option<char>, E> {
    alt((
        map(delimited(char('['), alpha1, char(']')), |char: &str| {
            char.chars().next()
        }),
        map(tag("   "), |_| None),
    ))
    .parse(input)
}

// This function parses all the consecutive lines that contain valid structure pieces (Boxes or Holes)
fn parse_structure<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Structure, E> {
    map(
        separated_list0(char('\n'), many1(preceded(opt(char(' ')), parse_box))),
        |structure| transpose(structure), // The parsed structure is transposed so we transpose it back
    )
    .parse(input)
}

#[derive(Debug)]
struct Instruction {
    crates: usize,
    from: usize,
    to: usize,
}

// Helper function to parse digit characters into any type of number
fn parse_number<'a, F: FromStr, E: ParseError<&'a str> + FromExternalError<&'a str, F::Err>>(
    input: &'a str,
) -> IResult<&'a str, F, E> {
    map_res(digit1, |n: &'a str| n.parse::<F>()).parse(input)
}

// This function can parse instructions in the shape "move X from Y to Z" into an Instruction defined above
fn parse_instruction<
    'a,
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, Instruction, E> {
    map(
        tuple((
            preceded(tuple((tag("move"), space0)), parse_number::<usize, E>),
            preceded(
                tuple((space0, tag("from"), space0)),
                parse_number::<usize, E>,
            ),
            preceded(tuple((space0, tag("to"), space0)), parse_number::<usize, E>),
        )),
        |(crates, from, to)| Instruction {
            crates,
            from: from - 1,
            to: to - 1,
        },
    )
    .parse(input)
}

// Parses the entire structure and set of instructions for a given file
fn parse_input<'a, E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>>(
    input: &'a str,
) -> IResult<&'a str, (Structure, Vec<Instruction>), E> {
    tuple((
        parse_structure, // The file contains the structure
        preceded(
            take_until("move"), // Then a bunch of new lines and numbers we can ignore until we find the start of an instruction
            separated_list0(char('\n'), parse_instruction), // From there we parse all the available instructions
        ),
    ))
    .parse(input)
}

// Each instruction can be executed by either CrateMover 9000 or 9001
trait CrateInstuction {
    type Error;
    fn cratemover9000(&self, crates: Structure) -> Result<Structure, Self::Error>;
    fn cratemover9001(&self, crates: Structure) -> Result<Structure, Self::Error>;
}

impl CrateInstuction for Instruction {
    type Error = ();

    fn cratemover9000(&self, mut crates: Structure) -> Result<Structure, Self::Error> {
        // If from is out of range return an error
        if self.from > crates.len() {
            return Err(());
        }
        // If to is out of range return an error
        if self.to > crates.len() {
            return Err(());
        }

        // For each one of the crates we need to move
        for _i in 0..self.crates {
            // We pop one crate from the "from" stack
            match crates[self.from].pop() {
                Some(popped) => crates[self.to].push(popped), // If there was one, we push it to the "to" stack
                None => return Err(()),
            }
        }

        Ok(crates)
    }

    fn cratemover9001(&self, mut crates: Structure) -> Result<Structure, Self::Error> {
        // If from is out of range return an error
        if self.from > crates.len() {
            return Err(());
        }
        // If to is out of range return an error
        if self.to > crates.len() {
            return Err(());
        }

        // First calculate the new size of the "from" stack
        let len = crates[self.from].len() - self.crates;
        // Split-off the top of the "from" stack
        let items = crates[self.from].split_off(len);

        // For each of the items we just split off
        for i in items.iter() {
            crates[self.to].push(*i) // We push it back to the "to" stack
        }

        Ok(crates)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    // First we parse our file
    let (_, (structure, instructions)) = parse_input::<()>.parse(input).unwrap();

    // Then we iterate over the instruction
    let s = instructions
        .iter()
        // We execute each instruction with our CrateMover 9000 and fold over the resulting structure
        .fold(structure, |st, inst| inst.cratemover9000(st).unwrap());

    // We get the top items from the resulting structure
    Some(s.top_items())
}

pub fn part_two(input: &str) -> Option<String> {
    // First we parse our file
    let (_, (structure, instructions)) = parse_input::<()>.parse(input).unwrap();

    // Then we iterate over the instruction
    let s = instructions
        .iter()
        // We execute each instruction with our CrateMover 9001 and fold over the resulting structure
        .fold(structure, |st, inst| inst.cratemover9001(st).unwrap());

    // We get the top items from the resulting structure
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
