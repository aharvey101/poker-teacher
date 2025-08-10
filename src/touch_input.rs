use bevy::prelude::*;
use bevy::input::touch::TouchPhase;

use crate::betting_ui::{BettingButtonAction, BettingButton, HumanPlayerInput};
use crate::betting::PlayerAction;
use crate::haptics::HapticFeedbackEvent;

impl From<BettingButtonAction> for PlayerAction {
    fn from(action: BettingButtonAction) -> Self {
        match action {
            BettingButtonAction::Fold => PlayerAction::Fold,
            BettingButtonAction::Check => PlayerAction::Check,
            BettingButtonAction::Call => PlayerAction::Call,
            BettingButtonAction::Raise => PlayerAction::Raise(20), // Default raise amount, will be updated below
            BettingButtonAction::IncreaseRaise => PlayerAction::Raise(0), // Placeholder, handled separately
            BettingButtonAction::DecreaseRaise => PlayerAction::Raise(0), // Placeholder, handled separately
        }
    }
}

// Unified input system that handles both touch and mouse input
pub fn handle_unified_input(
    mut touch_events: EventReader<TouchInput>,
    mut human_input: ResMut<HumanPlayerInput>,
    // Query for mouse/interaction events
    interaction_query: Query<(&BettingButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    // Query for touch events (all buttons)
    all_button_query: Query<(&Node, &GlobalTransform, &BettingButton), With<Button>>,
    mut haptic_feedback: EventWriter<HapticFeedbackEvent>,
) {
    // Handle mouse/button interactions
    for (betting_button, interaction) in &interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            info!("Mouse click on button: {:?}", betting_button.action);
            
            match betting_button.action {
                BettingButtonAction::IncreaseRaise => {
                    human_input.raise_amount = (human_input.raise_amount + 5).min(100);
                    info!("Increased raise amount to: {}", human_input.raise_amount);
                }
                BettingButtonAction::DecreaseRaise => {
                    human_input.raise_amount = (human_input.raise_amount.saturating_sub(5)).max(5);
                    info!("Decreased raise amount to: {}", human_input.raise_amount);
                }
                BettingButtonAction::Raise => {
                    human_input.pending_action = Some(PlayerAction::Raise(human_input.raise_amount));
                }
                _ => {
                    human_input.pending_action = Some(PlayerAction::from(betting_button.action.clone()));
                }
            }
            haptic_feedback.send(HapticFeedbackEvent);
        }
    }
    
    // Handle touch input
    for event in touch_events.read() {
        if event.phase == TouchPhase::Started {
            info!("Touch started at position: {:?}", event.position);
            
            let mut found_button = false;
            for (node, transform, betting_button) in &all_button_query {
                let button_rect = node.logical_rect(transform);
                info!("Checking button {:?} at rect: {:?}", betting_button.action, button_rect);
                
                if button_rect.contains(event.position) {
                    info!("Touch hit button: {:?}", betting_button.action);
                    
                    match betting_button.action {
                        BettingButtonAction::IncreaseRaise => {
                            human_input.raise_amount = (human_input.raise_amount + 5).min(100);
                            info!("Touch: Increased raise amount to: {}", human_input.raise_amount);
                        }
                        BettingButtonAction::DecreaseRaise => {
                            human_input.raise_amount = (human_input.raise_amount.saturating_sub(5)).max(5);
                            info!("Touch: Decreased raise amount to: {}", human_input.raise_amount);
                        }
                        BettingButtonAction::Raise => {
                            human_input.pending_action = Some(PlayerAction::Raise(human_input.raise_amount));
                        }
                        _ => {
                            human_input.pending_action = Some(PlayerAction::from(betting_button.action.clone()));
                        }
                    }
                    
                    haptic_feedback.send(HapticFeedbackEvent);
                    found_button = true;
                    break; // Only handle the first button hit
                }
            }
            
            if !found_button {
                info!("Touch did not hit any button");
            }
        }
    }
}
