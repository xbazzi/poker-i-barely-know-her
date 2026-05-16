use crate::card::{Card, Rank};
use std::collections::HashMap;

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

pub fn best_hand(cards: &[Card; 7]) -> HandRank {
    let size = cards.len();
    let mut best: Option<HandRank> = None;
    for i in 0..size {
        for j in (i + 1)..size {
            let hand = cards
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i && *idx != j)
                .map(|(_, card)| *card)
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let rank = evaluate(&hand);
            if best.as_ref().map_or(true, |b| &rank > b) {
                best = Some(rank);
            }
        }
    }
    best.unwrap()
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
            return HandRank::StraightFlush(ranks[0]);
        } else {
            return HandRank::StraightFlush(Rank::Five);
        }
    } else if flush {
        if let [r0, r1, r2, r3, r4] = ranks.as_slice() {
            return HandRank::Flush(*r0, *r1, *r2, *r3, *r4);
        } else {
            unreachable!();
        }
    } else if straight {
        if !is_wheel {
            return HandRank::Straight(ranks[0]);
        } else {
            return HandRank::Straight(Rank::Five);
        };
    } else {
        let counts = rank_counts(&ranks);
        match counts.as_slice() {
            [(4, quad), (1, kicker)] => HandRank::FourOfAKind(*quad, *kicker),
            [(3, trip), (2, pair)] => HandRank::FullHouse(*trip, *pair),
            [(3, trip), (1, k1), (1, k2)] => HandRank::ThreeOfAKind(*trip, *k1, *k2),
            [(2, p1), (2, p2), (1, k)] => HandRank::TwoPair(*p1, *p2, *k),
            [(2, pair), (1, k1), (1, k2), (1, k3)] => HandRank::OnePair(*pair, *k1, *k2, *k3),
            _ => {
                if let [r0, r1, r2, r3, r4] = ranks.as_slice() {
                    HandRank::HighCard(*r0, *r1, *r2, *r3, *r4)
                } else {
                    unreachable!();
                }
            }
        }
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

pub fn rank_counts(ranks: &[Rank]) -> Vec<(usize, Rank)> {
    let mut map: HashMap<Rank, usize> = HashMap::new();
    for rank in ranks {
        *map.entry(*rank).or_insert(0) += 1;
    }
    let mut counts: Vec<(usize, Rank)> =
        map.into_iter().map(|(rank, count)| (count, rank)).collect();
    counts.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    counts
}
