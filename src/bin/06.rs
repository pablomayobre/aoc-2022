use nom::{
    bytes::complete::take,
    combinator::{map, recognize, verify},
    error::ParseError,
    multi::many_till,
    Parser,
};

fn has_repeated_char(s: &str) -> bool {
    // Loop through all the characters in a string
    s.chars()
        .enumerate()
        .find_map(|(i, c)| {
            // For each one check if the character appears again after itself
            s.chars()
                .enumerate()
                .skip(i + 1) // We can skip characters before it because we already checked those
                .find(|(_, other)| c == *other)
        })
        // Check if any character was found more than once
        .is_some()
}

fn find_unique_sequence<'a, E: ParseError<&'a str>>(size: usize) -> impl Parser<&'a str, usize, E> {
    move |input: &'a str| {
        map(
            // Consume one character until we find a sequence with the specified size that has no repeated characters
            recognize(many_till(
                take(1usize),
                verify(take(size), |value| !has_repeated_char(value)),
            )),
            // When found, we care about the position of the last character consumed
            |s: &'a str| -> usize { s.len() },
        )
        .parse(input)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // Start of Packet is a sequence of 4 unique characters
    let mut start_of_packet = find_unique_sequence::<()>(4);

    match start_of_packet.parse(input) {
        Ok((_tail, result)) => return Some(result as u32),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // Start of Message is a sequence of 14 unique characters
    let mut start_of_message = find_unique_sequence::<()>(14);

    match start_of_message.parse(input) {
        Ok((_tail, result)) => return Some(result as u32),
        _ => None,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
