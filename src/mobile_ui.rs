use bevy::prelude::*;
use crate::betting_ui::{BettingButtonAction, BettingButton};
use crate::player::Player;

// Mobile-optimized UI components
#[derive(Component)]
pub struct MobilePlayerUI {
    
    pub player_id: u32,
}

#[derive(Component)]
pub struct MobileGameInfo;

#[derive(Component)]
pub struct MobileBettingPanel;

#[derive(Component)]
pub struct MobileTeachingPanel;

// Mobile-friendly constants
const MOBILE_BUTTON_HEIGHT: f32 = 60.0;
const MOBILE_TOUCH_PADDING: f32 = 8.0;
const MOBILE_TEXT_SIZE_MEDIUM: f32 = 18.0;
const MOBILE_TEXT_SIZE_SMALL: f32 = 14.0;

// Color scheme optimized for mobile readability
const MOBILE_PRIMARY_BG: Color = Color::rgba(0.08, 0.12, 0.16, 0.95);
const MOBILE_SECONDARY_BG: Color = Color::rgba(0.12, 0.16, 0.20, 0.90);
// Enhanced button colors with better contrast
const MOBILE_ACCENT_GREEN: Color = Color::rgb(0.15, 0.7, 0.3);
const MOBILE_ACCENT_RED: Color = Color::rgb(0.85, 0.25, 0.15);
const MOBILE_ACCENT_BLUE: Color = Color::rgb(0.2, 0.5, 0.85);
// Enhanced text colors
const MOBILE_TEXT_PRIMARY: Color = Color::rgb(0.98, 0.98, 0.98);

pub fn setup_mobile_ui(mut commands: Commands) {
    println!("🔧 Setting up mobile UI...");
    // Full-screen container with mobile-optimized layout
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top section: Game info and opponent players (20% of screen)
            create_mobile_top_section(parent);
            
            // Middle section: Community cards and pot (50% of screen)
            create_mobile_middle_section(parent);
            
            // Bottom section: Player hand and controls (30% of screen)
            create_mobile_bottom_section(parent);
        });
}

fn create_mobile_top_section(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(MOBILE_TOUCH_PADDING)),
                ..default()
            },
            background_color: MOBILE_SECONDARY_BG.into(),
            ..default()
        })
        .with_children(|top_parent| {
            // AI Player 1 (left)
            create_mobile_ai_player_card(top_parent, 1, FlexDirection::Row);
            
            // Center: Game phase and pot
            create_mobile_game_info(top_parent);
            
            // AI Player 2 (right)
            create_mobile_ai_player_card(top_parent, 2, FlexDirection::RowReverse);
        });
}

fn create_mobile_middle_section(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(MOBILE_TOUCH_PADDING)),
                ..default()
            },
            background_color: MOBILE_PRIMARY_BG.into(),
            ..default()
        })
        .with_children(|middle_parent| {
            // Community cards area
            middle_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(90.0),
                        height: Val::Percent(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(16.0)),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.2).into(),
                    ..default()
                })
                .with_children(|community_parent| {
                    println!("🃏 Creating community cards...");
                    // Create 5 community cards (flop, turn, river)
                    let community_cards = [
                        (crate::cards::Suit::Hearts, crate::cards::Rank::Ace),
                        (crate::cards::Suit::Spades, crate::cards::Rank::King),
                        (crate::cards::Suit::Diamonds, crate::cards::Rank::Queen),
                        (crate::cards::Suit::Clubs, crate::cards::Rank::Jack),
                        (crate::cards::Suit::Hearts, crate::cards::Rank::Ten),
                    ];
                    
                    for (suit, rank) in community_cards.iter() {
                        community_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(45.0),  // Medium size for community cards
                                    height: Val::Px(63.0), // Proportional height
                                    margin: UiRect::all(Val::Px(3.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.98, 0.98, 0.96).into(), // Card face color
                                border_color: Color::rgb(0.7, 0.7, 0.7).into(),
                                ..default()
                            })
                            .with_children(|card_parent| {
                                // Top rank
                                card_parent.spawn(TextBundle::from_section(
                                    crate::mobile_cards::mobile_rank_symbol(*rank),
                                    TextStyle {
                                        font_size: 10.0,
                                        color: crate::mobile_cards::mobile_suit_color(*suit),
                                        ..default()
                                    },
                                ));
                                
                                // Center suit symbol
                                card_parent.spawn(TextBundle::from_section(
                                    crate::mobile_cards::mobile_suit_symbol(*suit),
                                    TextStyle {
                                        font_size: 16.0,
                                        color: crate::mobile_cards::mobile_suit_color(*suit),
                                        ..default()
                                    },
                                ));
                                
                                // Bottom rank (rotated)
                                card_parent.spawn(TextBundle::from_section(
                                    crate::mobile_cards::mobile_rank_symbol(*rank),
                                    TextStyle {
                                        font_size: 10.0,
                                        color: crate::mobile_cards::mobile_suit_color(*suit),
                                        ..default()
                                    },
                                ));
                            })
                            .insert(crate::mobile_cards::MobileCard {
                                card: crate::cards::Card {
                                    suit: *suit,
                                    rank: *rank,
                                },
                                is_community: true,
                                is_face_down: false,
                            });
                    }
                });
            
            // Teaching/hints area (collapsible)
            create_mobile_teaching_panel(middle_parent);
        });
}

fn create_mobile_bottom_section(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: MOBILE_SECONDARY_BG.into(),
            ..default()
        })
        .with_children(|bottom_parent| {
            // Player cards area (15% of bottom section)
            create_mobile_player_cards_area(bottom_parent);
            
            // Betting controls (85% of bottom section)
            create_mobile_betting_controls(bottom_parent);
        })
        .insert(MobileBettingPanel);
}

fn create_mobile_ai_player_card(parent: &mut ChildBuilder, player_id: u32, direction: FlexDirection) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(35.0),
                height: Val::Percent(80.0),
                flex_direction: direction,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: MOBILE_ACCENT_BLUE.with_a(0.2).into(),
            border_color: MOBILE_ACCENT_BLUE.with_a(0.5).into(),
            ..default()
        })
        .with_children(|player_parent| {
            // Cards container for 2 face-down cards
            player_parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|cards_parent| {
                    println!("🃏 Creating AI player {} cards...", player_id);
                    // Create 2 cards for each AI player
                    for _i in 0..2 {
                        cards_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(30.0),  // Smaller for AI players
                                    height: Val::Px(42.0), // Proportional height
                                    margin: UiRect::all(Val::Px(2.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.0, 1.0).into(), // Bright magenta to see if it shows up
                                border_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                ..default()
                            })
                            .with_children(|card_parent| {
                                // Add smaller card back pattern for AI cards
                                for row in 0..2 {
                                    card_parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::SpaceEvenly,
                                                width: Val::Percent(100.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|row_parent| {
                                            let symbol = if row == 0 { "♠" } else { "♦" };
                                            row_parent.spawn(TextBundle::from_section(
                                                symbol,
                                                TextStyle {
                                                    font_size: 8.0,  // Smaller for AI cards
                                                    color: Color::rgb(0.7, 0.7, 0.9),
                                                    ..default()
                                                },
                                            ));
                                        });
                                }
                            })
                            .insert(crate::mobile_cards::MobileCard {
                                card: crate::cards::Card {
                                    suit: crate::cards::Suit::Spades,
                                    rank: crate::cards::Rank::Ace,
                                },
                                is_community: false,
                                is_face_down: true,
                            });
                    }
                });
            
            // Player info
            player_parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|info_parent| {
                    info_parent.spawn(TextBundle::from_section(
                        format!("AI {}", player_id),
                        TextStyle {
                            font_size: MOBILE_TEXT_SIZE_MEDIUM,
                            color: MOBILE_TEXT_PRIMARY,
                            ..default()
                        },
                    ));
                    
                    info_parent.spawn(TextBundle::from_section(
                        "$1000",
                        TextStyle {
                            font_size: MOBILE_TEXT_SIZE_MEDIUM, // Larger for better readability
                            color: Color::rgb(0.9, 0.9, 0.3), // Gold color for chip amounts
                            ..default()
                        },
                    ));
                });
        })
        .insert(MobilePlayerUI { player_id });
}

fn create_mobile_game_info(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|info_parent| {
            // POT label with better styling
            info_parent.spawn(TextBundle::from_section(
                "POT",
                TextStyle {
                    font_size: MOBILE_TEXT_SIZE_MEDIUM, // Larger for better visibility
                    color: MOBILE_TEXT_PRIMARY, // Higher contrast
                    ..default()
                },
            ));
            
            // Pot amount with enhanced styling
            info_parent.spawn(TextBundle::from_section(
                "$20",
                TextStyle {
                    font_size: 28.0, // Even larger for prominence
                    color: Color::rgb(0.2, 0.9, 0.3), // Brighter green for better visibility
                    ..default()
                },
            ));
            
            // Game phase with better visibility
            info_parent.spawn(TextBundle::from_section(
                "River",
                TextStyle {
                    font_size: MOBILE_TEXT_SIZE_MEDIUM, // Larger than before
                    color: MOBILE_TEXT_PRIMARY, // Higher contrast
                    ..default()
                },
            ));
        })
        .insert(MobileGameInfo);
}

fn create_mobile_teaching_panel(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                height: Val::Px(50.0), // Fixed smaller height to prevent overlap
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: MOBILE_ACCENT_BLUE.with_a(0.08).into(), // More subtle
            border_color: MOBILE_ACCENT_BLUE.with_a(0.2).into(),
            ..default()
        })
        .with_children(|teaching_parent| {
            teaching_parent.spawn(TextBundle::from_section(
                "💡 Your Turn! Last chance to bet before final reveal.",
                TextStyle {
                    font_size: MOBILE_TEXT_SIZE_SMALL,
                    color: MOBILE_TEXT_PRIMARY,
                    ..default()
                },
            ));
        })
        .insert(MobileTeachingPanel);
}

fn create_mobile_player_cards_area(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(25.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|cards_parent| {
            println!("🃏 Creating player cards...");
            // Player's hole cards - use card back design instead of blank rectangles
            for _i in 0..2 {
                cards_parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(60.0),  // Slightly larger for better visibility
                            height: Val::Px(84.0), // Better aspect ratio
                            margin: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.25, 0.55).into(), // Card back blue
                        border_color: Color::rgb(0.8, 0.8, 0.8).into(),
                        ..default()
                    })
                    .with_children(|card_parent| {
                        // Add card back pattern using text symbols
                        for row in 0..3 {
                            card_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|row_parent| {
                                    for col in 0..2 {
                                        let symbol = match (row + col) % 4 {
                                            0 => "♠",
                                            1 => "♥", 
                                            2 => "♦",
                                            _ => "♣",
                                        };
                                        row_parent.spawn(TextBundle::from_section(
                                            symbol,
                                            TextStyle {
                                                font_size: 12.0,
                                                color: Color::rgb(0.7, 0.7, 0.9),
                                                ..default()
                                            },
                                        ));
                                    }
                                });
                        }
                    })
                    .insert(crate::mobile_cards::MobileCard {
                        card: crate::cards::Card {
                            suit: crate::cards::Suit::Spades,
                            rank: crate::cards::Rank::Ace,
                        },
                        is_community: false,
                        is_face_down: true,
                    });
            }
        });
}

fn create_mobile_betting_controls(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(75.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(MOBILE_TOUCH_PADDING)),
                ..default()
            },
            ..default()
        })
        .with_children(|betting_parent| {
            // Main betting buttons row
            betting_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(MOBILE_BUTTON_HEIGHT),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    create_mobile_betting_button(buttons_parent, "FOLD", BettingButtonAction::Fold, MOBILE_ACCENT_RED);
                    create_mobile_betting_button(buttons_parent, "CALL", BettingButtonAction::Call, MOBILE_ACCENT_GREEN);
                    create_mobile_betting_button(buttons_parent, "RAISE", BettingButtonAction::Raise, MOBILE_ACCENT_BLUE);
                });
            
            // Raise amount controls
            create_mobile_raise_controls(betting_parent);
        });
}

fn create_mobile_betting_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: BettingButtonAction,
    color: Color,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Px(MOBILE_BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)), // Slightly thicker border
                margin: UiRect::all(Val::Px(4.0)), // Add some spacing between buttons
                ..default()
            },
            background_color: color.into(),
            border_color: Color::rgb(
                (color.r() + 0.2).min(1.0), 
                (color.g() + 0.2).min(1.0), 
                (color.b() + 0.2).min(1.0)
            ).into(), // Lighter border for better definition
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: MOBILE_TEXT_SIZE_MEDIUM,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(BettingButton { action });
}

fn create_mobile_raise_controls(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|raise_parent| {
            // Decrease button
            create_mobile_raise_adjust_button(raise_parent, "-$5", BettingButtonAction::DecreaseRaise);
            
            // Amount display with enhanced styling
            raise_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(140.0), // Wider for better visibility
                        height: Val::Px(50.0), // Taller for better touch
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::horizontal(Val::Px(16.0)),
                        border: UiRect::all(Val::Px(2.0)), // Thicker border
                        ..default()
                    },
                    background_color: Color::rgba(0.2, 0.3, 0.4, 0.8).into(), // Darker background for contrast
                    border_color: MOBILE_ACCENT_BLUE.into(), // Blue border for consistency
                    ..default()
                })
                .with_children(|amount_parent| {
                    amount_parent.spawn(TextBundle::from_section(
                        "$20",
                        TextStyle {
                            font_size: 20.0, // Larger text for better visibility
                            color: Color::rgb(0.9, 0.9, 0.3), // Gold color like chip amounts
                            ..default()
                        },
                    ));
                });
            
            // Increase button
            create_mobile_raise_adjust_button(raise_parent, "+$5", BettingButtonAction::IncreaseRaise);
        });
}

fn create_mobile_raise_adjust_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: BettingButtonAction,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(80.0), // Slightly wider for better touch target
                height: Val::Px(50.0), // Taller for better touch target
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            background_color: MOBILE_SECONDARY_BG.into(),
            border_color: MOBILE_ACCENT_BLUE.into(), // More distinctive border
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: MOBILE_TEXT_SIZE_MEDIUM, // Larger text for better readability
                    color: MOBILE_TEXT_PRIMARY,
                    ..default()
                },
            ));
        })
        .insert(BettingButton { action });
}

// System to update mobile UI based on game state
pub fn update_mobile_player_info(
    _player_ui_query: Query<&mut Text, With<MobilePlayerUI>>,
    players: Query<&Player>,
) {
    // Update player information displays
    for _player in players.iter() {
        // Update chip counts, betting status, etc.
        // Implementation details...
    }
}

// System to show/hide mobile teaching panel
pub fn manage_mobile_teaching_panel(
    _teaching_panel_query: Query<&mut Visibility, With<MobileTeachingPanel>>,
    // Add other necessary queries and resources
) {
    // Toggle visibility based on game state and user preferences
    // Implementation details...
}
