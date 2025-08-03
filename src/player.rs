use bevy::prelude::*;
use crate::cards::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    Human,
    AI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise(u32), // Amount to raise
}

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub player_type: PlayerType,
    pub chips: u32,
    pub hole_cards: Vec<Card>,
    pub current_bet: u32,
    pub has_folded: bool,
    pub position: Vec3, // For rendering position
}

impl Player {
    pub fn new(id: u32, player_type: PlayerType, chips: u32, position: Vec3) -> Self {
        Self {
            id,
            player_type,
            chips,
            hole_cards: Vec::new(),
            current_bet: 0,
            has_folded: false,
            position,
        }
    }
    
    pub fn add_card(&mut self, card: Card) {
        self.hole_cards.push(card);
    }
    
    pub fn clear_cards(&mut self) {
        self.hole_cards.clear();
    }
    
    pub fn can_act(&self) -> bool {
        !self.has_folded && self.chips > 0
    }
    
    pub fn fold(&mut self) {
        self.has_folded = true;
    }
    
    pub fn bet(&mut self, amount: u32) -> u32 {
        let actual_bet = amount.min(self.chips);
        self.chips -= actual_bet;
        self.current_bet += actual_bet;
        actual_bet
    }
}

#[derive(Component)]
pub struct HumanPlayer;

#[derive(Component)]
pub struct AIPlayer {
    pub difficulty: AIDifficulty,
}

#[derive(Debug, Clone, Copy)]
pub enum AIDifficulty {
    Beginner,
    Intermediate,
}
