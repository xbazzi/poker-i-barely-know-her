use crate::{
    card::Card,
    deck::Deck,
    evaluator::{HandRank, best_hand},
};
use std::fmt;

const PLAYER_HAND_CAPACITY: usize = 2;
const COMMUNITY_HAND_CAPACITY: usize = 2;
const NUM_PLAYERS: usize = 2; // ai and one player
const CARDS_PER_PLAYER: usize = 2; // ai and one player

pub struct Hand {
    cards: Vec<Card>,
}

pub struct Player {
    hand: Hand,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: Vec::with_capacity(PLAYER_HAND_CAPACITY),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hand: Hand::default(),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let joined = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", joined)
    }
}

pub fn deal_cards_to_players(deck: &mut Deck, players: &mut [Player; NUM_PLAYERS]) {
    for _ in 0..CARDS_PER_PLAYER {
        for p in players.iter_mut() {
            if let Some(card) = deck.deal() {
                p.hand.cards.push(card);
            }
        }
    }
}

pub fn play() {
    let mut deck = Deck::new();
    let mut community_hand = Hand {
        cards: Vec::with_capacity(COMMUNITY_HAND_CAPACITY),
    };

    let human: Player = Player::default();
    let ai: Player = Player::default();

    let mut players: [Player; NUM_PLAYERS] = [human, ai];

    deck.shuffle();
    deal_cards_to_players(&mut deck, &mut players);

    println!("Your cards: {}", players[0].hand);

    // deal the flop
    print!("Flop: ");
    deck.deal();
    if let Some(card) = deck.deal() {
        community_hand.cards.push(card);
        print!("{} ", card);
    }
    deck.deal();
    if let Some(card) = deck.deal() {
        community_hand.cards.push(card);
        print!("{} ", card);
    }
    deck.deal();
    if let Some(card) = deck.deal() {
        community_hand.cards.push(card);
        println!("{}", card);
    }

    // deal the turn
    deck.deal();
    if let Some(card) = deck.deal() {
        community_hand.cards.push(card);
        print!("Turn: {}\n", card);
    }

    // deal the river
    deck.deal();
    if let Some(card) = deck.deal() {
        community_hand.cards.push(card);
        println!("River: {}", card);
    }

    // let winning_hand = best_hand(player_hand.as_slice());
}
