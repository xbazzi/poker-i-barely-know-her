# poker-i-barely-know-her

A Texas Hold'em poker simulator written in Rust. Deals hole cards and community cards, evaluates the best 5-card hand from 7, and declares a winner.

## Usage

```bash
cargo run
```

The simulator reads `game_config.txt` from the current directory. This file contains two integers on separate lines:

```
<number of decks>
<number of players>
```

Example `game_config.txt`:
```
1
2
```

## How it works

Each round deals 2 hole cards per player and 5 community cards (flop, turn, river). Each player's best 5-card hand is selected from their 2 hole cards + 5 community cards, then hands are ranked and a winner is declared.

### Hand rankings (low → high)

1. High Card
2. One Pair
3. Two Pair
4. Three of a Kind
5. Straight
6. Flush
7. Full House
8. Four of a Kind
9. Straight Flush
10. Royal Flush

## Project structure

```
src/
├── main.rs        entry point — reads odds.txt, starts game
├── card.rs        Suit/Rank enums, Card type
├── deck.rs        Deck with shuffle and deal
├── evaluator.rs   5-card evaluator and 7-card best-hand selector
└── game.rs        Player, dealing flow, winner determination
```

## Build

Requires Rust (2024 edition).

```bash
cargo build
cargo test
```
