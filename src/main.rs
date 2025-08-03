use bevy::prelude::*;

mod cards;
mod player;
mod game_state;
mod rendering;
mod ui;
mod game_controller;
mod poker_rules;
mod betting;

use cards::Deck;
use game_state::{GameState, GameData};
use player::{Player, PlayerType, HumanPlayer, AIPlayer, AIDifficulty};
use game_controller::GameController;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Poker Teacher".into(),
                resolution: (1024.0, 768.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<Deck>()
        .init_resource::<GameData>()
        .init_resource::<GameController>()
        .init_resource::<betting::BettingRound>()
        .add_systems(Startup, (setup, ui::setup_ui))
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
    // Player positions in a triangle around the table
    let positions = [
        Vec3::new(0.0, -200.0, 0.0),    // Human player (bottom)
        Vec3::new(-300.0, 100.0, 0.0),  // AI player 1 (top left)
        Vec3::new(300.0, 100.0, 0.0),   // AI player 2 (top right)
    ];
    
    // Spawn human player
    commands.spawn((
        Player::new(0, PlayerType::Human, 1000, positions[0]),
        HumanPlayer,
    ));
    
    // Spawn AI players
    commands.spawn((
        Player::new(1, PlayerType::AI, 1000, positions[1]),
        AIPlayer { difficulty: AIDifficulty::Beginner },
    ));
    
    commands.spawn((
        Player::new(2, PlayerType::AI, 1000, positions[2]),
        AIPlayer { difficulty: AIDifficulty::Beginner },
    ));
    
    println!("Poker Teacher Game Starting!");
    println!("Players spawned: 1 Human, 2 AI");
    println!("Press SPACE to pause/resume auto-advance");
}
