use bevy::prelude::*;
use crate::cards::{Card, Suit, Rank};
use crate::player::Player;

// Constants for card rendering
const CARD_WIDTH: f32 = 60.0;
const CARD_HEIGHT: f32 = 84.0;
const CARD_CORNER_RADIUS: f32 = 8.0;

// Component to mark rendered cards
#[derive(Component)]
pub struct RenderedCard {
    pub card: Card,
    pub owner_id: Option<u32>, // None for community cards
}

// Component for card back (face-down cards)
#[derive(Component)]
pub struct CardBack;

// Colors for suits
const HEART_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);
const DIAMOND_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);
const CLUB_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const SPADE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CARD_BACKGROUND: Color = Color::srgb(0.95, 0.95, 0.9);
const CARD_BACK_COLOR: Color = Color::srgb(0.2, 0.3, 0.6);

pub fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts => HEART_COLOR,
        Suit::Diamonds => DIAMOND_COLOR,
        Suit::Clubs => CLUB_COLOR,
        Suit::Spades => SPADE_COLOR,
    }
}

pub fn suit_symbol(suit: Suit) -> &'static str {
    match suit {
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
        Suit::Spades => "♠",
    }
}

pub fn rank_symbol(rank: Rank) -> &'static str {
    match rank {
        Rank::Two => "2",
        Rank::Three => "3", 
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "J",
        Rank::Queen => "Q",
        Rank::King => "K",
        Rank::Ace => "A",
    }
}

// System to render cards for players
pub fn render_player_cards(
    mut commands: Commands,
    players: Query<&Player>,
    rendered_cards: Query<Entity, With<RenderedCard>>,
) {
    // Clear existing rendered cards
    for entity in rendered_cards.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Render cards for each player
    for player in players.iter() {
        let card_spacing = CARD_WIDTH + 10.0;
        let start_x = player.position.x - (card_spacing * (player.hole_cards.len() as f32 - 1.0)) / 2.0;
        
        for (i, &card) in player.hole_cards.iter().enumerate() {
            let card_pos = Vec3::new(
                start_x + i as f32 * card_spacing,
                player.position.y - 50.0, // Cards below player position
                1.0, // Above background
            );
            
            spawn_card(&mut commands, card, card_pos, Some(player.id));
        }
    }
}

// System to render community cards
pub fn render_community_cards(
    mut commands: Commands,
    game_data: Res<crate::game_state::GameData>,
    rendered_community_cards: Query<Entity, (With<RenderedCard>, Without<CardBack>)>,
) {
    // Clear existing community cards
    for entity in rendered_community_cards.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Render community cards in the center
    let card_spacing = CARD_WIDTH + 15.0;
    let start_x = -(card_spacing * (game_data.community_cards.len() as f32 - 1.0)) / 2.0;
    
    for (i, &card) in game_data.community_cards.iter().enumerate() {
        let card_pos = Vec3::new(
            start_x + i as f32 * card_spacing,
            0.0, // Center of table
            1.0,
        );
        
        spawn_card(&mut commands, card, card_pos, None);
    }
}

fn spawn_card(commands: &mut Commands, card: Card, position: Vec3, owner_id: Option<u32>) {
    // Card background (rectangle)
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: CARD_BACKGROUND,
                custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            // Rank text (top-left)
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    rank_symbol(card.rank),
                    TextStyle {
                        font_size: 16.0,
                        color: suit_color(card.suit),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(-CARD_WIDTH/2.0 + 8.0, CARD_HEIGHT/2.0 - 12.0, 0.1),
                ..default()
            });
            
            // Suit symbol (top-left, below rank)
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    suit_symbol(card.suit),
                    TextStyle {
                        font_size: 14.0,
                        color: suit_color(card.suit),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(-CARD_WIDTH/2.0 + 8.0, CARD_HEIGHT/2.0 - 28.0, 0.1),
                ..default()
            });
            
            // Large suit symbol in center
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    suit_symbol(card.suit),
                    TextStyle {
                        font_size: 24.0,
                        color: suit_color(card.suit),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
            
            // Rank text (bottom-right, rotated)
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    rank_symbol(card.rank),
                    TextStyle {
                        font_size: 16.0,
                        color: suit_color(card.suit),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(CARD_WIDTH/2.0 - 8.0, -CARD_HEIGHT/2.0 + 12.0, 0.1)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                ..default()
            });
            
            // Suit symbol (bottom-right, rotated)
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    suit_symbol(card.suit),
                    TextStyle {
                        font_size: 14.0,
                        color: suit_color(card.suit),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(CARD_WIDTH/2.0 - 8.0, -CARD_HEIGHT/2.0 + 28.0, 0.1)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                ..default()
            });
        })
        .insert(RenderedCard { card, owner_id });
}

// System to render card backs for AI players (face-down cards)
pub fn render_card_backs_for_ai(
    mut commands: Commands,
    players: Query<&Player>,
    card_backs: Query<Entity, With<CardBack>>,
) {
    // Clear existing card backs
    for entity in card_backs.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Render card backs for AI players only
    for player in players.iter() {
        if matches!(player.player_type, crate::player::PlayerType::AI) && !player.hole_cards.is_empty() {
            let card_spacing = CARD_WIDTH + 10.0;
            let start_x = player.position.x - (card_spacing * (player.hole_cards.len() as f32 - 1.0)) / 2.0;
            
            for i in 0..player.hole_cards.len() {
                let card_pos = Vec3::new(
                    start_x + i as f32 * card_spacing,
                    player.position.y - 50.0,
                    1.0,
                );
                
                spawn_card_back(&mut commands, card_pos);
            }
        }
    }
}

fn spawn_card_back(commands: &mut Commands, position: Vec3) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: CARD_BACK_COLOR,
                custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            // Card back pattern (simple cross pattern)
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "🂠",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(0.8, 0.8, 0.9),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        })
        .insert(CardBack);
}
