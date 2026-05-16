mod card;
mod deck;
mod evaluator;
mod game;

use std::fs;
use std::io;

use crate::{
    card::{Card, Rank, Suit},
    deck::Deck,
};

fn main() -> io::Result<()> {
    // pro way
    let contents = fs::read_to_string("odds.txt").expect("failed to read file");
    let mut nums = contents
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("expected a number, phahm"));

    let decks = nums.next().expect("missing number of decks");
    let players = nums.next().expect("missing number of players");
    println!("decks: {}, players: {}\n", decks, players);

    let mut deck = Deck::new();

    let print_deck = |deck: &Deck| {
        let deck_size = deck.get_cards().len();
        for (i, card) in deck.get_cards().iter().enumerate() {
            if i % 13 == 0 {
                println!();
            }
            if i + 1 == deck_size {
                print!("{card} ");
            } else {
                print!("{card} ");
            }
        }
        println!();
    };

    print_deck(&deck);
    println!("\nShuffling the deck...");
    deck.shuffle();

    print_deck(&deck);
    print!(
        "\nDealing 3 cards: {} {} {}",
        deck.deal().expect("deck is empty"),
        deck.deal().expect("deck is empty"),
        deck.deal().expect("deck is empty")
    );
    print_deck(&deck);

    Ok(())
}
