use crate::{
    card::Card,
    deck::Deck,
    evaluator::{HandRank, best_hand},
};

pub fn play() {
    let mut deck = Deck::new();
    let mut player_hand: Vec<Option<Card>> = vec![None, None];
    let mut ai_hand: Vec<Option<Card>> = vec![None, None];
    let mut community_hand: Vec<Option<Card>> = vec![None, None, None, None, None];

    player_hand.push(deck.deal());
    ai_hand.push(deck.deal());
    player_hand.push(deck.deal());
    ai_hand.push(deck.deal());

    print_cards(player_hand);

    deck.deal();
    community_hand.push(deck.deal());
    deck.deal();
    community_hand.push(deck.deal());
    deck.deal();
    community_hand.push(deck.deal());
    print_cards(community_hand); // flop

    deck.deal();
    community_hand.push(deck.deal()); // turn
    let card_to_print = community_hand.last().unwrap().unwrap();
    println!("{}", card_to_print);

    deck.deal();
    community_hand.push(deck.deal()); // river
    println!("{}", card_to_print);

    deck.deal();
    print_cards(community_hand[community_hand.len() - 1]);

    let winning_hand = best_hand(player_hand.as_slice());
}

pub fn print_cards(cards: Vec<Option<Card>>) {
    for card in cards {
        println!("{}", card.expect("No card found"));
    }
}
