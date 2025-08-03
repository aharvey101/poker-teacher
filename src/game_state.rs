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
