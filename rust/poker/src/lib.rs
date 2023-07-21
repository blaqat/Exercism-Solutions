use std::{cmp::Ordering, collections::HashMap};

type Hand = Vec<Card>;

// Macro to create a hand from a string of cards or a list of card literals.
macro_rules! hand {
    ($hand_string:expr) => {
       $hand_string.split_whitespace().map(Card::from).collect::<Hand>()
    };
    ($($card:literal),+) => {
        vec![$(Cards::from($card),)+]
    };
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winners = vec![];
    hands.iter().fold(&"_", |prev, curr: &&str| {
        let (mut hand_1, mut hand_2) = (hand![prev], hand![curr]);

        hand_1.sort();
        hand_2.sort();

        match hand_1.cmp(&hand_2) {
            Ordering::Less => {
                winners.clear();
                winners.push(*curr);
                curr
            }
            Ordering::Equal => {
                winners.push(*curr);
                curr
            }
            Ordering::Greater => {
                winners.clear();
                winners.push(*prev);
                prev
            }
        }
    });
    winners
}

#[derive(PartialEq, Eq, Clone)]
struct Card {
    value: i32,
    card_type: CardTypes,
}
const EMPTY_CARD: Card = Card {
    value: 0,
    card_type: CardTypes::None,
};

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut value = value.chars().rev();
        let (card_type, card) = (value.next().unwrap(), value.rev().collect::<String>());

        let card_type = match card_type {
            'S' => CardTypes::Spades,
            'H' => CardTypes::Hearts,
            'D' => CardTypes::Diamonds,
            'C' => CardTypes::Clovers,
            _ => CardTypes::None,
        };

        let value = match card.as_str() {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            c if c.parse::<i32>().is_ok() => c.parse::<i32>().unwrap(),
            _ => 0,
        };

        Self { value, card_type }
    }
}

// Definition of card types: Spades, Hearts, Diamonds, Clovers
#[derive(PartialEq, Eq, Clone, Copy)]
enum CardTypes {
    Spades,
    Hearts,
    Diamonds,
    Clovers,
    None,
}

// Definition of poker hand rankings
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rankings {
    HighCard(i32),
    Pair(i32),
    TwoPairs(i32),
    ThreeOfAKind(i32),
    Straight(i32),
    Flush,
    FullHouse(i32),
    FourOfAKind(i32),
    StraightFlush(i32),
}

impl Rankings {
    //Ranks the Poker Rankings numerically for easy comparisons keeping track of the highest card
    fn rank(&self) -> (i32, i32) {
        match self {
            Rankings::StraightFlush(n) => (9, *n),
            Rankings::FourOfAKind(n) => (8, *n),
            Rankings::FullHouse(n) => (7, *n),
            Rankings::Flush => (6, 0),
            Rankings::Straight(n) => (5, *n),
            Rankings::ThreeOfAKind(n) => (4, *n),
            Rankings::TwoPairs(n) => (3, *n),
            Rankings::Pair(n) => (2, *n),
            Rankings::HighCard(n) => (1, *n),
        }
    }
}

// PartialOrd implementation for Rankings, allowing Rankings to be compared
impl PartialOrd for Rankings {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

// Implement conversion from a Hand to a Ranking
impl From<&Hand> for Rankings {
    fn from(hand: &Hand) -> Self {
        let mut hand = hand.clone();

        match (hand.is_flush(), hand.is_straight(), hand.get_pairs()) {
            (true, true, _) => Rankings::StraightFlush(hand[0].value),
            (false, false, pairs) if pairs[0].1 == 4 => Rankings::FourOfAKind(pairs[0].0),
            (false, false, pairs) if pairs[0].1 == 3 && pairs[1].1 == 2 => {
                Rankings::FullHouse(pairs[0].1)
            }
            (true, false, _) => Rankings::Flush,
            (false, true, _) => Rankings::Straight(hand[0].value),
            (false, false, pairs) if pairs[0].1 == 3 => Rankings::ThreeOfAKind(pairs[0].0),
            (false, false, pairs) if pairs[0].1 == 2 && pairs[1].1 == 2 => {
                Rankings::TwoPairs(pairs[0].0)
            }
            (false, false, pairs) if pairs[0].1 == 2 => Rankings::Pair(pairs[0].0),
            (_, _, _) => Rankings::HighCard(hand[0].value),
        }
    }
}

// Definition of the SortedHand trait
trait SortedHand {
    fn is_straight(&mut self) -> bool;
    fn is_flush(&self) -> bool;
    fn get_pairs(&self) -> Vec<(i32, i32)>;
    fn cmp(&self, other: &Self) -> Ordering;
    fn get_vals(&self) -> Vec<i32>;
    fn sort(&mut self);
}

// Implementation of SortedHand for Hand
impl SortedHand for Hand {
    // Sort the hand by card value
    fn sort(&mut self) {
        self.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }

    // Compare two hands
    fn cmp(&self, other: &Self) -> Ordering {
        let rank = Rankings::from(self);
        match rank.partial_cmp(&Rankings::from(other)).unwrap() {
            // Rankings resulted from pairs get compared by pairs, others get compared by card values
            Ordering::Equal => match rank {
                Rankings::Pair(_)
                | Rankings::TwoPairs(_)
                | Rankings::ThreeOfAKind(_)
                | Rankings::FullHouse(_)
                | Rankings::FourOfAKind(_) => self.get_pairs().cmp(&other.get_pairs()),
                _ => self.get_vals().cmp(&other.get_vals()),
            },
            ord => ord,
        }
    }

    // Check if a hand is a flush
    fn is_flush(&self) -> bool {
        let head_type = self[0].card_type;

        head_type != CardTypes::None && !self.iter().any(|card| card.card_type != head_type)
    }

    // Check if a hand is a straight
    fn is_straight(&mut self) -> bool {
        if self[0].card_type == CardTypes::None {
            return false;
        }
        // If card is an ace, then tests ace with value as 1
        if self[0].value == 14 {
            let mut other = self.clone();
            let taken = other.remove(0);
            other.push(Card {
                value: 1,
                card_type: taken.card_type,
            });

            if other.is_straight() {
                self.remove(0);
                self.push(Card {
                    value: 1,
                    card_type: taken.card_type,
                });
                return true;
            }
        }

        let mut s_iter = self.iter().rev();
        let mut range = (s_iter.next().unwrap_or(&EMPTY_CARD).value
            ..=s_iter.last().unwrap_or(&EMPTY_CARD).value)
            .rev();

        !self
            .iter()
            .any(|card| range.next().unwrap_or(0) != card.value)
    }

    // Get the pairs in a hand
    fn get_pairs(&self) -> Vec<(i32, i32)> {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();

        self.iter().for_each(|card| {
            freq_map
                .entry(card.value)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });

        let mut freq_vec: Vec<(i32, i32)> = freq_map.into_iter().collect();

        freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));

        freq_vec
    }

    // Get the values of the cards in a hand
    fn get_vals(&self) -> Vec<i32> {
        self.iter().map(|card| card.value).collect()
    }
}
