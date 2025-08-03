use bevy::prelude::*;
use rand::prelude::*;

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
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    
    pub fn reset(&mut self) {
        *self = Deck::default();
        self.shuffle();
    }
}
