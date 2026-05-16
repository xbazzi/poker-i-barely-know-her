use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

const DECK_CAPACITY: usize = 52;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn get_cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(DECK_CAPACITY);
        let suits = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = vec![
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        for r in &ranks {
            for s in &suits {
                cards.push(Card { suit: *s, rank: *r });
            }
        }

        Self { cards }
    }
}
