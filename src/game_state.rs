use bevy::prelude::*;

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
    
    pub fn get_betting_order(&self, is_preflop: bool) -> Vec<u32> {
        let mut order = Vec::new();
        
        if is_preflop {
            // Pre-flop: start with player after big blind
            let first_to_act = (self.dealer_button + 3) % self.total_players;
            for i in 0..self.total_players {
                order.push((first_to_act + i) % self.total_players);
            }
        } else {
            // Post-flop: start with small blind
            let first_to_act = self.get_small_blind_player();
            for i in 0..self.total_players {
                order.push((first_to_act + i) % self.total_players);
            }
        }
        
        order
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

impl GameData {
    pub fn next_player(&mut self, total_players: u32) {
        self.current_player = (self.current_player + 1) % total_players;
    }
    
    pub fn add_to_pot(&mut self, amount: u32) {
        self.pot += amount;
    }
    
    pub fn new_round(&mut self) {
        self.round_number += 1;
        self.pot = 0;
        self.current_bet = 0;
        self.community_cards.clear();
        self.current_player = 0;
    }
}
