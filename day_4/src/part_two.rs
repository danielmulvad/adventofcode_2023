/*
--- Part Two ---
Just as you're about to report your findings to the Elf, one of you realizes that the rules have actually been printed on the back of every card this whole time.

There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.

Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

This time, the above example goes differently:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
Your copy of card 2 also wins one copy each of cards 3 and 4.
Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!

Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?
*/
#[derive(Debug)]
struct Card {
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    pub fn new(numbers: Vec<u32>, winning_numbers: Vec<u32>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }
}

pub struct PartTwo {
    cards: Vec<Card>,
}

impl PartTwo {
    fn parse_side(side: &str) -> Vec<u32> {
        let mut numbers: Vec<u32> = Vec::new();
        let split_side = side.split(" ").collect::<Vec<&str>>();
        for number in split_side {
            let number = number.trim();
            if number.is_empty() {
                continue;
            }
            numbers.push(number.parse::<u32>().expect("Not a number"));
        }
        numbers
    }

    fn parse_lines(lines: &Vec<String>) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        for line in lines {
            // Split by colon to get the right side
            let split_line = line.split(":").collect::<Vec<&str>>();
            let right_side = split_line.get(1).expect("No right side").trim();
            // Split by pipe to get the winning numbers and the numbers
            let split_right_side = right_side.split("|").collect::<Vec<&str>>();
            let left_side = split_right_side.get(0).expect("No left side");
            let right_side = split_right_side.get(1).expect("No right side");

            let numbers = Self::parse_side(right_side);
            let winning_numbers = Self::parse_side(left_side);
            cards.push(Card::new(numbers, winning_numbers));
        }
        cards
    }

    pub fn new(lines: &Vec<String>) -> Self {
        let cards = Self::parse_lines(lines);
        Self { cards }
    }

    pub fn run(&self) -> u32 {
        let mut scratchcard_counts = vec![1; self.cards.len()]; // Initialize with 1 for each original card

        for (index, card) in self.cards.iter().enumerate() {
            let mut new_wins = vec![0; self.cards.len()];

            for _ in 0..scratchcard_counts[index] {
                let matches = card
                    .winning_numbers
                    .iter()
                    .filter(|&n| card.numbers.contains(n))
                    .count();
                for win_index in index + 1..index + 1 + matches {
                    if win_index < self.cards.len() {
                        new_wins[win_index] += 1;
                    }
                }
            }

            for (i, &wins) in new_wins.iter().enumerate() {
                scratchcard_counts[i] += wins;
            }
        }

        scratchcard_counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_reader;

    use super::*;
    use std::io::BufRead;

    #[test]
    fn test_part_two() {
        let buf_reader = file_reader("./test_data.txt");
        let lines: Vec<String> = buf_reader.lines().map(|l| l.unwrap_or_default()).collect();
        let result = PartTwo::new(&lines).run();

        assert_eq!(result, 30);
    }
}
