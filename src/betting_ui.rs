use bevy::prelude::*;
use bevy::ui::{node_bundles::{NodeBundle, ButtonBundle}, Style};
use bevy::text::TextStyle;
use bevy::hierarchy::ChildBuilder;
use crate::player::{Player, PlayerType};
use crate::betting::{PlayerAction, BettingRound};
use crate::game_state::GameState;

// Betting UI Components
#[derive(Component)]
pub struct BettingUI;

#[derive(Component)]
pub struct BettingButton {
    pub action: BettingButtonAction,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum BettingButtonAction {
    Fold,
    #[allow(dead_code)]
    Check,
    Call,
    Raise,
    IncreaseRaise,
    DecreaseRaise,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct RaiseSlider;

#[derive(Component)]
pub struct RaiseAmountDisplay;

#[derive(Resource)]
pub struct HumanPlayerInput {
    pub pending_action: Option<PlayerAction>,
    pub raise_amount: u32,
}

impl Default for HumanPlayerInput {
    fn default() -> Self {
        Self {
            pending_action: None,
            raise_amount: 20,
        }
    }
}

// Button colors
const BUTTON_NORMAL: Color = Color::rgb(0.3, 0.3, 0.3);
#[allow(dead_code)]
const BUTTON_HOVER: Color = Color::rgb(0.4, 0.4, 0.4);
#[allow(dead_code)]
const BUTTON_PRESSED: Color = Color::rgb(0.2, 0.2, 0.2);
const FOLD_BUTTON_COLOR: Color = Color::rgb(0.7, 0.2, 0.2);
const CALL_BUTTON_COLOR: Color = Color::rgb(0.2, 0.6, 0.2);
const RAISE_BUTTON_COLOR: Color = Color::rgb(0.2, 0.4, 0.7);

pub fn setup_betting_ui(mut commands: Commands) {
    // Betting panel for human player (bottom center, moved higher to avoid cards)
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(120.0),
                left: Val::Percent(50.0),
                width: Val::Px(400.0),
                height: Val::Px(80.0),
                margin: UiRect::left(Val::Px(-200.0)), // Center horizontally
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
            ..default()
        })
        .with_children(|parent| {
            // Fold button
            create_betting_button(parent, "FOLD", BettingButtonAction::Fold, FOLD_BUTTON_COLOR);
            
            // Call/Check button  
            create_betting_button(parent, "CALL", BettingButtonAction::Call, CALL_BUTTON_COLOR);
            
            // Raise controls container
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|raise_parent| {
                // Raise amount display
                raise_parent.spawn((
                    TextBundle::from_section(
                        "Raise: $20",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    RaiseAmountDisplay,
                ));
                
                // Raise amount controls
                raise_parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(25.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|controls| {
                    create_small_button(controls, "-", BettingButtonAction::DecreaseRaise);
                    create_small_button(controls, "+", BettingButtonAction::IncreaseRaise);
                });
            });
            
            // Raise button
            create_betting_button(parent, "RAISE", BettingButtonAction::Raise, RAISE_BUTTON_COLOR);
        })
        .insert(BettingUI);
}

fn create_betting_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: BettingButtonAction,
    color: Color,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(BettingButton { action: action.clone() })
        .insert(action); // Also insert the action as a separate component for easier querying
}

fn create_small_button(parent: &mut ChildBuilder, text: &str, action: BettingButtonAction) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(70.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: BUTTON_NORMAL.into(),
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(BettingButton { action: action.clone() })
        .insert(action); // Also insert the action as a separate component for easier querying
}

// System to show/hide betting UI based on game state and current player
pub fn manage_betting_ui_visibility(
    mut betting_ui_query: Query<&mut Visibility, With<BettingUI>>,
    players: Query<&Player>,
    game_state: Res<State<GameState>>,
    betting_round: Res<BettingRound>,
) {
    if let Ok(mut visibility) = betting_ui_query.get_single_mut() {
        // Show betting UI if:
        // 1. We're in a betting phase
        // 2. Current player is human 
        // 3. Betting is not complete
        let should_show = matches!(
            game_state.get(),
            GameState::PreFlop | GameState::Flop | GameState::Turn | GameState::River
        ) && !betting_round.betting_complete;
        
        if should_show {
            // Check if current player is human using betting round's next player
            if let Some(current_player_id) = betting_round.peek_next_player() {
                if let Some(current_player) = players.iter().find(|p| p.id == current_player_id) {
                    if matches!(current_player.player_type, PlayerType::Human) && !current_player.has_folded {
                        *visibility = Visibility::Visible;
                        return;
                    }
                }
            }
        }
        
        *visibility = Visibility::Hidden;
    }
}



// System to update the raise amount display
pub fn update_raise_amount_display(
    mut amount_display_query: Query<&mut Text, With<RaiseAmountDisplay>>,
    human_input: Res<HumanPlayerInput>,
) {
    if human_input.is_changed() {
        for mut text in &mut amount_display_query {
            text.sections[0].value = format!("Raise: ${}", human_input.raise_amount);
        }
    }
}

// System to update betting button text based on current situation
pub fn update_betting_button_text(
    mut button_query: Query<(&BettingButton, &Children)>,
    mut text_query: Query<&mut Text>,
    betting_round: Res<BettingRound>,
    players: Query<&Player>,
    game_data: Res<crate::game_state::GameData>,
) {
    // Find current human player
    let current_human_player = players
        .iter()
        .find(|p| p.id == game_data.current_player && matches!(p.player_type, PlayerType::Human));
    
    let Some(human_player) = current_human_player else {
        return;
    };
    
    let call_amount = betting_round.current_bet.saturating_sub(human_player.current_bet);
    
    for (betting_button, children) in &mut button_query {
        if let BettingButtonAction::Check = betting_button.action {
            // Update Check/Call button text
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if call_amount == 0 {
                        text.sections[0].value = "CHECK".to_string();
                    } else {
                        text.sections[0].value = format!("CALL ${}", call_amount);
                    }
                }
            }
        }
    }
}

// System to reset raise amount when a new hand starts
pub fn reset_raise_amount_on_new_hand(
    mut human_input: ResMut<HumanPlayerInput>,
    current_state: Res<State<GameState>>,
) {
    // Reset raise amount when dealing starts (new hand)
    if current_state.is_changed() && *current_state == GameState::Dealing {
        human_input.raise_amount = 20; // Reset to default
        human_input.pending_action = None; // Clear any pending action
        info!("🔄 Reset raise amount to default ($20) for new hand");
    }
}
