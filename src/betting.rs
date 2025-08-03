use bevy::prelude::*;
use crate::player::{Player, PlayerType, AIPlayer};
use crate::game_state::GameState;
use crate::ai_player::{make_advanced_ai_decision, AIPlayerComponent};
use crate::cards::Card;
use crate::betting_ui::HumanPlayerInput;

// Player betting actions
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise(u32), // Amount to raise by
}

// Resource to track betting round state
#[derive(Resource, Debug)]
pub struct BettingRound {
    pub current_bet: u32,
    pub min_raise: u32,
    pub players_to_act: Vec<u32>,
    pub betting_complete: bool,
    pub pot: u32,
}

impl Default for BettingRound {
    fn default() -> Self {
        Self {
            current_bet: 0,
            min_raise: 10, // Minimum raise amount
            players_to_act: Vec::new(),
            betting_complete: false,
            pot: 0,
        }
    }
}

impl BettingRound {
    pub fn new(player_ids: Vec<u32>, small_blind: u32) -> Self {
        Self {
            current_bet: small_blind * 2, // Big blind
            min_raise: small_blind * 2,
            players_to_act: player_ids,
            betting_complete: false,
            pot: 0,
        }
    }
    
    pub fn reset_for_new_round(&mut self, player_ids: Vec<u32>) {
        self.current_bet = 0;
        self.players_to_act = player_ids;
        self.betting_complete = false;
        info!("Betting round reset - players to act: {:?}", self.players_to_act);
    }
    
    pub fn is_complete(&self) -> bool {
        self.betting_complete || self.players_to_act.is_empty()
    }
    
    pub fn peek_next_player(&self) -> Option<u32> {
        self.players_to_act.last().copied()
    }
    
    pub fn next_player(&mut self) -> Option<u32> {
        if let Some(player_id) = self.players_to_act.pop() {
            info!("Next player to act: {}, remaining: {:?}", player_id, self.players_to_act);
            Some(player_id)
        } else {
            self.betting_complete = true;
            info!("Betting round complete - no more players to act");
            None
        }
    }
}

// Simple AI decision making
fn make_ai_decision(player: &Player, betting_round: &BettingRound) -> PlayerAction {
    let call_amount = betting_round.current_bet.saturating_sub(player.current_bet);
    
    // Very simple AI logic based on chips and call amount
    if call_amount > player.chips {
        PlayerAction::Fold
    } else if call_amount == 0 {
        PlayerAction::Check
    } else if call_amount <= player.chips / 4 {
        // Call if it's less than 25% of chips
        PlayerAction::Call
    } else {
        // Fold if it's too expensive for simple AI
        PlayerAction::Fold
    }
}

// System to handle AI player decisions
pub fn ai_player_system(
    mut players: Query<(&mut Player, Option<&AIPlayerComponent>)>,
    mut betting_round: ResMut<BettingRound>,
    game_state: Res<State<GameState>>,
    game_data: Res<crate::game_state::GameData>,
    mut human_input: ResMut<HumanPlayerInput>,
) {
    // Only process AI actions during betting phases
    match game_state.get() {
        GameState::PreFlop | GameState::Flop | GameState::Turn | GameState::River => {},
        _ => return,
    }
    
    if betting_round.betting_complete {
        return;
    }
    
    // Get the next player to act
    if let Some(current_player_id) = betting_round.peek_next_player() {
        // First pass: count active players and find current player
        let active_players = players.iter()
            .filter(|(p, _)| !p.has_folded)
            .count();
        
        let mut current_player_data: Option<(Player, Option<AIPlayerComponent>)> = None;
        
        // Find the current player and clone their data
        for (player, ai_component) in players.iter() {
            if player.id == current_player_id && !player.has_folded {
                current_player_data = Some((
                    player.clone(),
                    ai_component.cloned()
                ));
                break;
            }
        }
        
        if let Some((player_data, ai_comp)) = current_player_data {
            let action = match player_data.player_type {
                PlayerType::AI => {
                    // Determine position (simplified - just use player ID for now)
                    let position = player_data.id as usize;
                    
                    // Use advanced AI if component is present, otherwise use simple AI
                    if let Some(ai_component) = ai_comp {
                        make_advanced_ai_decision(
                            &player_data,
                            &betting_round,
                            &game_data.community_cards,
                            &ai_component.personality,
                            active_players,
                            position,
                        )
                    } else {
                        make_ai_decision(&player_data, &betting_round)
                    }
                },
                PlayerType::Human => {
                    // Check if human has made a decision
                    if let Some(human_action) = human_input.pending_action.take() {
                        human_action
                    } else {
                        // Human hasn't decided yet, don't remove them from queue
                        return;
                    }
                },
            };
            
            // Only remove the player from the queue after they've made a decision
            betting_round.next_player(); // This pops the player from the queue
            
            // Second pass: apply the action to the actual player
            for (mut player, _) in players.iter_mut() {
                if player.id == current_player_id {
                    process_player_action(&mut player, action, &mut betting_round);
                    break;
                }
            }
        }
    }
}

// Process a player's action
fn process_player_action(
    player: &mut Player,
    action: PlayerAction,
    betting_round: &mut BettingRound,
) {
    match action {
        PlayerAction::Fold => {
            player.has_folded = true;
            info!("Player {} folded", player.id);
        }
        PlayerAction::Check => {
            info!("Player {} checked", player.id);
        }
        PlayerAction::Call => {
            let call_amount = betting_round.current_bet.saturating_sub(player.current_bet);
            if player.chips >= call_amount {
                player.chips -= call_amount;
                player.current_bet += call_amount;
                betting_round.pot += call_amount;
                info!("Player {} called with ${}", player.id, call_amount);
            } else {
                // All-in
                let all_in_amount = player.chips;
                player.current_bet += all_in_amount;
                betting_round.pot += all_in_amount;
                player.chips = 0;
                info!("Player {} went all-in with ${}", player.id, all_in_amount);
            }
        }
        PlayerAction::Raise(amount) => {
            let total_bet = betting_round.current_bet + amount;
            if player.chips >= total_bet {
                let bet_amount = total_bet - player.current_bet;
                player.chips -= bet_amount;
                player.current_bet = total_bet;
                betting_round.pot += bet_amount;
                betting_round.current_bet = total_bet;
                betting_round.min_raise = amount;
                info!("Player {} raised to ${}", player.id, total_bet);
            } else {
                // Convert to all-in
                let all_in_amount = player.chips;
                player.current_bet += all_in_amount;
                betting_round.pot += all_in_amount;
                player.chips = 0;
                info!("Player {} went all-in with ${}", player.id, all_in_amount);
            }
        }
    }
}

// System to check if betting round is complete
pub fn check_betting_round_complete(
    players: Query<&Player>,
    mut betting_round: ResMut<BettingRound>,
) {
    if betting_round.betting_complete {
        return;
    }
    
    let active_players: Vec<_> = players
        .iter()
        .filter(|p| !p.has_folded)
        .collect();
    
    // If only one player remains, betting is complete
    if active_players.len() <= 1 {
        betting_round.betting_complete = true;
        info!("Betting complete - only {} active players remain", active_players.len());
        return;
    }
    
    // Check if all active players have acted and bets are equal
    let current_bet = betting_round.current_bet;
    let all_bets_equal = active_players
        .iter()
        .all(|p| p.current_bet >= current_bet || p.chips == 0); // Account for all-in
    
    if betting_round.players_to_act.is_empty() && all_bets_equal {
        betting_round.betting_complete = true;
        info!("Betting round complete - {} players remain", active_players.len());
    }
}
