use bevy::prelude::*;
use crate::cards::Deck;
use crate::player::{Player, PlayerType};
use crate::game_state::{GameState, GameData, GamePosition};
use crate::betting::BettingRound;
use crate::poker_rules::{evaluate_hand, hand_rank_name};

// Resource to control game timing
#[derive(Resource)]
pub struct GameController {
    pub state_timer: Timer,
    pub auto_advance: bool,
}

impl Default for GameController {
    fn default() -> Self {
        Self {
            state_timer: Timer::from_seconds(2.0, TimerMode::Once),
            auto_advance: true,
        }
    }
}

// System to handle automatic game state transitions
pub fn game_state_controller(
    time: Res<Time>,
    mut controller: ResMut<GameController>,
    mut game_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    mut deck: ResMut<Deck>,
    mut game_data: ResMut<GameData>,
    mut game_position: ResMut<GamePosition>,
    mut players: Query<&mut Player>,
    mut betting_round: ResMut<BettingRound>,
) {
    if !controller.auto_advance {
        return;
    }
    
    // Check if we should pause auto-advance for human player input
    match current_state.get() {
        GameState::PreFlop | GameState::Flop | GameState::Turn | GameState::River => {
            // Check if it's a human player's turn to act
            if let Some(current_player_id) = betting_round.peek_next_player() {
                for player in players.iter() {
                    if player.id == current_player_id && !player.has_folded {
                        if matches!(player.player_type, crate::player::PlayerType::Human) {
                            // It's a human player's turn - don't auto-advance
                            return;
                        }
                        break;
                    }
                }
            }
        },
        _ => {}, // Allow auto-advance for non-betting phases
    }
    
    controller.state_timer.tick(time.delta());
    
    if controller.state_timer.finished() {
        match current_state.get() {
            GameState::Setup => {
                // Initialize new round
                deck.reset();
                game_data.new_round();
                
                // Reset all players for new round
                for mut player in players.iter_mut() {
                    player.clear_hand();
                    player.current_bet = 0;
                    player.has_folded = false;
                }
                
                // Initialize betting round
                let player_ids: Vec<u32> = players.iter().map(|p| p.id).collect();
                *betting_round = BettingRound::new(player_ids, 10); // $10 small blind
                
                info!("Starting new poker round!");
                game_state.set(GameState::Dealing);
                controller.state_timer.reset();
            },
            
            GameState::Dealing => {
                // First post blinds before dealing
                post_blinds(&mut players, &game_position, &mut game_data);
                
                // Deal 2 cards to each player
                for mut player in players.iter_mut() {
                    for _ in 0..2 {
                        if let Some(card) = deck.deal() {
                            player.add_card(card);
                        }
                    }
                }
                
                info!("Cards dealt to all players, blinds posted");
                
                // Start pre-flop betting with proper betting order
                let active_players: Vec<u32> = players
                    .iter()
                    .filter(|p| !p.has_folded)
                    .map(|p| p.id)
                    .collect();
                betting_round.reset_for_new_round(active_players);
                
                // Set current bet to big blind amount
                game_data.current_bet = game_position.big_blind_amount;
                
                game_state.set(GameState::PreFlop);
                controller.state_timer = Timer::from_seconds(0.5, TimerMode::Once); // Faster for betting
                controller.state_timer.reset();
            },
            
            GameState::PreFlop => {
                // Check if betting is complete
                if betting_round.is_complete() {
                    // Update pot in game data
                    game_data.pot = betting_round.pot;
                    
                    let active_players = players.iter().filter(|p| !p.has_folded).count();
                    if active_players <= 1 {
                        info!("Only one player remaining, skipping to showdown");
                        game_state.set(GameState::Showdown);
                    } else {
                        info!("Pre-flop betting complete, moving to flop");
                        game_state.set(GameState::Flop);
                    }
                    controller.state_timer = Timer::from_seconds(2.0, TimerMode::Once);
                    controller.state_timer.reset();
                }
                // If betting not complete, keep waiting
            },
            
            GameState::Flop => {
                // Deal 3 community cards
                for _ in 0..3 {
                    if let Some(card) = deck.deal() {
                        game_data.community_cards.push(card);
                    }
                }
                
                info!("Flop dealt: {} community cards", game_data.community_cards.len());
                
                // Start post-flop betting
                let active_players: Vec<u32> = players
                    .iter()
                    .filter(|p| !p.has_folded)
                    .map(|p| p.id)
                    .collect();
                betting_round.reset_for_new_round(active_players);
                
                game_state.set(GameState::Turn);
                controller.state_timer = Timer::from_seconds(0.5, TimerMode::Once);
                controller.state_timer.reset();
            },
            
            GameState::Turn => {
                // Check if betting is complete, or deal turn card
                if !game_data.community_cards.is_empty() && game_data.community_cards.len() == 3 {
                    // Deal turn card
                    if let Some(card) = deck.deal() {
                        game_data.community_cards.push(card);
                    }
                    info!("Turn dealt: {} community cards", game_data.community_cards.len());
                    
                    // Start turn betting
                    let active_players: Vec<u32> = players
                        .iter()
                        .filter(|p| !p.has_folded)
                        .map(|p| p.id)
                        .collect();
                    betting_round.reset_for_new_round(active_players);
                } else if betting_round.is_complete() {
                    // Betting complete, move to river
                    game_data.pot = betting_round.pot;
                    
                    let active_players = players.iter().filter(|p| !p.has_folded).count();
                    if active_players <= 1 {
                        game_state.set(GameState::Showdown);
                    } else {
                        game_state.set(GameState::River);
                    }
                    controller.state_timer = Timer::from_seconds(2.0, TimerMode::Once);
                    controller.state_timer.reset();
                }
            },
            
            GameState::River => {
                // Check if betting is complete, or deal river card
                if game_data.community_cards.len() == 4 {
                    // Deal final community card
                    if let Some(card) = deck.deal() {
                        game_data.community_cards.push(card);
                    }
                    info!("River dealt: {} community cards", game_data.community_cards.len());
                    
                    // Start river betting
                    let active_players: Vec<u32> = players
                        .iter()
                        .filter(|p| !p.has_folded)
                        .map(|p| p.id)
                        .collect();
                    betting_round.reset_for_new_round(active_players);
                } else if betting_round.is_complete() {
                    // Final betting complete, move to showdown
                    game_data.pot = betting_round.pot;
                    game_state.set(GameState::Showdown);
                    controller.state_timer = Timer::from_seconds(3.0, TimerMode::Once);
                    controller.state_timer.reset();
                }
            },
            
            GameState::Showdown => {
                // Evaluate hands and determine winner
                determine_winner(&mut players, &game_data, &mut game_position);
                
                game_state.set(GameState::GameOver);
                controller.state_timer = Timer::from_seconds(5.0, TimerMode::Once);
                controller.state_timer.reset();
            },
            
            GameState::GameOver => {
                // Check for game end conditions
                let players_with_chips: Vec<&Player> = players.iter().filter(|p| p.chips > 0).collect();
                
                if players_with_chips.len() <= 1 {
                    // Game is over - only one player has chips left
                    if let Some(winner) = players_with_chips.first() {
                        let winner_name = match winner.player_type {
                            PlayerType::Human => "Human",
                            PlayerType::AI => "AI",
                        };
                        info!("🎉 GAME OVER! {} Player {} wins the entire game with ${} chips!", 
                              winner_name, winner.id, winner.chips);
                    } else {
                        info!("🎉 GAME OVER! All players are eliminated.");
                    }
                    
                    // Reset the game after 10 seconds
                    controller.state_timer = Timer::from_seconds(10.0, TimerMode::Once);
                    if controller.state_timer.finished() {
                        // Reset all players' chips for a new game
                        for mut player in players.iter_mut() {
                            player.chips = 1000; // Reset to starting chips
                            player.clear_hand();
                            player.current_bet = 0;
                            player.has_folded = false;
                        }
                        info!("🔄 Starting new game! All players reset to $1000 chips.");
                        game_state.set(GameState::Setup);
                        controller.state_timer = Timer::from_seconds(2.0, TimerMode::Once);
                        controller.state_timer.reset();
                    }
                } else {
                    // Multiple players still have chips - continue to next round
                    info!("Round complete, starting new round...");
                    info!("Players remaining: {}", players_with_chips.len());
                    for player in &players_with_chips {
                        let player_name = match player.player_type {
                            PlayerType::Human => "Human",
                            PlayerType::AI => "AI",
                        };
                        info!("  {} Player {}: ${} chips", player_name, player.id, player.chips);
                    }
                    
                    game_state.set(GameState::Setup);
                    controller.state_timer = Timer::from_seconds(2.0, TimerMode::Once);
                    controller.state_timer.reset();
                }
            },
        }
    }
}

fn determine_winner(players: &mut Query<&mut Player>, game_data: &GameData, game_position: &mut GamePosition) {
    let mut evaluations = Vec::new();
    
    // Evaluate each active player's hand
    for player in players.iter() {
        if !player.has_folded && !player.hole_cards.is_empty() {
            let evaluation = evaluate_hand(&player.hole_cards, &game_data.community_cards);
            evaluations.push((player.id, evaluation, player.player_type));
        }
    }
    
    if evaluations.is_empty() {
        info!("No active players for showdown");
        return;
    }
    
    // Sort by hand strength (best first)
    evaluations.sort_by(|(_, eval_a, _), (_, eval_b, _)| eval_b.cmp(eval_a));
    
    // Log all hands
    info!("=== SHOWDOWN ===");
    for (player_id, evaluation, player_type) in &evaluations {
        let player_name = match player_type {
            PlayerType::Human => "Human",
            PlayerType::AI => "AI",
        };
        info!(
            "{} Player {}: {} (Primary: {}, Secondary: {})",
            player_name,
            player_id,
            hand_rank_name(&evaluation.rank),
            evaluation.primary_value,
            evaluation.secondary_value
        );
    }
    
    // Winner is first in sorted list
    let (winner_id, winner_evaluation, winner_type) = &evaluations[0];
    let winner_name = match winner_type {
        PlayerType::Human => "Human",
        PlayerType::AI => "AI",
    };
    
    // CRITICAL FIX: Actually transfer chips to winner!
    for mut player in players.iter_mut() {
        if player.id == *winner_id {
            player.chips += game_data.pot;
            info!(
                "💰 CHIPS TRANSFERRED: {} Player {} receives ${} (new total: ${})",
                winner_name,
                winner_id,
                game_data.pot,
                player.chips
            );
            break;
        }
    }
    
    info!(
        "🏆 WINNER: {} Player {} with {} wins pot of ${}!",
        winner_name,
        winner_id,
        hand_rank_name(&winner_evaluation.rank),
        game_data.pot
    );
    
    // Advance dealer button for next hand
    game_position.advance_dealer_button();
}

// System to display current game state in console
pub fn debug_game_state(
    current_state: Res<State<GameState>>,
    game_data: Res<GameData>,
    players: Query<&Player>,
) {
    // Only run when state changes
    if current_state.is_changed() {
        info!("=== GAME STATE: {:?} ===", current_state.get());
        info!("Pot: ${}", game_data.pot);
        info!("Community Cards: {}", game_data.community_cards.len());
        info!("Current Player: {}", game_data.current_player);
        
        for player in players.iter() {
            let player_type = match player.player_type {
                PlayerType::Human => "Human",
                PlayerType::AI => "AI",
            };
            info!(
                "{} Player {}: ${} chips, {} cards, bet: ${}, folded: {}",
                player_type,
                player.id,
                player.chips,
                player.hole_cards.len(),
                player.current_bet,
                player.has_folded
            );
        }
        info!("================");
    }
}

// System to pause/resume game controller
pub fn toggle_auto_advance(
    input: Res<ButtonInput<KeyCode>>,
    mut controller: ResMut<GameController>,
) {
    if input.just_pressed(KeyCode::Space) {
        controller.auto_advance = !controller.auto_advance;
        if controller.auto_advance {
            info!("Game auto-advance ENABLED (press SPACE to pause)");
        } else {
            info!("Game auto-advance PAUSED (press SPACE to resume)");
        }
    }
}

// Helper function to post blinds at the start of each hand
fn post_blinds(
    players: &mut Query<&mut Player>,
    game_position: &GamePosition,
    game_data: &mut GameData,
) {
    let small_blind_player = game_position.get_small_blind_player();
    let big_blind_player = game_position.get_big_blind_player();
    
    info!("💰 Posting blinds - SB: Player {} ({}), BB: Player {} ({})", 
          small_blind_player, game_position.small_blind_amount,
          big_blind_player, game_position.big_blind_amount);
    
    // Post small blind
    for mut player in players.iter_mut() {
        if player.id == small_blind_player {
            let blind_amount = game_position.small_blind_amount.min(player.chips);
            player.chips = player.chips.saturating_sub(blind_amount);
            game_data.pot += blind_amount;
            info!("🔸 Player {} posts small blind: {} chips (remaining: {})", 
                  player.id, blind_amount, player.chips);
            break;
        }
    }
    
    // Post big blind
    for mut player in players.iter_mut() {
        if player.id == big_blind_player {
            let blind_amount = game_position.big_blind_amount.min(player.chips);
            player.chips = player.chips.saturating_sub(blind_amount);
            game_data.pot += blind_amount;
            info!("🔹 Player {} posts big blind: {} chips (remaining: {})", 
                  player.id, blind_amount, player.chips);
            break;
        }
    }
    
    info!("💰 Total pot after blinds: {} chips", game_data.pot);
}
