use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("hand has cards and bid");
            let bid = bid.parse::<usize>().expect("bid is a valid number");
            let cards = cards
                .chars()
                .map(|c| Card::from(c).expect("card is valid"))
                .collect::<Vec<_>>();
            let kind = HandKind::parse(&cards);
            Hand { cards, bid, kind }
        })
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from(c: char) -> Option<Card> {
        match c {
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'T' => Some(Self::Ten),
            'J' => Some(Self::Jack),
            'Q' => Some(Self::Queen),
            'K' => Some(Self::King),
            'A' => Some(Self::Ace),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandKind {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandKind {
    fn parse(cards: &Vec<Card>) -> HandKind {
        let mut card_map = BTreeMap::<Card, usize>::new();
        cards.iter().for_each(|card| {
            card_map
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        card_map
            .values()
            .fold(HandKind::HighCard, |kind, card_count| {
                match (card_count, kind) {
                    (5, _) | (_, HandKind::Five) => HandKind::Five,
                    (4, _) | (_, HandKind::Four) => HandKind::Four,
                    (3, HandKind::Pair) | (2, HandKind::Three) | (_, HandKind::FullHouse) => {
                        HandKind::FullHouse
                    }
                    (3, _) | (_, HandKind::Three) => HandKind::Three,
                    (2, HandKind::Pair) | (_, HandKind::TwoPair) => HandKind::TwoPair,
                    (2, _) | (_, HandKind::Pair) => HandKind::Pair,
                    _ => HandKind::HighCard,
                }
            })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    kind: HandKind,
    cards: Vec<Card>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 6440)
    }
}
