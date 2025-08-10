use bevy::prelude::*;

mod ai_player;
mod animations;
mod audio;
mod betting;
mod betting_ui;
mod cards;
mod game_controller;
mod game_speed;
mod game_state;
mod haptics;
mod lifecycle;
mod mobile_cards;
mod mobile_ui;
mod player;
mod poker_rules;
mod rendering;
mod teaching;
mod touch_input;
mod ui;

use ai_player::{AIPersonality, AIPlayerComponent};
use cards::Deck;
use game_controller::GameController;
use game_state::{GameData, GameState};
use player::{AIDifficulty, AIPlayer, HumanPlayer, Player, PlayerType};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Poker Teacher".into(),
                resolution: (390.0, 844.0).into(), // iPhone-like aspect ratio
                resizable: false,                  // Fixed for mobile
                fit_canvas_to_parent: true,        // Better for mobile web
                ..default()
            }),
            ..default()
        }))
        .add_plugins(audio::AudioPlugin)
        .add_plugins(game_speed::GameSpeedPlugin)
        .add_plugins(animations::AnimationPlugin)
        .add_state::<GameState>()
        .init_resource::<Deck>()
        .init_resource::<GameData>()
        .init_resource::<game_state::GamePosition>()
        .init_resource::<GameController>()
        .init_resource::<betting::BettingRound>()
        .init_resource::<betting_ui::HumanPlayerInput>()
        .init_resource::<teaching::TeachingState>()
        .add_event::<haptics::HapticFeedbackEvent>()
        .add_systems(
            Startup,
            (
                setup,
                // Use mobile UI instead of desktop UI
                mobile_ui::setup_mobile_ui,
                teaching::setup_teaching_ui,
            ),
        )
        .add_systems(
            Update,
            (
                // Game logic systems
                game_controller::game_state_controller,
                game_controller::debug_game_state,
                game_controller::toggle_auto_advance,
                // Betting systems
                betting::ai_player_system,
                betting::check_betting_round_complete,
            ),
        )
        .add_systems(
            Update,
            (
                // Mobile input systems
                touch_input::handle_unified_input,
                touch_input::handle_gesture_controls,
                // Mobile UI systems
                mobile_ui::update_mobile_player_info,
                mobile_ui::manage_mobile_teaching_panel,
                // Betting UI systems (adapted for mobile)
                betting_ui::update_raise_amount_display,
                betting_ui::reset_raise_amount_on_new_hand,
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
                // Rendering systems (adapted for mobile)
                mobile_cards::render_mobile_cards,
                mobile_cards::update_mobile_cards,
                mobile_cards::animate_mobile_cards,
                // Keep pot display update
                ui::update_pot_display,
                ui::update_game_phase_display,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn 3 players: 1 human, 2 AI
    // Player positions in a triangle around the table
    let positions = [
        Vec3::new(0.0, -200.0, 0.0),   // Human player (bottom)
        Vec3::new(-300.0, 100.0, 0.0), // AI player 1 (top left)
        Vec3::new(300.0, 100.0, 0.0),  // AI player 2 (top right)
    ];

    // Spawn human player
    commands.spawn((
        Player::new(0, PlayerType::Human, 1000, positions[0]),
        HumanPlayer,
    ));

    // Spawn AI players with advanced AI components
    commands.spawn((
        Player::new(1, PlayerType::AI, 1000, positions[1]),
        AIPlayer {
            difficulty: AIDifficulty::Beginner,
        },
        AIPlayerComponent {
            personality: AIPersonality::beginner(),
        },
    ));

    commands.spawn((
        Player::new(2, PlayerType::AI, 1000, positions[2]),
        AIPlayer {
            difficulty: AIDifficulty::Intermediate,
        },
        AIPlayerComponent {
            personality: AIPersonality::intermediate(),
        },
    ));

    println!("Poker Teacher Game Starting!");
    println!("Players spawned: 1 Human, 2 AI");
    println!("Press SPACE to pause/resume auto-advance");
}
