use bevy::prelude::*;

mod cards;
mod player;
mod game_state;
mod rendering;
mod ui;
mod game_controller;
mod poker_rules;
mod betting;
mod ai_player;
mod betting_ui;
mod teaching;
mod audio;
mod game_speed;
mod animations;
mod touch_input;
mod haptics;
mod lifecycle;

use cards::Deck;
use game_state::{GameState, GameData, AppState};
use player::{Player, PlayerType, HumanPlayer, AIPlayer, AIDifficulty};
use game_controller::GameController;
use ai_player::{AIPlayerComponent, AIPersonality};
use haptics::HapticFeedbackEvent;

// Export C-compatible function for mobile linking
#[no_mangle]
pub extern "C" fn start_app() {
    main();
}

// Main function for mobile
#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Teach Poker".into(),
                resolution: (375.0, 812.0).into(), // iPhone-like resolution
                resizable: false, // Mobile apps typically don't resize
                ..default()
            }),
            ..default()
        }))
        .add_plugins(audio::AudioPlugin)
        .add_plugins(game_speed::GameSpeedPlugin)
        .add_plugins(animations::AnimationPlugin)
        .add_state::<GameState>()
        .add_state::<AppState>()
        .init_resource::<Deck>()
        .init_resource::<GameData>()
        .init_resource::<game_state::GamePosition>()
        .init_resource::<GameController>()
        .init_resource::<betting::BettingRound>()
        .init_resource::<betting_ui::HumanPlayerInput>()
        .init_resource::<teaching::TeachingState>()
        .add_event::<HapticFeedbackEvent>()
        .add_systems(Startup, (setup, ui::setup_ui, betting_ui::setup_betting_ui, teaching::setup_teaching_ui))
        .add_systems(
            Update,
            (
                // Mobile-specific systems
                touch_input::handle_unified_input,
                haptics::handle_haptic_feedback,
                lifecycle::handle_app_lifecycle,
                
                // Game logic systems  
                game_controller::game_state_controller,
                game_controller::debug_game_state,
                game_controller::toggle_auto_advance,
            ),
        )
        .add_systems(
            Update,
            (
                // Betting systems
                betting::ai_player_system,
                betting::check_betting_round_complete,
                
                // Betting UI systems
                betting_ui::manage_betting_ui_visibility,
                betting_ui::handle_raise_adjustment,
                betting_ui::update_betting_button_text,
                betting_ui::reset_raise_amount_on_new_hand,
            ),
        )
        .add_systems(
            Update,
            (
                // Teaching systems
                teaching::provide_contextual_explanations,
                teaching::explain_hand_rankings,
                teaching::highlight_valid_actions,
                teaching::provide_hand_analysis,
                teaching::update_teaching_display,
            ),
        )
        .add_systems(
            Update,
            (
                // Rendering systems
                rendering::render_player_cards,
                rendering::render_community_cards,
                rendering::render_card_backs_for_ai,
                
                // UI systems
                ui::setup_player_ui,
                ui::update_pot_display,
                ui::update_game_phase_display,
                ui::update_player_ui,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());
    
    // Spawn 3 players: 1 human, 2 AI
    // Player positions adjusted for mobile screen
    let positions = [
        Vec3::new(0.0, -300.0, 0.0),    // Human player (bottom) - adjusted for mobile
        Vec3::new(-200.0, 100.0, 0.0),  // AI player 1 (top left) - closer for mobile
        Vec3::new(200.0, 100.0, 0.0),   // AI player 2 (top right) - closer for mobile
    ];
    
    // Spawn human player
    commands.spawn((
        Player::new(0, PlayerType::Human, 1000, positions[0]),
        HumanPlayer,
    ));
    
    // Spawn AI players with AI components
    commands.spawn((
        Player::new(1, PlayerType::AI, 1000, positions[1]),
        AIPlayer { difficulty: AIDifficulty::Beginner },
        AIPlayerComponent {
            personality: AIPersonality::beginner(),
        },
    ));
    
    commands.spawn((
        Player::new(2, PlayerType::AI, 1000, positions[2]),
        AIPlayer { difficulty: AIDifficulty::Intermediate },
        AIPlayerComponent {
            personality: AIPersonality::intermediate(),
        },
    ));
    
    println!("Teach Poker Mobile Starting!");
    println!("Players spawned: 1 Human, 2 AI");
    println!("Touch controls enabled for mobile");
}
