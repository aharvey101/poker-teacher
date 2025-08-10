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

// Enhanced input system optimized for mobile touch
pub fn handle_unified_input(
    mut touch_events: EventReader<TouchInput>,
    mut human_input: ResMut<HumanPlayerInput>,
    // Query for mouse/interaction events (for desktop testing)
    interaction_query: Query<(&BettingButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    // Query for touch events (all buttons) with expanded touch targets
    all_button_query: Query<(&Node, &GlobalTransform, &BettingButton), With<Button>>,
    mut haptic_feedback: EventWriter<HapticFeedbackEvent>,
    windows: Query<&Window>,
) {
    // Handle mouse/button interactions (for desktop testing)
    for (betting_button, interaction) in &interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            info!("Mouse click on button: {:?}", betting_button.action);
            handle_betting_action(&mut human_input, &betting_button.action, &mut haptic_feedback);
        }
    }
    
    // Handle touch input with improved mobile experience
    for event in touch_events.read() {
        match event.phase {
            TouchPhase::Started => {
                handle_touch_started(event, &all_button_query, &mut human_input, &mut haptic_feedback, &windows);
            }
            TouchPhase::Moved => {
                // Could implement gesture detection here for swipe controls
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                // Could implement tap confirmation or gesture completion here
            }
        }
    }
}

fn handle_touch_started(
    event: &TouchInput,
    all_button_query: &Query<(&Node, &GlobalTransform, &BettingButton), With<Button>>,
    human_input: &mut ResMut<HumanPlayerInput>,
    haptic_feedback: &mut EventWriter<HapticFeedbackEvent>,
    windows: &Query<&Window>,
) {
    info!("Touch started at position: {:?}", event.position);
    
    // Get window dimensions for proper scaling
    let window = windows.single();
    let window_size = Vec2::new(window.width(), window.height());
    
    let mut found_button = false;
    for (node, transform, betting_button) in all_button_query.iter() {
        // Create expanded touch target (44pt minimum as per iOS guidelines)
        let button_rect = node.logical_rect(transform);
        let min_touch_size = 44.0;
        
        // Expand touch target if button is smaller than minimum
        let expanded_rect = Rect {
            min: Vec2::new(
                button_rect.min.x - (min_touch_size - button_rect.width()).max(0.0) / 2.0,
                button_rect.min.y - (min_touch_size - button_rect.height()).max(0.0) / 2.0,
            ),
            max: Vec2::new(
                button_rect.max.x + (min_touch_size - button_rect.width()).max(0.0) / 2.0,
                button_rect.max.y + (min_touch_size - button_rect.height()).max(0.0) / 2.0,
            ),
        };
        
        // Convert touch position to UI coordinates
        let ui_position = Vec2::new(
            event.position.x,
            window_size.y - event.position.y, // Flip Y coordinate for UI space
        );
        
        info!("Checking button {:?} at rect: {:?}, expanded: {:?}, touch at: {:?}", 
              betting_button.action, button_rect, expanded_rect, ui_position);
        
        if expanded_rect.contains(ui_position) {
            info!("Touch hit button: {:?}", betting_button.action);
            handle_betting_action(human_input, &betting_button.action, haptic_feedback);
            found_button = true;
            break; // Only handle the first button hit
        }
    }
    
    if !found_button {
        info!("Touch did not hit any button");
    }
}

fn handle_betting_action(
    human_input: &mut ResMut<HumanPlayerInput>,
    action: &BettingButtonAction,
    haptic_feedback: &mut EventWriter<HapticFeedbackEvent>,
) {
    match action {
        BettingButtonAction::IncreaseRaise => {
            // Increase by $5 with mobile-friendly increments
            human_input.raise_amount = (human_input.raise_amount + 5).min(200); // Increased max for mobile
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
            human_input.pending_action = Some(PlayerAction::from(action.clone()));
        }
    }
    
    // Provide tactile feedback for better mobile UX
    haptic_feedback.send(HapticFeedbackEvent);
}

// System to handle gesture-based controls (optional enhancement)
pub fn handle_gesture_controls(
    mut touch_events: EventReader<TouchInput>,
    mut human_input: ResMut<HumanPlayerInput>,
) {
    static mut GESTURE_START: Option<Vec2> = None;
    static mut GESTURE_THRESHOLD: f32 = 50.0; // Minimum swipe distance
    
    for event in touch_events.read() {
        match event.phase {
            TouchPhase::Started => {
                unsafe { GESTURE_START = Some(event.position); }
            }
            TouchPhase::Ended => {
                if let Some(start_pos) = unsafe { GESTURE_START } {
                    let swipe_distance = event.position - start_pos;
                    
                    // Horizontal swipes for raise amount adjustment
                    if swipe_distance.x.abs() > unsafe { GESTURE_THRESHOLD } && swipe_distance.y.abs() < 30.0 {
                        if swipe_distance.x > 0.0 {
                            // Swipe right: increase raise
                            human_input.raise_amount = (human_input.raise_amount + 10).min(200);
                            info!("Gesture: Increased raise to {}", human_input.raise_amount);
                        } else {
                            // Swipe left: decrease raise
                            human_input.raise_amount = (human_input.raise_amount.saturating_sub(10)).max(5);
                            info!("Gesture: Decreased raise to {}", human_input.raise_amount);
                        }
                    }
                    
                    // Vertical swipes for quick actions
                    if swipe_distance.y.abs() > unsafe { GESTURE_THRESHOLD } && swipe_distance.x.abs() < 30.0 {
                        if swipe_distance.y < 0.0 {
                            // Swipe up: quick fold
                            human_input.pending_action = Some(PlayerAction::Fold);
                            info!("Gesture: Quick fold");
                        }
                    }
                }
                unsafe { GESTURE_START = None; }
            }
            _ => {}
        }
    }
}
