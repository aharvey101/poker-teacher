use bevy::prelude::*;

#[derive(Event)]
pub struct HapticFeedbackEvent;

pub fn handle_haptic_feedback(
    mut events: EventReader<HapticFeedbackEvent>,
) {
    for _ in events.read() {
        // This is where you would interface with a native library
        // to trigger haptic feedback. For now, we'll just log it.
        info!(" BZZT! Haptic feedback triggered.");
    }
}
