use bevy::prelude::*;
use crate::cards::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    Human,
    AI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
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
    #[allow(dead_code)]
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
    
    #[allow(dead_code)]
    pub fn place_bet(&mut self, amount: u32) -> bool {
        if amount > self.chips {
            false
        } else {
            self.chips -= amount;
            self.current_bet += amount;
            true
        }
    }
    
    #[allow(dead_code)]
    pub fn fold(&mut self) {
        self.has_folded = true;
    }
    
    #[allow(dead_code)]
    pub fn reset_for_new_hand(&mut self) {
        self.has_folded = false;
        self.current_bet = 0;
        self.hole_cards.clear();
    }
    
    pub fn is_all_in(&self) -> bool {
        self.chips == 0 && !self.has_folded
    }
    
    #[allow(dead_code)]
    pub fn can_bet(&self, amount: u32) -> bool {
        !self.has_folded && self.chips >= amount
    }
    
    #[allow(dead_code)]
    pub fn total_invested(&self) -> u32 {
        self.current_bet
    }
    
    #[allow(dead_code)]
    pub fn is_eliminated(&self) -> bool {
        self.chips == 0 && self.has_folded
    }
    
    #[allow(dead_code)]
    pub fn can_act(&self) -> bool {
        !self.has_folded && !self.is_all_in()
    }
    
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub difficulty: AIDifficulty,
}

#[derive(Debug, Clone, Copy)]
pub enum AIDifficulty {
    Beginner,
    Intermediate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Suit, Rank};
    
    #[test]
    fn test_player_creation() {
        let player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        assert_eq!(player.id, 1);
        assert_eq!(player.player_type, PlayerType::Human);
        assert_eq!(player.chips, 1000);
        assert_eq!(player.current_bet, 0);
        assert!(!player.has_folded);
        assert_eq!(player.hole_cards.len(), 0);
    }
    
    #[test]
    fn test_add_cards() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Spades, Rank::King);
        
        player.add_card(card1);
        assert_eq!(player.hole_cards.len(), 1);
        assert_eq!(player.hole_cards[0], card1);
        
        player.add_card(card2);
        assert_eq!(player.hole_cards.len(), 2);
        assert_eq!(player.hole_cards[1], card2);
    }
    
    #[test]
    fn test_place_bet_success() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        let success = player.place_bet(100);
        assert!(success);
        assert_eq!(player.chips, 900);
        assert_eq!(player.current_bet, 100);
    }
    
    #[test]
    fn test_place_bet_insufficient_chips() {
        let mut player = Player::new(1, PlayerType::Human, 100, Vec3::ZERO);
        
        let success = player.place_bet(150);
        assert!(!success);
        assert_eq!(player.chips, 100); // Should remain unchanged
        assert_eq!(player.current_bet, 0);
    }
    
    #[test]
    fn test_multiple_bets() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        player.place_bet(100);
        assert_eq!(player.chips, 900);
        assert_eq!(player.current_bet, 100);
        
        player.place_bet(50);
        assert_eq!(player.chips, 850);
        assert_eq!(player.current_bet, 150);
    }
    
    #[test]
    fn test_fold() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        assert!(!player.has_folded);
        player.fold();
        assert!(player.has_folded);
    }
    
    #[test]
    fn test_all_in() {
        let mut player = Player::new(1, PlayerType::Human, 100, Vec3::ZERO);
        
        assert!(!player.is_all_in());
        
        player.place_bet(100);
        assert!(player.is_all_in());
        assert_eq!(player.chips, 0);
    }
    
    #[test]
    fn test_all_in_when_folded() {
        let mut player = Player::new(1, PlayerType::Human, 0, Vec3::ZERO);
        
        player.fold();
        assert!(!player.is_all_in()); // Folded players are not all-in
    }
    
    #[test]
    fn test_can_bet() {
        let mut player = Player::new(1, PlayerType::Human, 500, Vec3::ZERO);
        
        assert!(player.can_bet(100));
        assert!(player.can_bet(500));
        assert!(!player.can_bet(600));
        
        player.fold();
        assert!(!player.can_bet(100)); // Can't bet when folded
    }
    
    #[test]
    fn test_reset_for_new_hand() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        // Set up some state
        player.add_card(Card::new(Suit::Hearts, Rank::Ace));
        player.add_card(Card::new(Suit::Spades, Rank::King));
        player.place_bet(100);
        player.fold();
        
        assert_eq!(player.hole_cards.len(), 2);
        assert_eq!(player.current_bet, 100);
        assert!(player.has_folded);
        
        // Reset for new hand
        player.reset_for_new_hand();
        
        assert_eq!(player.hole_cards.len(), 0);
        assert_eq!(player.current_bet, 0);
        assert!(!player.has_folded);
        assert_eq!(player.chips, 900); // Chips should remain as they were
    }
    
    #[test]
    fn test_clear_cards() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        player.add_card(Card::new(Suit::Hearts, Rank::Ace));
        player.add_card(Card::new(Suit::Spades, Rank::King));
        assert_eq!(player.hole_cards.len(), 2);
        
        player.clear_cards();
        assert_eq!(player.hole_cards.len(), 0);
    }
    
    #[test]
    fn test_total_invested() {
        let mut player = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        
        assert_eq!(player.total_invested(), 0);
        
        player.place_bet(100);
        assert_eq!(player.total_invested(), 100);
        
        player.place_bet(50);
        assert_eq!(player.total_invested(), 150);
    }
    
    #[test]
    fn test_ai_vs_human_player() {
        let human = Player::new(1, PlayerType::Human, 1000, Vec3::ZERO);
        let ai = Player::new(2, PlayerType::AI, 1000, Vec3::new(1.0, 0.0, 0.0));
        
        assert_eq!(human.player_type, PlayerType::Human);
        assert_eq!(ai.player_type, PlayerType::AI);
        assert_ne!(human.id, ai.id);
    }
}
