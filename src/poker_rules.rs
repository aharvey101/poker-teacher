use bevy::prelude::*;
use crate::cards::{Card, Suit};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandEvaluation {
    pub rank: HandRank,
    pub primary_value: u8,    // Main value (e.g., pair rank, high card)
    pub secondary_value: u8,  // Secondary value (e.g., kicker, second pair)
    pub kickers: Vec<u8>,     // Remaining cards for tie-breaking
}

impl PartialOrd for HandEvaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandEvaluation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare hand ranks
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => {
                // Then compare primary values
                match self.primary_value.cmp(&other.primary_value) {
                    std::cmp::Ordering::Equal => {
                        // Then compare secondary values
                        match self.secondary_value.cmp(&other.secondary_value) {
                            std::cmp::Ordering::Equal => {
                                // Finally compare kickers
                                self.kickers.cmp(&other.kickers)
                            }
                            other => other,
                        }
                    }
                    other => other,
                }
            }
            other => other,
        }
    }
}

pub fn evaluate_hand(hole_cards: &[Card], community_cards: &[Card]) -> HandEvaluation {
    let mut all_cards = Vec::new();
    all_cards.extend_from_slice(hole_cards);
    all_cards.extend_from_slice(community_cards);
    
    // Find the best 5-card hand from available cards
    let best_hand = find_best_five_card_hand(&all_cards);
    evaluate_five_card_hand(&best_hand)
}

fn find_best_five_card_hand(cards: &[Card]) -> Vec<Card> {
    if cards.len() <= 5 {
        return cards.to_vec();
    }
    
    let mut best_hand = Vec::new();
    let mut best_evaluation = HandEvaluation {
        rank: HandRank::HighCard,
        primary_value: 0,
        secondary_value: 0,
        kickers: vec![],
    };
    
    // Generate all possible 5-card combinations
    for combo in combinations(cards, 5) {
        let evaluation = evaluate_five_card_hand(&combo);
        if evaluation > best_evaluation {
            best_evaluation = evaluation;
            best_hand = combo;
        }
    }
    
    best_hand
}

fn combinations(cards: &[Card], k: usize) -> Vec<Vec<Card>> {
    if k == 0 {
        return vec![vec![]];
    }
    if cards.is_empty() || k > cards.len() {
        return vec![];
    }
    if k == cards.len() {
        return vec![cards.to_vec()];
    }
    
    let mut result = Vec::new();
    
    // Include first card
    for mut combo in combinations(&cards[1..], k - 1) {
        combo.insert(0, cards[0]);
        result.push(combo);
    }
    
    // Exclude first card
    result.extend(combinations(&cards[1..], k));
    
    result
}

fn evaluate_five_card_hand(cards: &[Card]) -> HandEvaluation {
    if cards.len() != 5 {
        error!("Hand evaluation requires exactly 5 cards, got {}", cards.len());
        return HandEvaluation {
            rank: HandRank::HighCard,
            primary_value: 0,
            secondary_value: 0,
            kickers: vec![0; 5],
        };
    }
    
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by(|a, b| b.rank.cmp(&a.rank)); // Sort descending
    
    let ranks: Vec<u8> = sorted_cards.iter().map(|c| c.rank as u8).collect();
    let suits: Vec<Suit> = sorted_cards.iter().map(|c| c.suit).collect();
    
    // Count rank frequencies
    let mut rank_counts = HashMap::new();
    for &rank in &ranks {
        *rank_counts.entry(rank).or_insert(0) += 1;
    }
    
    let is_flush = suits.iter().all(|&s| s == suits[0]);
    let is_straight = is_straight_hand(&ranks);
    
    // Check for royal flush
    if is_flush && is_straight && ranks[0] == 14 { // Ace high straight
        return HandEvaluation {
            rank: HandRank::RoyalFlush,
            primary_value: 14,
            secondary_value: 0,
            kickers: vec![],
        };
    }
    
    // Check for straight flush
    if is_flush && is_straight {
        return HandEvaluation {
            rank: HandRank::StraightFlush,
            primary_value: ranks[0],
            secondary_value: 0,
            kickers: vec![],
        };
    }
    
    // Sort rank counts by frequency and then by rank
    let mut count_groups: Vec<(usize, u8)> = rank_counts
        .into_iter()
        .map(|(rank, count)| (count, rank))
        .collect();
    count_groups.sort_by(|a, b| {
        match b.0.cmp(&a.0) {
            std::cmp::Ordering::Equal => b.1.cmp(&a.1),
            other => other,
        }
    });
    
    match count_groups.as_slice() {
        // Four of a kind
        [(4, quad_rank), (1, kicker)] => HandEvaluation {
            rank: HandRank::FourOfAKind,
            primary_value: *quad_rank,
            secondary_value: 0,
            kickers: vec![*kicker],
        },
        
        // Full house
        [(3, trip_rank), (2, pair_rank)] => HandEvaluation {
            rank: HandRank::FullHouse,
            primary_value: *trip_rank,
            secondary_value: *pair_rank,
            kickers: vec![],
        },
        
        // Three of a kind
        [(3, trip_rank), (1, k1), (1, k2)] => {
            let mut kickers = vec![*k1, *k2];
            kickers.sort_by(|a, b| b.cmp(a));
            HandEvaluation {
                rank: HandRank::ThreeOfAKind,
                primary_value: *trip_rank,
                secondary_value: 0,
                kickers,
            }
        },
        
        // Two pair
        [(2, high_pair), (2, low_pair), (1, kicker)] => {
            let (high, low) = if high_pair > low_pair {
                (*high_pair, *low_pair)
            } else {
                (*low_pair, *high_pair)
            };
            HandEvaluation {
                rank: HandRank::TwoPair,
                primary_value: high,
                secondary_value: low,
                kickers: vec![*kicker],
            }
        },
        
        // One pair
        [(2, pair_rank), (1, k1), (1, k2), (1, k3)] => {
            let mut kickers = vec![*k1, *k2, *k3];
            kickers.sort_by(|a, b| b.cmp(a));
            HandEvaluation {
                rank: HandRank::OnePair,
                primary_value: *pair_rank,
                secondary_value: 0,
                kickers,
            }
        },
        
        // High card or flush or straight
        _ => {
            if is_flush {
                HandEvaluation {
                    rank: HandRank::Flush,
                    primary_value: ranks[0],
                    secondary_value: 0,
                    kickers: ranks[1..].to_vec(),
                }
            } else if is_straight {
                HandEvaluation {
                    rank: HandRank::Straight,
                    primary_value: ranks[0],
                    secondary_value: 0,
                    kickers: vec![],
                }
            } else {
                HandEvaluation {
                    rank: HandRank::HighCard,
                    primary_value: ranks[0],
                    secondary_value: 0,
                    kickers: ranks[1..].to_vec(),
                }
            }
        }
    }
}

fn is_straight_hand(ranks: &[u8]) -> bool {
    if ranks.len() != 5 {
        return false;
    }
    
    // Check for regular straight
    for i in 0..4 {
        if ranks[i] - ranks[i + 1] != 1 {
            // Check for low ace straight (A-2-3-4-5)
            if ranks == &[14, 5, 4, 3, 2] {
                return true;
            }
            return false;
        }
    }
    
    true
}

pub fn hand_rank_name(rank: &HandRank) -> &'static str {
    match rank {
        HandRank::HighCard => "High Card",
        HandRank::OnePair => "One Pair", 
        HandRank::TwoPair => "Two Pair",
        HandRank::ThreeOfAKind => "Three of a Kind",
        HandRank::Straight => "Straight",
        HandRank::Flush => "Flush",
        HandRank::FullHouse => "Full House",
        HandRank::FourOfAKind => "Four of a Kind",
        HandRank::StraightFlush => "Straight Flush",
        HandRank::RoyalFlush => "Royal Flush",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Suit, Rank};
    
    #[test]
    fn test_royal_flush() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Ten),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::RoyalFlush);
        assert_eq!(eval.primary_value, 14);
    }
    
    #[test]
    fn test_straight_flush() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Nine),
            Card::new(Suit::Spades, Rank::Eight),
            Card::new(Suit::Spades, Rank::Seven),
            Card::new(Suit::Spades, Rank::Six),
            Card::new(Suit::Spades, Rank::Five),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::StraightFlush);
        assert_eq!(eval.primary_value, 9);
    }
    
    #[test]
    fn test_four_of_a_kind() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Hearts, Rank::Ace),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::FourOfAKind);
        assert_eq!(eval.primary_value, 13); // King
        assert_eq!(eval.kickers[0], 14); // Ace kicker
    }
    
    #[test]
    fn test_full_house() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ace),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::FullHouse);
        assert_eq!(eval.primary_value, 13); // Kings
        assert_eq!(eval.secondary_value, 14); // Aces
    }
    
    #[test]
    fn test_flush() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Ten),
            Card::new(Suit::Hearts, Rank::Eight),
            Card::new(Suit::Hearts, Rank::Five),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::Flush);
        assert_eq!(eval.primary_value, 13); // King high
    }
    
    #[test]
    fn test_straight() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Hearts, Rank::Nine),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::Straight);
        assert_eq!(eval.primary_value, 13); // King high
    }
    
    #[test]
    fn test_low_ace_straight() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::Five),
            Card::new(Suit::Clubs, Rank::Four),
            Card::new(Suit::Spades, Rank::Three),
            Card::new(Suit::Hearts, Rank::Two),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::Straight);
        assert_eq!(eval.primary_value, 14); // Ace high (wheel)
    }
    
    #[test]
    fn test_three_of_a_kind() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Queen),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::ThreeOfAKind);
        assert_eq!(eval.primary_value, 13); // King trips
        assert_eq!(eval.kickers[0], 14); // Ace kicker
        assert_eq!(eval.kickers[1], 12); // Queen kicker
    }
    
    #[test]
    fn test_two_pair() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Ace),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::TwoPair);
        assert_eq!(eval.primary_value, 13); // Kings
        assert_eq!(eval.secondary_value, 12); // Queens
        assert_eq!(eval.kickers[0], 14); // Ace kicker
    }
    
    #[test]
    fn test_pair() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::OnePair);
        assert_eq!(eval.primary_value, 14); // Ace pair
        assert_eq!(eval.kickers[0], 13); // King kicker
        assert_eq!(eval.kickers[1], 12); // Queen kicker
        assert_eq!(eval.kickers[2], 11); // Jack kicker
    }
    
    #[test]
    fn test_high_card() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Nine),
        ];
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::HighCard);
        assert_eq!(eval.primary_value, 14); // Ace high
        assert_eq!(eval.kickers[0], 13); // King
        assert_eq!(eval.kickers[1], 12); // Queen
        assert_eq!(eval.kickers[2], 11); // Jack
        assert_eq!(eval.kickers[3], 9); // Nine
    }
    
    #[test]
    fn test_hand_comparison() {
        let royal_flush = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Ten),
        ];
        
        let straight_flush = vec![
            Card::new(Suit::Spades, Rank::Nine),
            Card::new(Suit::Spades, Rank::Eight),
            Card::new(Suit::Spades, Rank::Seven),
            Card::new(Suit::Spades, Rank::Six),
            Card::new(Suit::Spades, Rank::Five),
        ];
        
        let royal_eval = evaluate_five_card_hand(&royal_flush);
        let straight_eval = evaluate_five_card_hand(&straight_flush);
        
        assert!(royal_eval > straight_eval);
    }
    
    #[test]
    fn test_best_hand_from_seven() {
        let player_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
        ];
        
        let community_cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Ten),
        ];
        
        let best_hand = evaluate_hand(&player_cards, &community_cards);
        assert_eq!(best_hand.rank, HandRank::Straight); // A-K-Q-J-10 straight is better than Aces and Kings two pair
        assert_eq!(best_hand.primary_value, 14); // High card of straight (Ace)
    }
    
    #[test]
    fn test_invalid_hand_size() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
        ]; // Only 3 cards
        
        let eval = evaluate_five_card_hand(&cards);
        assert_eq!(eval.rank, HandRank::HighCard);
        assert_eq!(eval.primary_value, 0);
    }
    
    #[test]
    fn test_hand_rank_names() {
        assert_eq!(hand_rank_name(&HandRank::RoyalFlush), "Royal Flush");
        assert_eq!(hand_rank_name(&HandRank::StraightFlush), "Straight Flush");
        assert_eq!(hand_rank_name(&HandRank::FourOfAKind), "Four of a Kind");
        assert_eq!(hand_rank_name(&HandRank::FullHouse), "Full House");
        assert_eq!(hand_rank_name(&HandRank::Flush), "Flush");
        assert_eq!(hand_rank_name(&HandRank::Straight), "Straight");
        assert_eq!(hand_rank_name(&HandRank::ThreeOfAKind), "Three of a Kind");
        assert_eq!(hand_rank_name(&HandRank::TwoPair), "Two Pair");
        assert_eq!(hand_rank_name(&HandRank::OnePair), "One Pair");
        assert_eq!(hand_rank_name(&HandRank::HighCard), "High Card");
    }
}
