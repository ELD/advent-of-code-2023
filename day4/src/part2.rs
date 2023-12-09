// TODO: I know there's a dynamic programming solution that speeds up the runtime, just not sure
// how to design and implement it
use std::{
    collections::{BTreeSet, VecDeque},
    fs::read_to_string,
};

type Cards = VecDeque<(Card, usize)>;

#[derive(Clone, Debug)]
struct Card {
    winning: BTreeSet<usize>,
    mine: BTreeSet<usize>,
}

pub fn solution() -> usize {
    let input = read_to_string("day4/input.txt").unwrap();
    calculate_scratch_cards(parse_input(&input))
}

fn calculate_scratch_cards(mut cards: Cards) -> usize {
    if let Some((top_card, count)) = cards.pop_front() {
        let card_count = cards.len();
        for _ in 0..count {
            let score = score_card(&top_card);
            if score == 0 {
                break;
            }

            for num in 0..score {
                cards[num % card_count].1 += 1;
            };
        }
        let recurrence = calculate_scratch_cards(cards);
        count + recurrence
    } else {
        0
    }
}

fn score_card(card: &Card) -> usize {
    card.winning.intersection(&card.mine).count()
}

fn parse_input(input: &str) -> Cards {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let numbers = parts.nth(1).expect("malformed card input");
            let (winning, mine) = parse_numbers(numbers);
            (Card { winning, mine }, 1)
        })
        .collect::<VecDeque<(Card, usize)>>()
}

fn parse_numbers(numbers: &str) -> (BTreeSet<usize>, BTreeSet<usize>) {
    let mut parts = numbers.split('|');
    let winning = parts.next().expect("malformed card input").trim();
    let mine = parts.next().expect("malformed card input").trim();

    (
        winning
            .split_whitespace()
            .map(|n| n.parse().expect("not an ascii number"))
            .collect::<BTreeSet<_>>(),
        mine.split_whitespace()
            .map(|n| n.parse().expect("not an ascii number"))
            .collect::<BTreeSet<_>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example() {
        assert_eq!(calculate_scratch_cards(parse_input(INPUT)), 30);
    }

    #[test]
    fn with_none_winning() {
        let cards = parse_input("Card 1: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(calculate_scratch_cards(cards), 1);
    }

    #[test]
    fn with_one_winning_card() {
        let input = r"Card 1: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 2: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = parse_input(input);
        assert_eq!(calculate_scratch_cards(cards), 3);
    }
}
