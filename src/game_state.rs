use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Playing,
    Paused,
    #[allow(dead_code)] // Reserved for mobile app lifecycle
    Suspended,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Setup,
    Dealing,
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
    GameOver,
}

#[derive(Resource, Debug)]
pub struct GameData {
    pub current_player: u32,
    pub pot: u32,
    pub current_bet: u32,
    pub community_cards: Vec<crate::cards::Card>,
    pub round_number: u32,
}

// New resource for managing dealer position and blinds
#[derive(Resource, Debug)]
pub struct GamePosition {
    pub dealer_button: u32,      // Player ID who has the dealer button
    pub small_blind_amount: u32, // Small blind amount
    pub big_blind_amount: u32,   // Big blind amount
    pub total_players: u32,      // Total number of players
}

impl Default for GamePosition {
    fn default() -> Self {
        Self {
            dealer_button: 0,
            small_blind_amount: 10,
            big_blind_amount: 20,
            total_players: 3,
        }
    }
}

impl GamePosition {
    pub fn get_small_blind_player(&self) -> u32 {
        // Small blind is next player after dealer
        (self.dealer_button + 1) % self.total_players
    }
    
    pub fn get_big_blind_player(&self) -> u32 {
        // Big blind is two players after dealer
        (self.dealer_button + 2) % self.total_players
    }
    
    pub fn advance_dealer_button(&mut self) {
        // Move dealer button to next player
        self.dealer_button = (self.dealer_button + 1) % self.total_players;
        info!("🔄 Dealer button moved to Player {}", self.dealer_button);
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            current_player: 0,
            pot: 0,
            current_bet: 0,
            community_cards: Vec::new(),
            round_number: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_state_enum_values() {
        // Test that game states can be compared for equality
        assert_eq!(GameState::Setup, GameState::Setup);
        assert_ne!(GameState::Setup, GameState::Dealing);
        assert_ne!(GameState::Dealing, GameState::PreFlop);
        assert_ne!(GameState::PreFlop, GameState::Flop);
        assert_ne!(GameState::Flop, GameState::Turn);
        assert_ne!(GameState::Turn, GameState::River);
        assert_ne!(GameState::River, GameState::Showdown);
        assert_ne!(GameState::Showdown, GameState::GameOver);
    }
    
    #[test]
    fn test_app_state_transitions() {
        assert_ne!(AppState::Playing, AppState::Suspended);
    }
    
    #[test]
    fn test_game_data_creation() {
        let game_data = GameData::default();
        
        assert_eq!(game_data.current_player, 0);
        assert_eq!(game_data.pot, 0);
        assert_eq!(game_data.current_bet, 0);
        assert_eq!(game_data.community_cards.len(), 0);
        assert_eq!(game_data.round_number, 1);
    }
    
    #[test]
    fn test_add_to_pot() {
        let mut game_data = GameData::default();
        
        game_data.add_to_pot(100);
        assert_eq!(game_data.pot, 100);
        
        game_data.add_to_pot(50);
        assert_eq!(game_data.pot, 150);
    }
    
    #[test]
    fn test_next_player() {
        let mut game_data = GameData::default();
        
        game_data.next_player(3);
        assert_eq!(game_data.current_player, 1);
        
        game_data.next_player(3);
        assert_eq!(game_data.current_player, 2);
        
        game_data.next_player(3);
        assert_eq!(game_data.current_player, 0); // Should wrap around
    }
    
    #[test]
    fn test_game_position_default() {
        let position = GamePosition::default();
        
        assert_eq!(position.dealer_button, 0);
        assert_eq!(position.small_blind_amount, 10);
        assert_eq!(position.big_blind_amount, 20);
        assert_eq!(position.total_players, 3);
    }
    
    #[test]
    fn test_blind_positions() {
        let position = GamePosition::default();
        
        // With dealer at position 0
        assert_eq!(position.get_small_blind_player(), 1);
        assert_eq!(position.get_big_blind_player(), 2);
    }
    
    #[test]
    fn test_advance_dealer_button() {
        let mut position = GamePosition::default();
        
        assert_eq!(position.dealer_button, 0);
        
        position.advance_dealer_button();
        assert_eq!(position.dealer_button, 1);
        
        position.advance_dealer_button();
        assert_eq!(position.dealer_button, 2);
        
        position.advance_dealer_button();
        assert_eq!(position.dealer_button, 0); // Should wrap around (3 players default)
    }
    
    #[test]
    fn test_betting_order_preflop() {
        let position = GamePosition::default();
        let order = position.get_betting_order(true);
        
        // Pre-flop order should start after big blind (position 0)
        assert_eq!(order, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_betting_order_postflop() {
        let position = GamePosition::default();
        let order = position.get_betting_order(false);
        
        // Post-flop order should start with small blind (position 1)
        assert_eq!(order, vec![1, 2, 0]);
    }
}

impl GameData {
    pub fn new_round(&mut self) {
        self.round_number += 1;
        self.pot = 0;
        self.current_bet = 0;
        self.community_cards.clear();
        self.current_player = 0;
    }
}
