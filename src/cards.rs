use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

#[derive(Resource)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();
        
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
                Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                Rank::Jack, Rank::Queen, Rank::King, Rank::Ace
            ] {
                cards.push(Card::new(suit, rank));
            }
        }
        
        Self { cards }
    }
}

impl Deck {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut deck = Self::default();
        deck.shuffle();
        deck
    }
    
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    
    #[allow(dead_code)]
    pub fn cards_remaining(&self) -> usize {
        self.cards.len()
    }
    
    pub fn reset(&mut self) {
        *self = Deck::default();
        self.shuffle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_deck_creation() {
        let deck = Deck::default(); // Use default for consistent testing
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.cards_remaining(), 52);
    }
    
    #[test]
    fn test_deck_has_all_cards() {
        let deck = Deck::default();
        let mut card_set = HashSet::new();
        
        // Check that we have exactly one of each card
        for card in &deck.cards {
            let card_tuple = (card.suit, card.rank as u8);
            assert!(!card_set.contains(&card_tuple), "Duplicate card found: {:?}", card);
            card_set.insert(card_tuple);
        }
        
        assert_eq!(card_set.len(), 52);
    }
    
    #[test]
    fn test_deck_shuffle() {
        let deck1 = Deck::default();
        let mut deck2 = Deck::default();
        
        deck2.shuffle();
        
        // It's extremely unlikely that shuffle produces the same order
        // This test might rarely fail due to randomness, but it's very unlikely
        let same_order = deck1.cards.iter().zip(deck2.cards.iter()).all(|(a, b)| {
            a.suit == b.suit && a.rank == b.rank
        });
        
        // If they are the same, shuffle again and check
        if same_order {
            deck2.shuffle();
            let still_same = deck1.cards.iter().zip(deck2.cards.iter()).all(|(a, b)| {
                a.suit == b.suit && a.rank == b.rank
            });
            assert!(!still_same, "Shuffle doesn't seem to be working");
        }
    }
    
    #[test]
    fn test_dealing_cards() {
        let mut deck = Deck::default();
        
        assert_eq!(deck.cards_remaining(), 52);
        
        let card1 = deck.deal();
        assert!(card1.is_some());
        assert_eq!(deck.cards_remaining(), 51);
        
        let card2 = deck.deal();
        assert!(card2.is_some());
        assert_eq!(deck.cards_remaining(), 50);
    }
    
    #[test]
    fn test_deck_exhaustion() {
        let mut deck = Deck::default();
        
        // Deal all cards
        for i in (0..52).rev() {
            assert_eq!(deck.cards_remaining(), i + 1);
            let card = deck.deal();
            assert!(card.is_some());
        }
        
        assert_eq!(deck.cards_remaining(), 0);
        
        // Should return None when deck is empty
        let card = deck.deal();
        assert!(card.is_none());
    }
    
    #[test]
    fn test_deck_reset() {
        let mut deck = Deck::default();
        
        // Deal some cards
        for _ in 0..10 {
            deck.deal();
        }
        
        assert_eq!(deck.cards_remaining(), 42);
        
        // Reset deck
        deck.reset();
        assert_eq!(deck.cards_remaining(), 52);
    }
    
    #[test]
    fn test_card_equality() {
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Hearts, Rank::Ace);
        let card3 = Card::new(Suit::Spades, Rank::Ace);
        
        assert_eq!(card1, card2);
        assert_ne!(card1, card3);
    }
    
    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Queen > Rank::Jack);
        assert!(Rank::Jack > Rank::Ten);
        assert!(Rank::Ten > Rank::Nine);
        assert!(Rank::Two < Rank::Three);
    }
    
    #[test]
    fn test_suit_enum() {
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        
        // Test that all suits are different
        for (i, suit1) in suits.iter().enumerate() {
            for (j, suit2) in suits.iter().enumerate() {
                if i != j {
                    assert_ne!(suit1, suit2);
                }
            }
        }
    }
}
