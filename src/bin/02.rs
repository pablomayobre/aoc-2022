use itertools::Itertools;

// First we define the hands each player can play
#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

// We use the TryFrom crate to convert from a string to a hand
impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            // We need to handle the cases for each player
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

// We also define a way to convert back and from numbers
impl TryFrom<u32> for Hand {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        // Using the module allows us to wrap around
        //Add 1 to get the losing move, add 2 to get the winning move
        match v % 3 {
            0 => Ok(Hand::Rock),
            1 => Ok(Hand::Paper),
            2 => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

// We then need the results for a match
#[derive(PartialEq, Clone, Copy)]
enum MatchResult {
    Lose,
    Draw,
    Win,
}

// For part 2 we need a way to convert from a string to a hand
impl TryFrom<&str> for MatchResult {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "X" => Ok(MatchResult::Lose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(()),
        }
    }
}

// This function calculates the match result given the hand for both players
fn get_match_result(oponent: Hand, own: Hand) -> MatchResult {
    if oponent == own {
        // If the hands are the same, it's a draw
        return MatchResult::Draw;
    }

    match (oponent, own) {
        // All the cases where own wins
        (Hand::Paper, Hand::Scissors)
        | (Hand::Scissors, Hand::Rock)
        | (Hand::Rock, Hand::Paper) => MatchResult::Win,
        // else own loses
        _ => MatchResult::Lose,
    }
}

// This function calculates what movement you need to perform to get a given match result
fn get_hand(oponent: Hand, result: MatchResult) -> Hand {
    match result {
        // In a draw you need to perform the same hand
        MatchResult::Draw => oponent,
        // In a win, you need to perform the winning move, to get this we add one to the current hand
        MatchResult::Win => Hand::try_from(oponent as u32 + 1).unwrap(),
        // In a lose, you need to perform the losing move, to get this we add two to the current hand
        MatchResult::Lose => Hand::try_from(oponent as u32 + 2).unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
        // Split the input into lines, each representing a match
            .split('\n')
            .filter_map(|play| -> Option<(Hand, Hand)> {
                // Each line then contains the hand for each of the players
                match play.split(' ').next_tuple() {
                    Some((a, b)) => Some((a.try_into().unwrap(), b.try_into().unwrap())),
                    None => None,
                }
            })
            .map(|(oponent, own)| -> u32 {
                // We then calculate the result of the match and add up the points
                // result * 3 + hand + 1
                get_match_result(oponent, own) as u32 * 3 + own as u32 + 1
            })
            .fold(0, |prev, next| prev + next),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            // Same as above but now we get the MatchResult as the second parameter
            .filter_map(|play| -> Option<(Hand, MatchResult)> {
                match play.split(' ').next_tuple() {
                    Some((a, b)) => Some((a.try_into().unwrap(), b.try_into().unwrap())),
                    None => None,
                }
            })
            .map(|(oponent, result)| -> u32 {
                // And calculate the hand instead of the match result
                result as u32 * 3 + get_hand(oponent, result) as u32 + 1
            })
            .fold(0, |prev, next| prev + next),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
