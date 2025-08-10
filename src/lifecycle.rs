use bevy::prelude::*;
use crate::game_state::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn handle_app_lifecycle(
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventReader<AppExit>,
) {
    for _ in app_exit_events.read() {
        bevy::log::info!("App exit requested, entering suspended state.");
        app_state.set(AppState::Suspended);
    }
}
