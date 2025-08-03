use bevy::prelude::*;
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

#[derive(Debug, Clone, PartialEq)]
pub enum BettingButtonAction {
    Fold,
    Check,
    Call,
    Raise,
}

#[derive(Component)]
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
            raise_amount: 20, // Default raise amount
        }
    }
}

// Button colors
const BUTTON_NORMAL: Color = Color::srgb(0.3, 0.3, 0.3);
const BUTTON_HOVER: Color = Color::srgb(0.4, 0.4, 0.4);
const BUTTON_PRESSED: Color = Color::srgb(0.2, 0.2, 0.2);
const FOLD_BUTTON_COLOR: Color = Color::srgb(0.7, 0.2, 0.2);
const CALL_BUTTON_COLOR: Color = Color::srgb(0.2, 0.6, 0.2);
const RAISE_BUTTON_COLOR: Color = Color::srgb(0.2, 0.4, 0.7);

pub fn setup_betting_ui(mut commands: Commands) {
    // Betting panel for human player (bottom center)
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(25.0),
                bottom: Val::Px(80.0),
                width: Val::Percent(50.0),
                height: Val::Px(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            visibility: Visibility::Hidden, // Hidden by default
            ..default()
        })
        .with_children(|parent| {
            // Fold Button
            create_betting_button(
                parent, 
                "FOLD", 
                BettingButtonAction::Fold, 
                FOLD_BUTTON_COLOR
            );
            
            // Check/Call Button (text will be updated dynamically)
            create_betting_button(
                parent, 
                "CHECK", 
                BettingButtonAction::Check, 
                CALL_BUTTON_COLOR
            );
            
            // Raise Section
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(150.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|raise_parent| {
                    // Raise Amount Display
                    raise_parent
                        .spawn(TextBundle::from_section(
                            "Raise: $20",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ))
                        .insert(RaiseAmountDisplay);
                    
                    // Raise Slider (simplified as buttons for now)
                    raise_parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|slider_parent| {
                            // Decrease button
                            create_small_button(slider_parent, "-", "decrease");
                            
                            // Increase button  
                            create_small_button(slider_parent, "+", "increase");
                        });
                    
                    // Raise Button
                    create_betting_button(
                        raise_parent, 
                        "RAISE", 
                        BettingButtonAction::Raise, 
                        RAISE_BUTTON_COLOR
                    );
                });
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
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(BettingButton { action });
}

fn create_small_button(parent: &mut ChildBuilder, text: &str, id: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(25.0),
                height: Val::Px(25.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BUTTON_NORMAL.into(),
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 12.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(Name::new(id.to_string()));
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

// System to handle betting button interactions
pub fn handle_betting_buttons(
    mut interaction_query: Query<
        (&Interaction, &BettingButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut human_input: ResMut<HumanPlayerInput>,
    betting_round: Res<BettingRound>,
    players: Query<&Player>,
) {
    // Find current human player using betting round
    let current_human_player = if let Some(current_id) = betting_round.peek_next_player() {
        players
            .iter()
            .find(|p| p.id == current_id && matches!(p.player_type, PlayerType::Human))
    } else {
        None
    };
    
    let Some(human_player) = current_human_player else {
        return;
    };
    
    for (interaction, betting_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED.into();
                
                // Process the betting action
                let action = match betting_button.action {
                    BettingButtonAction::Fold => Some(PlayerAction::Fold),
                    BettingButtonAction::Check => {
                        // Check if we can actually check (no bet to call)
                        let call_amount = betting_round.current_bet.saturating_sub(human_player.current_bet);
                        if call_amount == 0 {
                            Some(PlayerAction::Check)
                        } else {
                            Some(PlayerAction::Call) // Convert to call if there's a bet
                        }
                    },
                    BettingButtonAction::Call => Some(PlayerAction::Call),
                    BettingButtonAction::Raise => {
                        Some(PlayerAction::Raise(human_input.raise_amount))
                    },
                };
                
                if let Some(action) = action {
                    info!("Human player selected action: {:?}", action);
                    human_input.pending_action = Some(action);
                }
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER.into();
            }
            Interaction::None => {
                // Reset to appropriate color based on button type
                let base_color = match betting_button.action {
                    BettingButtonAction::Fold => FOLD_BUTTON_COLOR,
                    BettingButtonAction::Check | BettingButtonAction::Call => CALL_BUTTON_COLOR,
                    BettingButtonAction::Raise => RAISE_BUTTON_COLOR,
                };
                *color = base_color.into();
            }
        }
    }
}

// System to handle raise amount adjustment
pub fn handle_raise_adjustment(
    mut interaction_query: Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
    mut human_input: ResMut<HumanPlayerInput>,
    mut amount_display_query: Query<&mut Text, With<RaiseAmountDisplay>>,
    betting_round: Res<BettingRound>,
) {
    for (interaction, name) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            match name.as_str() {
                "increase" => {
                    human_input.raise_amount = (human_input.raise_amount + betting_round.min_raise).min(1000);
                }
                "decrease" => {
                    human_input.raise_amount = human_input.raise_amount.saturating_sub(betting_round.min_raise).max(betting_round.min_raise);
                }
                _ => continue,
            }
            
            // Update display
            if let Ok(mut text) = amount_display_query.get_single_mut() {
                text.sections[0].value = format!("Raise: ${}", human_input.raise_amount);
            }
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
