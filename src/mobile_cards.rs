use bevy::prelude::*;
use crate::cards::{Card, Suit, Rank};

// Constants for mobile card display
const MOBILE_CARD_WIDTH: f32 = 45.0;
const MOBILE_CARD_HEIGHT: f32 = 60.0;

// Mobile-friendly card colors with better contrast
const MOBILE_CARD_BG: Color = Color::rgb(0.98, 0.98, 0.96);
const MOBILE_CARD_BORDER: Color = Color::rgb(0.7, 0.7, 0.7);
const MOBILE_HEART_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
const MOBILE_DIAMOND_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
const MOBILE_CLUB_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
const MOBILE_SPADE_COLOR: Color = Color::rgb(0.05, 0.05, 0.05);
const MOBILE_CARD_BACK: Color = Color::rgb(0.15, 0.25, 0.55); // Richer blue for better contrast

// Component for mobile card display
#[derive(Component)]
pub struct MobileCard {
    #[allow(dead_code)]
    pub card: Card,
    #[allow(dead_code)]
    pub is_community: bool,
    pub is_face_down: bool,
}

#[derive(Component)]
pub struct MobileCardContainer;

pub fn mobile_suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts => MOBILE_HEART_COLOR,
        Suit::Diamonds => MOBILE_DIAMOND_COLOR,
        Suit::Clubs => MOBILE_CLUB_COLOR,
        Suit::Spades => MOBILE_SPADE_COLOR,
    }
}

pub fn mobile_suit_symbol(suit: Suit) -> &'static str {
    match suit {
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
        Suit::Spades => "♠",
    }
}

pub fn mobile_rank_symbol(rank: Rank) -> &'static str {
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

// System to render cards in mobile-optimized layout
pub fn render_mobile_cards(
    mut commands: Commands,
    // Query for existing mobile cards
    existing_cards: Query<Entity, With<MobileCard>>,
    // Community cards container
    community_container: Query<Entity, With<MobileCardContainer>>,
) {
    // Clear existing cards
    for entity in existing_cards.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
    
    // Find or create community cards container
    let container = if let Ok(container_entity) = community_container.get_single() {
        container_entity
    } else {
        // Create community cards container if it doesn't exist
        commands.spawn(MobileCardContainer).id()
    };
    
    // This would be called from game logic to update cards
    // For now, we'll create placeholder cards
    create_mobile_community_cards(&mut commands, container);
}

fn create_mobile_community_cards(commands: &mut Commands, container: Entity) {
    // Example community cards (this would come from game state)
    let example_cards = vec![
        Card { suit: Suit::Spades, rank: Rank::Ace },
        Card { suit: Suit::Hearts, rank: Rank::Three },
        Card { suit: Suit::Spades, rank: Rank::Eight },
        Card { suit: Suit::Spades, rank: Rank::Seven },
        Card { suit: Suit::Hearts, rank: Rank::Two },
    ];
    
    commands.entity(container).with_children(|parent| {
        for card in example_cards {
            create_mobile_card_ui(parent, card, false);
        }
    });
}

pub fn create_mobile_card_ui(parent: &mut ChildBuilder, card: Card, is_face_down: bool) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(MOBILE_CARD_WIDTH),
                height: Val::Px(MOBILE_CARD_HEIGHT),
                margin: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: if is_face_down { MOBILE_CARD_BACK } else { MOBILE_CARD_BG }.into(),
            border_color: MOBILE_CARD_BORDER.into(),
            ..default()
        })
        .with_children(|card_parent| {
            if !is_face_down {
                // Top rank and suit
                card_parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::FlexStart,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|top_parent| {
                        // Rank (top-left)
                        top_parent.spawn(TextBundle::from_section(
                            mobile_rank_symbol(card.rank),
                            TextStyle {
                                font_size: 12.0,
                                color: mobile_suit_color(card.suit),
                                ..default()
                            },
                        ));
                    });
                
                // Center suit symbol (larger)
                card_parent.spawn(TextBundle::from_section(
                    mobile_suit_symbol(card.suit),
                    TextStyle {
                        font_size: 20.0,
                        color: mobile_suit_color(card.suit),
                        ..default()
                    },
                ));
                
                // Bottom rank and suit (rotated appearance)
                card_parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::FlexEnd,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|bottom_parent| {
                        // Rank (bottom-right, smaller)
                        bottom_parent.spawn(TextBundle::from_section(
                            mobile_rank_symbol(card.rank),
                            TextStyle {
                                font_size: 8.0,
                                color: mobile_suit_color(card.suit),
                                ..default()
                            },
                        ));
                    });
            } else {
                // Enhanced face-down card design with pattern
                card_parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: MOBILE_CARD_BACK.into(),
                        ..default()
                    })
                    .with_children(|back_parent| {
                        // Create a pattern with multiple symbols for a classic card back look
                        for row in 0..3 {
                            back_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(30.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|row_parent| {
                                    for col in 0..2 {
                                        let symbol = if (row + col) % 2 == 0 { "♠" } else { "♦" };
                                        row_parent.spawn(TextBundle::from_section(
                                            symbol,
                                            TextStyle {
                                                font_size: if row == 1 { 14.0 } else { 10.0 },
                                                color: Color::rgba(1.0, 1.0, 1.0, 0.8),
                                                ..default()
                                            },
                                        ));
                                    }
                                });
                        }
                        
                        // Add a border pattern
                        back_parent.spawn(TextBundle::from_section(
                            "♦ ♠ ♥ ♣",
                            TextStyle {
                                font_size: 8.0,
                                color: Color::rgba(1.0, 1.0, 1.0, 0.6),
                                ..default()
                            },
                        ));
                    });
            }
        })
        .insert(MobileCard {
            card,
            is_community: true,
            is_face_down,
        });
}

// System to update mobile cards based on game state
pub fn update_mobile_cards(
    // Add game state queries here
    // players: Query<&Player>,
    // game_data: Res<GameData>,
    mut card_query: Query<&mut MobileCard>,
) {
    // Update card visibility, face-up/face-down state based on game progression
    for mut mobile_card in card_query.iter_mut() {
        // Implementation would depend on game state
        // For now, just ensure cards are visible
        mobile_card.is_face_down = false;
    }
}

// Enhanced card animations for mobile
pub fn animate_mobile_cards(
    time: Res<Time>,
    mut card_query: Query<&mut Transform, With<MobileCard>>,
) {
    // Add subtle animations like card flip, dealing animation, etc.
    for mut transform in card_query.iter_mut() {
        // Example: subtle hover effect
        let hover_offset = (time.elapsed_seconds() * 2.0).sin() * 1.0;
        transform.translation.y += hover_offset * 0.1;
    }
}
