use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // Split input by Elves marked by two new line characters
    let value = Regex::new(r"\n\s*\n")
        .unwrap()
        .split(input)
        // Each Elf contains a series of numbers split by newline characters
        .map(|elf| -> u32 {
            // Add the numbers together to get the calories intake
            elf.split('\n').fold(0, |before, next| -> u32 {
                before + next.parse::<u32>().unwrap_or(0)
            })
        })
        // Sort by how many calories they have consumed
        .sorted_by(|a, b| Ord::cmp(b, a))
        // Find the Elf with the largest calories intake
        .next();

    value
}

pub fn part_two(input: &str) -> Option<u32> {
    // Split input by Elves marked by two new line characters
    let mut iter = Regex::new(r"\n\s*\n")
        .unwrap()
        .split(input)
        // Each Elf contains a series of numbers split by newline characters
        .map(|elf| -> u32 {
            // Add the numbers together to get the calories intake
            elf.split('\n').fold(0, |before, next| -> u32 {
                before + next.parse::<u32>().unwrap_or(0)
            })
        })
        // Sort by how many calories they have consumed
        .sorted_by(|a, b| Ord::cmp(b, a));

    // Find the 3 Elves with the largest calories intake and add it up
    Some(iter.next().unwrap() + iter.next().unwrap() + iter.next().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
