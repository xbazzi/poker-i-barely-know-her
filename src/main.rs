mod card;
mod deck;
mod evaluator;
mod game;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("game_config.txt").expect("failed to read file");
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
