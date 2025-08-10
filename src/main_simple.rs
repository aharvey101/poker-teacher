use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Poker Teacher - Simple".into(),
                resolution: (1024.0, 768.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2d);
    
    // Spawn a simple text
    commands.spawn((
        Text::new("Poker Teacher - Building..."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
    
    println!("Simple Poker Teacher Game Starting!");
}
