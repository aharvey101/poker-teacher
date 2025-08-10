use bevy::prelude::*;
use bevy::input::touch::TouchPhase;

use crate::betting_ui::{BettingButtonAction, HumanPlayerInput};
use crate::betting::PlayerAction;
use crate::haptics::HapticFeedbackEvent;

impl From<BettingButtonAction> for PlayerAction {
    fn from(action: BettingButtonAction) -> Self {
        match action {
            BettingButtonAction::Fold => PlayerAction::Fold,
            BettingButtonAction::Check => PlayerAction::Check,
            BettingButtonAction::Call => PlayerAction::Call,
            BettingButtonAction::Raise => PlayerAction::Raise(20), // Default raise amount
        }
    }
}

pub fn handle_touch_input(
    mut touch_events: EventReader<TouchInput>,
    mut human_input: ResMut<HumanPlayerInput>,
    button_query: Query<(&Node, &GlobalTransform, &Interaction, &BettingButtonAction)>,
    mut haptic_feedback: EventWriter<HapticFeedbackEvent>,
) {
    for event in touch_events.read() {
        if event.phase == TouchPhase::Started {
            for (node, transform, _interaction, action) in &button_query {
                let button_rect = node.logical_rect(transform);
                if button_rect.contains(event.position) {
                    human_input.pending_action = Some(PlayerAction::from(action.clone()));
                    haptic_feedback.send(HapticFeedbackEvent);
                }
            }
        }
    }
}
