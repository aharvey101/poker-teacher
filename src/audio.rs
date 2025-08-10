use bevy::prelude::*;

// Audio events that can be triggered throughout the game
#[derive(Event)]
#[allow(dead_code)] // Audio system will be implemented in future
pub enum AudioEvent {
    CardDeal,
    ChipBet,
    ButtonClick,
    Fold,
    Call,
    Raise,
    WinHand,
    GameStart,
    NewRound,
}

// Resource to manage audio settings
#[derive(Resource)]
pub struct AudioSettings {
    pub sound_enabled: bool,
    pub volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            volume: 0.5,
        }
    }
}

// Audio system plugin
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AudioEvent>()
            .init_resource::<AudioSettings>()
            .add_systems(Update, (
                handle_audio_events,
            ));
    }
}

// Handle audio events by playing appropriate sounds
fn handle_audio_events(
    mut events: EventReader<AudioEvent>,
    audio_settings: Res<AudioSettings>,
) {
    if !audio_settings.sound_enabled {
        return;
    }

    for event in events.read() {
        // For now, we'll just log the audio events with distinctive emojis
        // In a full implementation, you'd load and play actual audio files
        let (emoji, description) = match event {
            AudioEvent::CardDeal => ("🃏", "Card Deal"),
            AudioEvent::ChipBet => ("💰", "Chip Bet"), 
            AudioEvent::ButtonClick => ("🔘", "Button Click"),
            AudioEvent::Fold => ("❌", "Fold"),
            AudioEvent::Call => ("📞", "Call"),
            AudioEvent::Raise => ("📈", "Raise"),
            AudioEvent::WinHand => ("🎉", "Win Hand"),
            AudioEvent::GameStart => ("🎮", "Game Start"),
            AudioEvent::NewRound => ("🔄", "New Round"),
        };

        info!("🔊 {}: {} (Volume: {:.1})", emoji, description, audio_settings.volume);
    }
}


