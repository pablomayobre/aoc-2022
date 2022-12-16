use std::ops::BitAnd;

use fixedbitset::FixedBitSet;
use itertools::Itertools;

fn get_priority(ch: &char) -> usize {
    // This function calculates the priority for a given character
    // a to z get priorities 0 to 25
    // A to Z get priorities 26 to 51
    match ch {
        'a'..='z' => (ch.clone() as usize) - ('a' as usize) + 0,
        'A'..='Z' => (ch.clone() as usize) - ('A' as usize) + 26,
        _ => 0,
    }
}

// Turns a string into a Set that will store which items are present in a rucksack
fn rucksack_hashset(rucksack: &str) -> FixedBitSet {
    // We are using a FixedBitSet for efficiency, we could use a HashSet
    rucksack
        .chars()
        .fold(FixedBitSet::with_capacity(52), |mut bitset, value| {
            // We are gonna grab the priority for each item in the sack and set the corresponding bit in the set
            bitset.insert(get_priority(&value));
            bitset
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split('\n')
        .filter_map(|rucksacks| -> Option<(&str, &str)> {
            // Split sacks in the middle, discard the ones that are not even
            match rucksacks.len() % 2 {
                0 => Some(rucksacks.split_at(rucksacks.len() / 2)),
                _ => None,
            }
        })
        // Convert the sacks into sets
        .map(|(left, right)| (rucksack_hashset(left), rucksack_hashset(right)))
        .map(|(left, right)| -> u32 {
            // Find the intersection of the sets
            let intersection = left.intersection(&right);
            // Add up the priorities of the items in that intersection
            intersection.fold(0, |prev, priority| prev + priority as u32 + 1)
        })
        // Return the total sum of the priorities
        .reduce(|prev, next| prev + next)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split('\n')
        .map(|rucksack| rucksack_hashset(rucksack))
        // Make groups of 3 sacks each
        .tuples::<(FixedBitSet, FixedBitSet, FixedBitSet)>()
        .map(|(a, b, c)| -> u32 {
            // Calculate the intersection of the sets for each one of the 3 sacks
            let badge = a.bitand(&b).bitand(&c);

            // Add up the priorities of the items in that intersection
            badge
                .ones()
                .fold(0, |prev, priority| prev + priority as u32 + 1)
        })
        // Return the total sum of the priorities
        .reduce(|prev, next| prev + next)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
