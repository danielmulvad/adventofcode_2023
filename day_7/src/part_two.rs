/*
--- Part Two ---
To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
KK677 is now the only two pair, making it the second-weakest hand.
T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/
use std::{ops::Deref, str::FromStr};

use itertools::{Itertools, Position};
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Clone, Copy, Debug)]
struct Hand {
    bid: u32,
    cards: (u32, u32, u32, u32, u32),
    hand_type: HandType,
}

impl Hand {
    fn new(bid: u32, cards: (u32, u32, u32, u32, u32), hand_type: HandType) -> Self {
        Self {
            hand_type,
            cards,
            bid,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();

        let counts = hand.chars().counts();
        let values = if let Some(joker_count) = counts.get(&'J') {
            if *joker_count == 5 {
                "5".to_string()
            } else {
                counts
                    .iter()
                    .filter_map(|(key, value)| (key != &'J').then_some(value))
                    .sorted()
                    .with_position()
                    .map(|(position, value)| match position {
                        Position::Last | Position::Only => value + joker_count,
                        _ => *value,
                    })
                    .join("")
            }
        } else {
            counts.values().sorted().join("")
        };

        let hand_type = match values.deref() {
            "5" => HandType::FiveOfAKind,
            "14" => HandType::FourOfAKind,
            "23" => HandType::FullHouse,
            "113" => HandType::ThreeOfAKind,
            "122" => HandType::TwoPair,
            "1112" => HandType::OnePair,
            "11111" => HandType::HighCard,
            _ => unreachable!(),
        };
        let cards =
            hand.chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    value => value.to_digit(10).unwrap(),
                })
                .collect_tuple()
                .unwrap();

        Ok(Self::new(bid, cards, hand_type))
    }
}

#[derive(Debug)]
pub struct PartTwo {
    hands: Vec<Hand>,
}

impl PartTwo {
    pub fn new(input: &str) -> Self {
        let hands = input
            .lines()
            .par_bridge()
            .map(|line| line.parse::<Hand>().unwrap())
            .collect();
        Self { hands }
    }

    #[tracing::instrument(skip(self))]
    pub fn run(&self) -> u32 {
        let winnings = self
            .hands
            .iter()
            .sorted_by_key(|hand| (hand.hand_type, hand.cards))
            .enumerate()
            .par_bridge()
            .map(|(index, hand)| (index as u32 + 1) * hand.bid)
            .sum();
        winnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test_data.txt");
        let output = PartTwo::new(input).run();
        assert_eq!(output, 5905);
    }
}
