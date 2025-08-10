use bevy::prelude::*;
use crate::player::{Player, AIDifficulty};
use crate::betting::{PlayerAction, BettingRound};
use crate::poker_rules::evaluate_hand;
use crate::cards::Card;
use rand::Rng;

/// AI personality traits that affect decision making
#[derive(Debug, Clone)]
pub struct AIPersonality {
    pub difficulty: AIDifficulty,
    pub aggression: f32,      // 0.0 = passive, 1.0 = very aggressive
    pub tightness: f32,       // 0.0 = loose, 1.0 = very tight
    pub bluff_frequency: f32, // 0.0 = never bluff, 1.0 = bluff often
    
    pub position_awareness: f32, // 0.0 = ignore position, 1.0 = highly position-aware
}

impl Default for AIPersonality {
    fn default() -> Self {
        Self {
            difficulty: AIDifficulty::Beginner,
            aggression: 0.3,
            tightness: 0.5,
            bluff_frequency: 0.1,
            position_awareness: 0.2,
        }
    }
}

impl AIPersonality {
    pub fn beginner() -> Self {
        Self {
            difficulty: AIDifficulty::Beginner,
            aggression: 0.2,
            tightness: 0.7,
            bluff_frequency: 0.05,
            position_awareness: 0.1,
        }
    }
    
    pub fn intermediate() -> Self {
        Self {
            difficulty: AIDifficulty::Intermediate,
            aggression: 0.4,
            tightness: 0.5,
            bluff_frequency: 0.15,
            position_awareness: 0.6,
        }
    }
}

/// Hand strength categories for AI decision making
#[derive(Debug, PartialEq, PartialOrd)]
enum HandStrength {
    Weak,      // High card, low pairs
    Medium,    // Decent pairs, two pair
    Strong,    // Three of a kind, straights, flushes
    VeryStrong, // Full house, four of a kind, straight/royal flush
}

/// Advanced AI decision making system
pub fn make_advanced_ai_decision(
    player: &Player,
    betting_round: &BettingRound,
    community_cards: &[Card],
    personality: &AIPersonality,
    players_in_hand: usize,
    position: usize, // 0 = early, higher = later
) -> PlayerAction {
    // Evaluate current hand strength
    let hand_strength = evaluate_hand_strength(&player.hole_cards, community_cards);
    
    // Calculate pot odds
    let pot_odds = calculate_pot_odds(betting_round, player);
    
    // Get base action based on difficulty and hand strength
    let base_action = match personality.difficulty {
        AIDifficulty::Beginner => beginner_decision(player, betting_round, &hand_strength),
        AIDifficulty::Intermediate => intermediate_decision(
            player, 
            betting_round, 
            &hand_strength, 
            pot_odds, 
            personality, 
            players_in_hand,
            position
        ),
    };
    
    // Add randomness and personality adjustments
    apply_personality_adjustments(base_action, personality, &hand_strength, betting_round)
}

/// Evaluate the strength of a poker hand
fn evaluate_hand_strength(hole_cards: &[Card], community_cards: &[Card]) -> HandStrength {
    // If we don't have enough cards yet, evaluate based on hole cards only
    if community_cards.len() < 3 {
        return evaluate_preflop_strength(hole_cards);
    }
    
    // Evaluate best 5-card hand
    let evaluation = evaluate_hand(hole_cards, community_cards);
    
    match evaluation.rank {
        crate::poker_rules::HandRank::HighCard => {
            if evaluation.primary_value >= 12 { // Queen high or better
                HandStrength::Weak
            } else {
                HandStrength::Weak
            }
        },
        crate::poker_rules::HandRank::OnePair => {
            if evaluation.primary_value >= 10 { // Pair of Jacks or better
                HandStrength::Medium
            } else {
                HandStrength::Weak
            }
        },
        crate::poker_rules::HandRank::TwoPair => HandStrength::Medium,
        crate::poker_rules::HandRank::ThreeOfAKind => HandStrength::Strong,
        crate::poker_rules::HandRank::Straight => HandStrength::Strong,
        crate::poker_rules::HandRank::Flush => HandStrength::Strong,
        crate::poker_rules::HandRank::FullHouse => HandStrength::VeryStrong,
        crate::poker_rules::HandRank::FourOfAKind => HandStrength::VeryStrong,
        crate::poker_rules::HandRank::StraightFlush => HandStrength::VeryStrong,
        crate::poker_rules::HandRank::RoyalFlush => HandStrength::VeryStrong,
    }
}

/// Evaluate pre-flop hand strength based on hole cards only
fn evaluate_preflop_strength(hole_cards: &[Card]) -> HandStrength {
    if hole_cards.len() != 2 {
        return HandStrength::Weak;
    }
    
    let card1 = &hole_cards[0];
    let card2 = &hole_cards[1];
    
    let rank1_val = card1.rank as u8;
    let rank2_val = card2.rank as u8;
    
    // Pocket pairs
    if rank1_val == rank2_val {
        if rank1_val >= 10 { // Pocket 10s or better
            return HandStrength::Strong;
        } else if rank1_val >= 7 { // Pocket 7s-9s
            return HandStrength::Medium;
        } else {
            return HandStrength::Weak;
        }
    }
    
    // High cards
    let high_card = rank1_val.max(rank2_val);
    let low_card = rank1_val.min(rank2_val);
    
    if high_card >= 12 && low_card >= 10 { // Face cards
        HandStrength::Medium
    } else if high_card >= 10 { // One face card
        HandStrength::Weak
    } else {
        HandStrength::Weak
    }
}

/// Calculate pot odds for the current situation
fn calculate_pot_odds(betting_round: &BettingRound, player: &Player) -> f32 {
    let call_amount = betting_round.current_bet.saturating_sub(player.current_bet);
    if call_amount == 0 {
        return 0.0; // No cost to continue
    }
    
    let pot_after_call = betting_round.pot + call_amount;
    call_amount as f32 / pot_after_call as f32
}

/// Beginner AI decision making - very simple rules
fn beginner_decision(
    player: &Player,
    betting_round: &BettingRound,
    hand_strength: &HandStrength,
) -> PlayerAction {
    let call_amount = betting_round.current_bet.saturating_sub(player.current_bet);
    
    // Can't afford to call
    if call_amount > player.chips {
        return PlayerAction::Fold;
    }
    
    // Free to check
    if call_amount == 0 {
        return PlayerAction::Check;
    }
    
    // Simple decision based on hand strength and cost
    match hand_strength {
        HandStrength::VeryStrong => {
            // Always raise with very strong hands
            let raise_amount = (betting_round.min_raise).min(player.chips / 4);
            if raise_amount > 0 {
                PlayerAction::Raise(raise_amount)
            } else {
                PlayerAction::Call
            }
        },
        HandStrength::Strong => {
            // Call or small raise with strong hands
            if call_amount <= player.chips / 6 {
                PlayerAction::Call
            } else {
                PlayerAction::Fold
            }
        },
        HandStrength::Medium => {
            // Only call if cheap
            if call_amount <= player.chips / 10 {
                PlayerAction::Call
            } else {
                PlayerAction::Fold
            }
        },
        HandStrength::Weak => {
            // Fold weak hands unless very cheap
            if call_amount <= betting_round.min_raise / 2 {
                PlayerAction::Call
            } else {
                PlayerAction::Fold
            }
        },
    }
}

/// Intermediate AI decision making - considers pot odds and position
fn intermediate_decision(
    player: &Player,
    betting_round: &BettingRound,
    hand_strength: &HandStrength,
    pot_odds: f32,
    personality: &AIPersonality,
    players_in_hand: usize,
    position: usize,
) -> PlayerAction {
    let call_amount = betting_round.current_bet.saturating_sub(player.current_bet);
    
    // Can't afford to call
    if call_amount > player.chips {
        return PlayerAction::Fold;
    }
    
    // Free to check
    if call_amount == 0 {
        return match hand_strength {
            HandStrength::VeryStrong | HandStrength::Strong => {
                // Bet for value with strong hands
                let bet_amount = (betting_round.min_raise * 2).min(player.chips / 4);
                if bet_amount > 0 {
                    PlayerAction::Raise(bet_amount)
                } else {
                    PlayerAction::Check
                }
            },
            _ => PlayerAction::Check,
        };
    }
    
    // Calculate hand strength multiplier based on position and players
    let position_factor = if position > players_in_hand / 2 { 1.2 } else { 0.9 };
    let player_factor = if players_in_hand <= 3 { 1.1 } else { 0.95 };
    
    // Pot odds decision making
    let required_equity = pot_odds;
    let estimated_equity = estimate_hand_equity(hand_strength, players_in_hand) * position_factor * player_factor;
    
    match hand_strength {
        HandStrength::VeryStrong => {
            // Always play very strong hands aggressively
            let raise_amount = (betting_round.pot / 2).min(player.chips / 3);
            if raise_amount >= betting_round.min_raise {
                PlayerAction::Raise(raise_amount)
            } else {
                PlayerAction::Call
            }
        },
        HandStrength::Strong => {
            if estimated_equity > required_equity * 0.8 {
                // Call or raise with good odds
                if personality.aggression > 0.4 && position > players_in_hand / 2 {
                    let raise_amount = betting_round.min_raise;
                    if raise_amount <= player.chips / 4 {
                        PlayerAction::Raise(raise_amount)
                    } else {
                        PlayerAction::Call
                    }
                } else {
                    PlayerAction::Call
                }
            } else {
                PlayerAction::Fold
            }
        },
        HandStrength::Medium => {
            if estimated_equity > required_equity * 1.2 {
                PlayerAction::Call
            } else {
                PlayerAction::Fold
            }
        },
        HandStrength::Weak => {
            if estimated_equity > required_equity * 1.5 && call_amount <= betting_round.min_raise {
                PlayerAction::Call
            } else {
                PlayerAction::Fold
            }
        },
    }
}

/// Estimate hand equity (probability of winning) based on hand strength
fn estimate_hand_equity(hand_strength: &HandStrength, players_in_hand: usize) -> f32 {
    let base_equity = match hand_strength {
        HandStrength::Weak => 0.15,
        HandStrength::Medium => 0.35,
        HandStrength::Strong => 0.65,
        HandStrength::VeryStrong => 0.85,
    };
    
    // Adjust for number of opponents
    let opponent_factor = match players_in_hand {
        2 => 1.0,
        3 => 0.9,
        4 => 0.8,
        _ => 0.7,
    };
    
    base_equity * opponent_factor
}

/// Apply personality adjustments to the base decision
fn apply_personality_adjustments(
    base_action: PlayerAction,
    personality: &AIPersonality,
    hand_strength: &HandStrength,
    betting_round: &BettingRound,
) -> PlayerAction {
    let mut rng = rand::thread_rng();
    
    // Add some randomness (5-15% chance to deviate)
    if rng.r#gen::<f32>() < 0.1 {
        match base_action {
            PlayerAction::Call => {
                if personality.aggression > 0.5 && rng.r#gen::<f32>() < personality.aggression {
                    // Sometimes raise instead of call
                    return PlayerAction::Raise(betting_round.min_raise);
                }
            },
            PlayerAction::Fold => {
                if personality.tightness < 0.3 && rng.r#gen::<f32>() < (1.0 - personality.tightness) {
                    // Sometimes call instead of fold (loose play)
                    return PlayerAction::Call;
                }
            },
            _ => {},
        }
    }
    
    // Occasional bluffs with weak hands
    if matches!(hand_strength, HandStrength::Weak) && rng.r#gen::<f32>() < personality.bluff_frequency {
        if betting_round.current_bet == 0 {
            return PlayerAction::Raise(betting_round.min_raise);
        }
    }
    
    base_action
}

/// Component to attach AI personality to players
#[derive(Component, Debug, Clone)]
pub struct AIPlayerComponent {
    pub personality: AIPersonality,
}

impl Default for AIPlayerComponent {
    fn default() -> Self {
        Self {
            personality: AIPersonality::beginner(),
        }
    }
}
