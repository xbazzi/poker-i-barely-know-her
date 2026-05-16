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

    // let's actually play the game now
    game::play();

    Ok(())
}
