use bevy::prelude::*;

// Resource to manage game speed and pause state
#[derive(Resource)]
pub struct GameSpeed {
    pub speed_multiplier: f32,
    pub is_paused: bool,
    
    pub auto_advance: bool,
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            is_paused: false,
            auto_advance: true,
        }
    }
}

// Component for timed events that can be affected by game speed
#[derive(Component)]
pub struct GameTimer {
    pub timer: Timer,
    pub base_duration: f32,
}

impl GameTimer {
    pub fn update_speed(&mut self, speed_multiplier: f32) {
        let new_duration = self.base_duration / speed_multiplier.max(0.1);
        self.timer.set_duration(std::time::Duration::from_secs_f32(new_duration));
    }
}

// Plugin for game speed control
pub struct GameSpeedPlugin;

impl Plugin for GameSpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSpeed>()
            .add_systems(Update, (
                update_game_timers,
            ));
    }
}



// System to update all game timers based on current speed
fn update_game_timers(
    mut timer_query: Query<&mut GameTimer>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    if game_speed.is_paused {
        return;
    }
    
    for mut game_timer in timer_query.iter_mut() {
        // Update timer speed if it changed
        if game_speed.is_changed() {
            game_timer.update_speed(game_speed.speed_multiplier);
        }
        
        // Tick the timer
        game_timer.timer.tick(time.delta());
    }
}
