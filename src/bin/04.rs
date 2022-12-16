use std::num::ParseIntError;

use itertools::{process_results, Itertools};
use ranges::{GenericRange, Relation};

enum RangeParseError {
    LeftSideParsing(ParseIntError),
    RightSideParsing(ParseIntError),
    InvalidRange,
}

// Turn a string like "4-5" into a closed range [4..=5]
fn get_range(ids: &str) -> Result<GenericRange<u32>, RangeParseError> {
    // Split the string and for each tuple parse the number
    match ids.split('-').next_tuple::<(&str, &str)>() {
        Some((left, right)) => Ok(GenericRange::new_closed(
            left // Parse the left side number, and report the error if it fails
                .parse::<u32>()
                .or_else(|err| Err(RangeParseError::LeftSideParsing(err)))?,
            right // Same here but for the right side
                .parse::<u32>()
                .or_else(|err| Err(RangeParseError::RightSideParsing(err)))?,
        )),
        // If we couldn't split it or didn't find anything in the string report as invalid
        _ => Err(RangeParseError::InvalidRange),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    process_results(
        // Split the input into lines
        input
            .split('\n')
            // Each line then contains a pair of ranges separated by a comma
            .filter_map(|pair| pair.split(',').next_tuple::<(&str, &str)>())
            // We then parse each range
            .map(
                |(left, right)| -> Result<(GenericRange<u32>, GenericRange<u32>), RangeParseError> {
                    Ok((get_range(left)?, get_range(right)?))
                },
            ),
        |ranges| {
            // Once we have our pairs of ranges, we see if the overlap
            ranges
                .map(
                    |(left, right): (GenericRange<u32>, GenericRange<u32>)| -> bool {
                        matches!(
                            left.relation(right),
                            // These are the four cases that we can consider a total overlap
                            Relation::Containing { .. }
                                | Relation::Starting { .. }
                                | Relation::Ending { .. }
                                | Relation::Equal(_)
                        )
                    },
                )
                // We filter out those that don't overlap
                .filter(|value| *value)
                // And return the count of overlapping pairs
                .count() as u32
        },
    )
    .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    process_results(
        input
            .split('\n')
            .filter_map(|pair| pair.split(',').next_tuple::<(&str, &str)>())
            .map(
                |(left, right)| -> Result<(GenericRange<u32>, GenericRange<u32>), RangeParseError> {
                    Ok((get_range(left)?, get_range(right)?))
                },
            ),
        |iter| {
            iter.map(
                |(left, right): (GenericRange<u32>, GenericRange<u32>)| -> bool {
                    matches!(
                        left.relation(right),
                        Relation::Containing { .. }
                            | Relation::Starting { .. }
                            | Relation::Ending { .. }
                            | Relation::Equal(_)
                            // The same as before but now we want to include ranges that overlap partially
                            | Relation::Overlapping { .. }
                    )
                },
            )
            .filter(|value| *value)
            .count() as u32
        },
    )
    .ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
