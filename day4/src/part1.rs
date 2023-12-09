use std::{collections::HashSet, fs::read_to_string};

struct Card {
    winning: HashSet<usize>,
    mine: HashSet<usize>,
}

pub fn solution() -> usize {
    let input = read_to_string("day4/input.txt").unwrap();
    calculate_points(&parse_input(&input))
}

fn calculate_points(cards: &[Card]) -> usize {
    cards.iter().map(|card| {
        let intersect = card.winning.intersection(&card.mine).count();
        if intersect < 1 {
            return 0;
        }
        1 << (intersect - 1)
    }).sum()
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let numbers = parts.nth(1).expect("malformed card input");
            let (winning, mine) = parse_numbers(numbers);
            Card { winning, mine }
        })
        .collect()
}

fn parse_numbers(numbers: &str) -> (HashSet<usize>, HashSet<usize>) {
    let mut parts = numbers.split('|');
    let winning = parts.next().expect("malformed card input").trim();
    let mine = parts.next().expect("malformed card input").trim();

    (
        winning
            .split_whitespace()
            .map(|n| n.parse().expect("not an ascii number"))
            .collect::<HashSet<_>>(),
        mine.split_whitespace()
            .map(|n| n.parse().expect("not an ascii number"))
            .collect::<HashSet<_>>(),
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
        assert_eq!(calculate_points(&parse_input(INPUT)), 13);
    }
}
