use bevy::prelude::*;
use bevy::ui::{node_bundles::{NodeBundle, TextBundle}, Style};
use bevy::text::TextStyle;
use crate::game_state::GameState;
use crate::player::{Player, PlayerType};

// Teaching system components
#[derive(Component)]
pub struct TeachingMessageDisplay;

#[derive(Component)]
pub struct HandAnalysisDisplay;

// Resource to track teaching state
#[derive(Resource)]
pub struct TeachingState {
    pub tutorial_mode: bool,
    pub show_hand_rankings: bool,
    pub show_rule_popup: bool,
    pub current_explanation: Option<String>,
    pub mistakes_shown: Vec<String>,
    pub last_game_state: Option<GameState>,
    pub last_current_player: Option<u32>,
    pub messages_shown_this_state: std::collections::HashSet<String>,
}

impl Default for TeachingState {
    fn default() -> Self {
        Self {
            tutorial_mode: true, // Enable by default for learning
            show_hand_rankings: false,
            show_rule_popup: false,
            current_explanation: None,
            mistakes_shown: Vec::new(),
            last_game_state: None,
            last_current_player: None,
            messages_shown_this_state: std::collections::HashSet::new(),
        }
    }
}

// Types of explanations available
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ExplanationType {
    HandRanking(String),
    BettingRule(String),
    GamePhase(String),
    PlayerAction(String),
    Mistake(String),
}

impl TeachingState {
    pub fn show_explanation(&mut self, explanation: ExplanationType) {
        match explanation {
            ExplanationType::HandRanking(msg) => {
                self.current_explanation = Some(format!("🃏 Hand Ranking: {}", msg));
                self.show_rule_popup = true;
            },
            ExplanationType::BettingRule(msg) => {
                self.current_explanation = Some(format!("💰 Betting Rule: {}", msg));
                self.show_rule_popup = true;
            },
            ExplanationType::GamePhase(msg) => {
                self.current_explanation = Some(format!("🎮 Game Phase: {}", msg));
                self.show_rule_popup = true;
            },
            ExplanationType::PlayerAction(msg) => {
                self.current_explanation = Some(format!("🎯 Player Action: {}", msg));
                self.show_rule_popup = true;
            },
            ExplanationType::Mistake(msg) => {
                if !self.mistakes_shown.contains(&msg) {
                    self.current_explanation = Some(format!("⚠️ Learning Tip: {}", msg));
                    self.show_rule_popup = true;
                    self.mistakes_shown.push(msg);
                }
            },
        }
        // Keep a log entry for reference but now we'll also update UI
        info!("📚 Teaching: {}", self.current_explanation.as_ref().unwrap_or(&"No explanation".to_string()));
    }
}

// System to setup teaching UI
pub fn setup_teaching_ui(mut commands: Commands) {
    info!("📚 Setting up teaching UI systems");
    
    // Teaching message display (bottom left corner)
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(2.0),
                bottom: Val::Percent(2.0),
                width: Val::Percent(45.0),
                min_height: Val::Percent(10.0),
                max_height: Val::Percent(25.0),
                padding: UiRect::all(Val::Percent(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.2, 0.4, 0.8).into(),
            border_color: Color::rgba(0.3, 0.4, 0.6, 1.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                        ..default()
                    },
                ))
                .insert(TeachingMessageDisplay);
        });
    
    // Hand analysis display (bottom right corner)
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(2.0),
                bottom: Val::Percent(2.0),
                width: Val::Percent(45.0),
                min_height: Val::Percent(10.0),
                max_height: Val::Percent(25.0),
                padding: UiRect::all(Val::Percent(2.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.4, 0.1, 0.75).into(),
            border_color: Color::rgba(0.2, 0.6, 0.2, 1.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                        ..default()
                    },
                ))
                .insert(HandAnalysisDisplay);
        });
    
    info!("🎓 TEACHING CONTROLS:");
    info!("   T - Toggle tutorial mode (explanations)");
    info!("   H - Toggle hand rankings guide");
    info!("   R - Show basic betting rules");
    info!("   ESC - Hide current explanation");
    info!("📖 Tutorial mode is ON - you'll get explanations during play!");
    
    info!("🎮 PHASE 7 GAME CONTROLS:");
    info!("   SPACE - Pause/resume auto-advance");
    info!("   P - Pause/resume game");
    info!("   , (comma) - Slow down game speed");
    info!("   . (period) - Speed up game speed");
    info!("   1 - Reset speed to normal (1.0x)");
    info!("   M - Mute/unmute audio");
    info!("   + - Increase volume");
    info!("   - - Decrease volume");
}

// System to update teaching message display
pub fn update_teaching_display(
    teaching_state: Res<TeachingState>,
    mut teaching_display_query: Query<&mut Text, With<TeachingMessageDisplay>>,
) {
    if let Ok(mut text) = teaching_display_query.get_single_mut() {
        if teaching_state.tutorial_mode && teaching_state.show_rule_popup {
            if let Some(ref explanation) = teaching_state.current_explanation {
                text.sections[0].value = explanation.clone();
            }
        } else {
            text.sections[0].value = "".to_string();
        }
    }
}



// System to provide contextual explanations during gameplay
pub fn provide_contextual_explanations(
    current_state: Res<State<GameState>>,
    mut teaching_state: ResMut<TeachingState>,
    players: Query<&Player>,
) {
    if !teaching_state.tutorial_mode {
        return;
    }
    
    // Only trigger explanations when state changes
    if !current_state.is_changed() {
        return;
    }
    
    match current_state.get() {
        GameState::Setup => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "Game is starting! Each player gets 2 hole cards and starts with chips.".to_string()
            ));
        },
        GameState::Dealing => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "Dealing phase - Each player receives 2 private cards. Small and big blinds are posted.".to_string()
            ));
        },
        GameState::PreFlop => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "Pre-Flop betting - Make decisions based only on your 2 hole cards. Big blind sets minimum bet.".to_string()
            ));
        },
        GameState::Flop => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "Flop - 3 community cards revealed! Now you can make poker hands with 5 cards total.".to_string()
            ));
        },
        GameState::Turn => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "Turn - 4th community card revealed. Your hand possibilities are becoming clearer.".to_string()
            ));
        },
        GameState::River => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "River - Final community card! This is your last chance to bet with complete information.".to_string()
            ));
        },
        GameState::Showdown => {
            teaching_state.show_explanation(ExplanationType::GamePhase(
                "Showdown - All remaining players reveal cards. Best 5-card poker hand wins the pot!".to_string()
            ));
        },
        GameState::GameOver => {
            let remaining_players = players.iter().filter(|p| p.chips > 0).count();
            if remaining_players <= 1 {
                teaching_state.show_explanation(ExplanationType::GamePhase(
                    "Game Over - Only one player has chips remaining! They are the winner.".to_string()
                ));
            }
        }
    }
}

// System to explain hand rankings when requested
pub fn explain_hand_rankings(
    teaching_state: Res<TeachingState>,
) {
    if teaching_state.show_hand_rankings && teaching_state.is_changed() {
        info!("🃏 POKER HAND RANKINGS (Best to Worst):");
        info!("   1. Royal Flush - A, K, Q, J, 10 all same suit");
        info!("   2. Straight Flush - 5 consecutive cards, same suit");
        info!("   3. Four of a Kind - 4 cards of same rank");
        info!("   4. Full House - 3 of a kind + pair");
        info!("   5. Flush - 5 cards of same suit");
        info!("   6. Straight - 5 consecutive cards");
        info!("   7. Three of a Kind - 3 cards of same rank");
        info!("   8. Two Pair - 2 pairs of different ranks");
        info!("   9. One Pair - 2 cards of same rank");
        info!("   10. High Card - No matching cards");
    }
}

// System to highlight valid actions for learning
pub fn highlight_valid_actions(
    mut teaching_state: ResMut<TeachingState>,
    current_state: Res<State<GameState>>,
    game_data: Res<crate::game_state::GameData>,
    players: Query<&Player>,
    mut hand_analysis_query: Query<&mut Text, With<HandAnalysisDisplay>>,
) {
    if !teaching_state.tutorial_mode {
        // Clear display if tutorial mode is off
        if let Ok(mut text) = hand_analysis_query.get_single_mut() {
            text.sections[0].value = "".to_string();
        }
        return;
    }
    
    // Check if state has changed or current player has changed
    let current_game_state = current_state.get().clone();
    let state_changed = teaching_state.last_game_state != Some(current_game_state);
    let player_changed = teaching_state.last_current_player != Some(game_data.current_player);
    
    if !state_changed && !player_changed {
        return; // Don't spam the same message
    }
    
    // Update tracked state
    teaching_state.last_game_state = Some(current_game_state);
    teaching_state.last_current_player = Some(game_data.current_player);
    
    // Clear messages for new state
    if state_changed {
        teaching_state.messages_shown_this_state.clear();
    }
    
    // Find human player
    if let Ok(human_player) = players.iter().find(|p| matches!(p.player_type, PlayerType::Human)).ok_or("No human player") {
        match current_state.get() {
            GameState::PreFlop | GameState::Flop | GameState::Turn | GameState::River => {
                if !human_player.has_folded && human_player.chips > 0 && game_data.current_player == 0 {
                    let message = "💡 Your Turn!\nOptions:\n• FOLD - Quit this hand\n• CHECK/CALL - Match current bet\n• RAISE - Increase the bet".to_string();
                    
                    if !teaching_state.messages_shown_this_state.contains(&message) {
                        // Update UI display
                        if let Ok(mut text) = hand_analysis_query.get_single_mut() {
                            text.sections[0].value = message.clone();
                        }
                        
                        // Keep log for reference
                        info!("💡 Your options: FOLD (quit hand), CHECK/CALL (match bet), RAISE (increase bet)");
                        teaching_state.messages_shown_this_state.insert(message);
                    }
                } else if let Ok(mut text) = hand_analysis_query.get_single_mut() {
                    // Clear display when it's not human player's turn
                    text.sections[0].value = "".to_string();
                }
            },
            _ => {
                // Clear display for non-betting phases
                if let Ok(mut text) = hand_analysis_query.get_single_mut() {
                    text.sections[0].value = "".to_string();
                }
            }
        }
    }
}

// System to provide hand analysis and suggestions
pub fn provide_hand_analysis(
    teaching_state: Res<TeachingState>,
    current_state: Res<State<GameState>>,
    _game_data: Res<crate::game_state::GameData>,
    betting_round: Res<crate::betting::BettingRound>,
    players: Query<&Player>,
    mut hand_analysis_query: Query<&mut Text, With<HandAnalysisDisplay>>,
) {
    if !teaching_state.tutorial_mode {
        return;
    }
    
    // Only update when state changes
    if !current_state.is_changed() {
        return;
    }
    
    // Find human player
    if let Ok(human_player) = players.iter().find(|p| matches!(p.player_type, PlayerType::Human)).ok_or("No human player") {
        match current_state.get() {
            GameState::PreFlop => {
                if !human_player.hole_cards.is_empty() {
                    let analysis = analyze_starting_hand_ui(&human_player.hole_cards, &betting_round);
                    if let Ok(mut text) = hand_analysis_query.get_single_mut() {
                        text.sections[0].value = analysis;
                    }
                }
            },
            GameState::Flop | GameState::Turn | GameState::River => {
                // For now, clear the hand analysis during later phases
                // We could add more detailed analysis here later
                if let Ok(mut text) = hand_analysis_query.get_single_mut() {
                    text.sections[0].value = "📊 Community cards revealed!\nAnalyze how they improve\nyour hand strength.".to_string();
                }
            },
            _ => {
                // Clear analysis display for other phases
                if let Ok(mut text) = hand_analysis_query.get_single_mut() {
                    text.sections[0].value = "".to_string();
                }
            }
        }
    }
}

// Helper function to analyze starting hand strength for UI display
fn analyze_starting_hand_ui(hole_cards: &[crate::cards::Card], _betting_round: &crate::betting::BettingRound) -> String {
    if hole_cards.len() != 2 {
        return "🃏 Hand Analysis:\nWaiting for cards...".to_string();
    }
    
    let card1 = &hole_cards[0];
    let card2 = &hole_cards[1];
    
    // Check for pocket pairs  
    if card1.rank == card2.rank {
        match card1.rank {
            crate::cards::Rank::Ace | crate::cards::Rank::King | crate::cards::Rank::Queen | crate::cards::Rank::Jack => {
                return format!("🔥 EXCELLENT!\nPocket {}s\nPremium starting hand!\nConsider raising.", get_rank_name(card1.rank));
            },
            crate::cards::Rank::Ten | crate::cards::Rank::Nine | crate::cards::Rank::Eight => {
                return format!("👍 GOOD!\nPocket {}s\nSolid hand - you can\nraise or call confidently.", get_rank_name(card1.rank));
            },
            _ => {
                return format!("📖 Pocket {}s\nSmall pairs can be tricky.\nConsider the betting action.", get_rank_name(card1.rank));
            }
        }
    }
    
    // Check for high cards
    let high_rank = if card1.rank > card2.rank { card1.rank } else { card2.rank };
    let low_rank = if card1.rank < card2.rank { card1.rank } else { card2.rank };
    let suited = card1.suit == card2.suit;
    
    if high_rank == crate::cards::Rank::Ace { // Ace
        if low_rank >= crate::cards::Rank::Ten {
            return format!("🔥 EXCELLENT!\nAce-{} {}\nPremium hand!\nStrong raise or call.", 
                  get_rank_name(low_rank), if suited { "suited" } else { "offsuit" });
        } else if low_rank >= crate::cards::Rank::Seven {
            return format!("👍 GOOD!\nAce-{} {}\nPlayable hand.\nConsider position & betting.", 
                  get_rank_name(low_rank), if suited { "suited" } else { "offsuit" });
        } else {
            return format!("⚠️ MARGINAL\nAce-{} {}\nWeak hand - be careful\nwith heavy betting.", 
                  get_rank_name(low_rank), if suited { "suited" } else { "offsuit" });
        }
    } else if high_rank >= crate::cards::Rank::Queen && low_rank >= crate::cards::Rank::Ten { // Face cards with 10+
        return format!("👍 GOOD!\n{}-{} {}\nSolid hand for\nmost situations.", 
              get_rank_name(high_rank), get_rank_name(low_rank), if suited { "suited" } else { "offsuit" });
    } else if suited && (rank_value(high_rank) - rank_value(low_rank) <= 4) {
        return format!("📖 {}-{} suited\nPotential for straights\nand flushes.\nPlay cautiously.", 
              get_rank_name(high_rank), get_rank_name(low_rank));
    } else {
        return format!("⚠️ WEAK\n{}-{} {}\nMarginal hand.\nConsider folding to\nheavy betting.", 
              get_rank_name(high_rank), get_rank_name(low_rank), if suited { "suited" } else { "offsuit" });
    }
}

// Helper function to get rank name for display
fn get_rank_name(rank: crate::cards::Rank) -> &'static str {
    match rank {
        crate::cards::Rank::Ace => "Ace",
        crate::cards::Rank::King => "King", 
        crate::cards::Rank::Queen => "Queen",
        crate::cards::Rank::Jack => "Jack",
        crate::cards::Rank::Ten => "Ten",
        crate::cards::Rank::Nine => "Nine",
        crate::cards::Rank::Eight => "Eight", 
        crate::cards::Rank::Seven => "Seven",
        crate::cards::Rank::Six => "Six",
        crate::cards::Rank::Five => "Five",
        crate::cards::Rank::Four => "Four",
        crate::cards::Rank::Three => "Three",
        crate::cards::Rank::Two => "Two",
    }
}

// Helper function to get numeric rank value
fn rank_value(rank: crate::cards::Rank) -> u8 {
    rank as u8
}
