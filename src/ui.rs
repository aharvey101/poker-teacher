#![allow(dead_code)]
use bevy::prelude::*;
use crate::player::{Player, PlayerType};
use crate::game_state::{GameData, GameState};

// UI Components
#[derive(Component)]
pub struct PlayerUI {
    pub player_id: u32,
}

#[derive(Component)]
pub struct GameInfoUI;

#[derive(Component)]
pub struct PotDisplay;

#[derive(Component)]
pub struct GamePhaseDisplay;

// Colors for UI
const UI_BACKGROUND: Color = Color::rgba(0.1, 0.1, 0.1, 0.8);
const UI_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const HUMAN_PLAYER_COLOR: Color = Color::rgb(0.2, 0.6, 0.2);
const AI_PLAYER_COLOR: Color = Color::rgb(0.6, 0.6, 0.2);

pub fn setup_ui(mut commands: Commands) {
    // Game info panel (top center)
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(35.0),
                top: Val::Percent(2.0),
                width: Val::Percent(30.0),
                height: Val::Percent(10.0),
                padding: UiRect::all(Val::Percent(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: UI_BACKGROUND.into(),
            ..default()
        })
        .with_children(|parent| {
            // Pot display
            parent
                .spawn(TextBundle::from_section(
                    "Pot: $0",
                    TextStyle {
                        font_size: 20.0,
                        color: UI_TEXT_COLOR,
                        ..default()
                    },
                ))
                .insert(PotDisplay);
            
            // Game phase display
            parent
                .spawn(TextBundle::from_section(
                    "Setup",
                    TextStyle {
                        font_size: 16.0,
                        color: UI_TEXT_COLOR,
                        ..default()
                    },
                ))
                .insert(GamePhaseDisplay);
        })
        .insert(GameInfoUI);
}

pub fn setup_player_ui(
    mut commands: Commands,
    players: Query<&Player>,
    existing_ui: Query<Entity, With<PlayerUI>>,
    game_state: Res<State<GameState>>,
) {
    // Only update when game state changes or player data might have changed
    if !game_state.is_changed() {
        return;
    }
    
    // Remove existing player UI only when something changed
    for entity in existing_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Create UI for each player
    for player in players.iter() {
        let (ui_position, ui_color) = match player.player_type {
            PlayerType::Human => {
                // Human player UI at bottom
                (
                    Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(35.0),
                        bottom: Val::Percent(2.0),
                        width: Val::Percent(30.0),
                        height: Val::Percent(10.0),
                        ..default()
                    },
                    HUMAN_PLAYER_COLOR
                )
            },
            PlayerType::AI => {
                // AI players at top sides
                let (left_percent, top_percent) = if player.id == 1 {
                    (5.0, 15.0) // Top left
                } else {
                    (70.0, 15.0) // Top right
                };
                
                (
                    Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(left_percent),
                        top: Val::Percent(top_percent),
                        width: Val::Percent(25.0),
                        height: Val::Percent(10.0),
                        ..default()
                    },
                    AI_PLAYER_COLOR
                )
            }
        };
        
        commands
            .spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Percent(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..ui_position
                },
                background_color: BackgroundColor(ui_color.with_a(0.8)),
                ..default()
            })
            .with_children(|parent| {
                // Player name
                let player_name = match player.player_type {
                    PlayerType::Human => "You",
                    PlayerType::AI => &format!("AI Player {}", player.id),
                };
                
                parent.spawn(TextBundle::from_section(
                    player_name,
                    TextStyle {
                        font_size: 16.0,
                        color: UI_TEXT_COLOR,
                        ..default()
                    },
                ));
                
                // Chip count
                parent.spawn(TextBundle::from_section(
                    format!("Chips: ${}", player.chips),
                    TextStyle {
                        font_size: 14.0,
                        color: UI_TEXT_COLOR,
                        ..default()
                    },
                ));
                
                // Current bet (if any)
                if player.current_bet > 0 {
                    parent.spawn(TextBundle::from_section(
                        format!("Bet: ${}", player.current_bet),
                        TextStyle {
                            font_size: 12.0,
                            color: Color::rgb(0.8, 0.8, 0.2),
                            ..default()
                        },
                    ));
                }
                
                // Folded status
                if player.has_folded {
                    parent.spawn(TextBundle::from_section(
                        "FOLDED",
                        TextStyle {
                            font_size: 12.0,
                            color: Color::rgb(0.8, 0.2, 0.2),
                            ..default()
                        },
                    ));
                }
            })
            .insert(PlayerUI { player_id: player.id });
    }
}

pub fn update_pot_display(
    mut pot_query: Query<&mut Text, With<PotDisplay>>,
    game_data: Res<GameData>,
) {
    if let Ok(mut text) = pot_query.get_single_mut() {
        text.sections[0].value = format!("Pot: ${}", game_data.pot);
    }
}

pub fn update_game_phase_display(
    mut phase_query: Query<&mut Text, With<GamePhaseDisplay>>,
    game_state: Res<State<GameState>>,
) {
    if let Ok(mut text) = phase_query.get_single_mut() {
        let phase_text = match game_state.get() {
            GameState::Setup => "Setup",
            GameState::Dealing => "Dealing Cards",
            GameState::PreFlop => "Pre-Flop",
            GameState::Flop => "Flop",
            GameState::Turn => "Turn",
            GameState::River => "River", 
            GameState::Showdown => "Showdown",
            GameState::GameOver => "Game Over",
        };
        text.sections[0].value = phase_text.to_string();
    }
}

pub fn update_player_ui(
    mut player_ui_query: Query<(&PlayerUI, &mut BackgroundColor, &Children)>,
    players: Query<&Player>,
    mut text_query: Query<&mut Text>,
    game_data: Res<GameData>,
) {
    for (player_ui, mut bg_color, children) in player_ui_query.iter_mut() {
        if let Some(player) = players.iter().find(|p| p.id == player_ui.player_id) {
            // Highlight current player
            let is_current_player = game_data.current_player == player.id;
            let base_color = match player.player_type {
                PlayerType::Human => HUMAN_PLAYER_COLOR,
                PlayerType::AI => AI_PLAYER_COLOR,
            };
            
            let alpha = if is_current_player { 1.0 } else { 0.6 };
            *bg_color = BackgroundColor(base_color.with_a(alpha));
            
            // Update text displays
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    // Update chip count, bet amount, etc.
                    if text.sections[0].value.starts_with("Chips:") {
                        text.sections[0].value = format!("Chips: ${}", player.chips);
                    } else if text.sections[0].value.starts_with("Bet:") && player.current_bet > 0 {
                        text.sections[0].value = format!("Bet: ${}", player.current_bet);
                    }
                }
            }
        }
    }
}
