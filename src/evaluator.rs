use crate::card::{Card, Rank};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard(Rank, Rank, Rank, Rank, Rank),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Rank, Rank, Rank, Rank, Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

pub fn evaluate(cards: &[Card; 5]) -> HandRank {
    let flush = is_flush(cards);
    let straight = is_straight(cards);

    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort_by(|a, b| b.cmp(a)); // descending

    let is_wheel = ranks[0] == Rank::Ace && ranks[1] == Rank::Five;

    if flush && straight {
        if ranks[0] == Rank::Ace && !is_wheel {
            return HandRank::RoyalFlush;
        } else if !is_wheel {
            return HandRank::StraightFlush(Rank::Five);
        } else {
            return HandRank::StraightFlush(ranks[0]);
        }
    } else if flush {
        if let [r0, r1, r2, r3, r4] = ranks.as_slice() {
            return HandRank::Flush(*r0, *r1, *r2, *r3, *r4);
        }
    } else if straight {
        if !is_wheel {
            return HandRank::Straight(ranks[0]);
        } else {
            return HandRank::Straight(Rank::Five);
        }
    } else {
        // pairs, triples, etc...
    }
}

pub fn is_flush(cards: &[Card; 5]) -> bool {
    cards.iter().all(|card| card.suit == cards[0].suit)
}

pub fn is_straight(cards: &[Card; 5]) -> bool {
    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();

    if is_wheel(&ranks) {
        return true;
    }

    for i in 1..ranks.len() {
        if (ranks[i] as u8 - ranks[i - 1] as u8) != 1 {
            return false;
        }
    }
    true
}

pub fn is_wheel(ranks: &Vec<Rank>) -> bool {
    let wheel = ranks == &vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];
    if wheel {
        return true;
    };
    false
}
