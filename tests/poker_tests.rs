use bevy::prelude::*;
use teach_poker::*;

/// Focused tests for poker-specific logic and rules
/// 
/// These tests validate that the poker game logic is correct,
/// including hand evaluation, betting rules, and game flow.
#[cfg(test)]
mod poker_rules_tests {
    use super::*;
    use teach_poker::{
        cards::{Card, Deck, Rank, Suit},
        player::{Player, PlayerType},
        poker_rules::{evaluate_hand, hand_rank_name, HandEvaluation, HandRank},
        betting::BettingRound,
    };

    /// Test basic poker hand rankings
    #[test]
    fn test_poker_hand_rankings() {
        // Test that hand rankings are correctly ordered
        assert!(HandRank::RoyalFlush as u8 > HandRank::StraightFlush as u8);
        assert!(HandRank::StraightFlush as u8 > HandRank::FourOfAKind as u8);
        assert!(HandRank::FourOfAKind as u8 > HandRank::FullHouse as u8);
        assert!(HandRank::FullHouse as u8 > HandRank::Flush as u8);
        assert!(HandRank::Flush as u8 > HandRank::Straight as u8);
        assert!(HandRank::Straight as u8 > HandRank::ThreeOfAKind as u8);
        assert!(HandRank::ThreeOfAKind as u8 > HandRank::TwoPair as u8);
        assert!(HandRank::TwoPair as u8 > HandRank::OnePair as u8);
        assert!(HandRank::OnePair as u8 > HandRank::HighCard as u8);
    }

    /// Test royal flush detection
    #[test]
    fn test_royal_flush_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
        ];
        let community_cards = vec![
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Ten),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::RoyalFlush);
        assert_eq!(hand_rank_name(&evaluation.rank), "Royal Flush");
    }

    /// Test straight flush detection
    #[test]
    fn test_straight_flush_detection() {
        let hole_cards = vec![
            Card::new(Suit::Spades, Rank::Nine),
            Card::new(Suit::Spades, Rank::Eight),
        ];
        let community_cards = vec![
            Card::new(Suit::Spades, Rank::Seven),
            Card::new(Suit::Spades, Rank::Six),
            Card::new(Suit::Spades, Rank::Five),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::StraightFlush);
        assert_eq!(hand_rank_name(&evaluation.rank), "Straight Flush");
    }

    /// Test four of a kind detection
    #[test]
    fn test_four_of_a_kind_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::FourOfAKind);
        assert_eq!(hand_rank_name(&evaluation.rank), "Four of a Kind");
    }

    /// Test full house detection
    #[test]
    fn test_full_house_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Spades, Rank::Queen),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::FullHouse);
        assert_eq!(hand_rank_name(&evaluation.rank), "Full House");
    }

    /// Test flush detection
    #[test]
    fn test_flush_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ten),
        ];
        let community_cards = vec![
            Card::new(Suit::Hearts, Rank::Eight),
            Card::new(Suit::Hearts, Rank::Five),
            Card::new(Suit::Hearts, Rank::Three),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::Flush);
        assert_eq!(hand_rank_name(&evaluation.rank), "Flush");
    }

    /// Test straight detection
    #[test]
    fn test_straight_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Diamonds, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Ten),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::Straight);
        assert_eq!(hand_rank_name(&evaluation.rank), "Straight");
    }

    /// Test three of a kind detection
    #[test]
    fn test_three_of_a_kind_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Spades, Rank::Queen),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Spades, Rank::Nine),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::ThreeOfAKind);
        assert_eq!(hand_rank_name(&evaluation.rank), "Three of a Kind");
    }

    /// Test two pair detection
    #[test]
    fn test_two_pair_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Spades, Rank::Nine),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::TwoPair);
        assert_eq!(hand_rank_name(&evaluation.rank), "Two Pair");
    }

    /// Test one pair detection
    #[test]
    fn test_one_pair_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Nine),
            Card::new(Suit::Spades, Rank::Seven),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::OnePair);
        assert_eq!(hand_rank_name(&evaluation.rank), "One Pair");
    }

    /// Test high card detection
    #[test]
    fn test_high_card_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Seven),
            Card::new(Suit::Spades, Rank::Five),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::HighCard);
        assert_eq!(hand_rank_name(&evaluation.rank), "High Card");
    }

    /// Test hand comparison - higher ranks beat lower ranks
    #[test]
    fn test_hand_comparison() {
        let royal_flush = HandEvaluation {
            rank: HandRank::RoyalFlush,
            primary_value: 14, // Ace high
            secondary_value: 0,
            kickers: vec![],
        };
        
        let straight_flush = HandEvaluation {
            rank: HandRank::StraightFlush,
            primary_value: 9, // Nine high
            secondary_value: 0,
            kickers: vec![],
        };
        
        assert!(royal_flush > straight_flush, "Royal flush should beat straight flush");
        
        let four_of_a_kind = HandEvaluation {
            rank: HandRank::FourOfAKind,
            primary_value: 14, // Aces
            secondary_value: 0,
            kickers: vec![],
        };
        
        assert!(straight_flush > four_of_a_kind, "Straight flush should beat four of a kind");
    }

    /// Test deck creation and shuffling
    #[test]
    fn test_deck_creation_and_shuffling() {
        let mut deck = Deck::default();
        
        // Deck should have 52 cards
        assert_eq!(deck.cards.len(), 52, "Deck should have 52 cards");
        
        // Should have 4 suits of 13 cards each
        let hearts: Vec<_> = deck.cards.iter().filter(|c| c.suit == Suit::Hearts).collect();
        let diamonds: Vec<_> = deck.cards.iter().filter(|c| c.suit == Suit::Diamonds).collect();
        let clubs: Vec<_> = deck.cards.iter().filter(|c| c.suit == Suit::Clubs).collect();
        let spades: Vec<_> = deck.cards.iter().filter(|c| c.suit == Suit::Spades).collect();
        
        assert_eq!(hearts.len(), 13, "Should have 13 hearts");
        assert_eq!(diamonds.len(), 13, "Should have 13 diamonds");
        assert_eq!(clubs.len(), 13, "Should have 13 clubs");
        assert_eq!(spades.len(), 13, "Should have 13 spades");
        
        // Test shuffling
        deck.shuffle();
        
        // After shuffling, should still have same cards but likely different order
        assert_eq!(deck.cards.len(), 52, "Should still have 52 cards after shuffle");
        
        // Very unlikely to have same order after shuffle (but theoretically possible)
        // Just verify we can shuffle without panic
        deck.shuffle();
        assert_eq!(deck.cards.len(), 52, "Should still have 52 cards after second shuffle");
    }

    /// Test card dealing
    #[test]
    fn test_card_dealing() {
        let mut deck = Deck::default();
        
        // Deal cards one by one
        let card1 = deck.deal();
        let card2 = deck.deal();
        
        assert!(card1.is_some(), "Should be able to deal first card");
        assert!(card2.is_some(), "Should be able to deal second card");
        assert_eq!(deck.cards.len(), 50, "Should have 50 cards left after dealing 2");
        
        // Deal all remaining cards
        for _ in 0..50 {
            let card = deck.deal();
            assert!(card.is_some(), "Should be able to deal card");
        }
        
        assert_eq!(deck.cards.len(), 0, "Deck should be empty after dealing all cards");
        
        // Trying to deal from empty deck should return None
        let no_card = deck.deal();
        assert!(no_card.is_none(), "Should not be able to deal from empty deck");
    }

    /// Test player creation and basic functionality
    #[test]
    fn test_player_creation() {
        let human_player = Player::new(0, PlayerType::Human, 1000, Vec3::ZERO);
        let ai_player = Player::new(1, PlayerType::AI, 1500, Vec3::new(100.0, 0.0, 0.0));
        
        assert_eq!(human_player.id, 0);
        assert_eq!(human_player.player_type, PlayerType::Human);
        assert_eq!(human_player.chips, 1000);
        assert!(!human_player.has_folded);
        assert_eq!(human_player.current_bet, 0);
        assert!(human_player.hole_cards.is_empty());
        
        assert_eq!(ai_player.id, 1);
        assert_eq!(ai_player.player_type, PlayerType::AI);
        assert_eq!(ai_player.chips, 1500);
    }

    /// Test betting round creation and management
    #[test]
    fn test_betting_round_management() {
        let player_ids = vec![0, 1, 2];
        let small_blind = 10;
        let betting_round = BettingRound::new(player_ids.clone(), small_blind);
        
        assert_eq!(betting_round.current_bet, 20); // Big blind
        assert_eq!(betting_round.min_raise, 20);
        assert_eq!(betting_round.players_to_act, player_ids);
        assert!(!betting_round.betting_complete);
        assert_eq!(betting_round.pot, 0);
    }

    /// Test betting round reset
    #[test]
    fn test_betting_round_reset() {
        let mut betting_round = BettingRound::new(vec![0, 1, 2], 10);
        
        // Modify the betting round
        betting_round.current_bet = 50;
        betting_round.betting_complete = true;
        betting_round.pot = 100;
        
        // Reset for new round
        betting_round.reset_for_new_round(vec![0, 1]);
        
        assert_eq!(betting_round.current_bet, 0);
        assert!(!betting_round.betting_complete);
        assert_eq!(betting_round.players_to_act, vec![0, 1]);
        // Note: pot is not reset in reset_for_new_round, that's expected
    }

    /// Test betting round completion detection
    #[test]
    fn test_betting_round_completion() {
        let mut betting_round = BettingRound::new(vec![0, 1, 2], 10);
        
        assert!(!betting_round.is_complete());
        
        // Remove all players
        while betting_round.next_player().is_some() {
            // Keep removing players
        }
        
        assert!(betting_round.is_complete());
    }

    /// Test card addition to player
    #[test]
    fn test_player_card_management() {
        let mut player = Player::new(0, PlayerType::Human, 1000, Vec3::ZERO);
        
        assert!(player.hole_cards.is_empty());
        
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Spades, Rank::King);
        
        player.add_card(card1);
        player.add_card(card2);
        
        assert_eq!(player.hole_cards.len(), 2);
        assert_eq!(player.hole_cards[0], card1);
        assert_eq!(player.hole_cards[1], card2);
        
        player.clear_cards();
        assert!(player.hole_cards.is_empty());
    }

    /// Test low ace straight (wheel)
    #[test]
    fn test_low_ace_straight() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Two),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Three),
            Card::new(Suit::Diamonds, Rank::Four),
            Card::new(Suit::Hearts, Rank::Five),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        assert_eq!(evaluation.rank, HandRank::Straight);
    }

    /// Test that non-straights are correctly identified
    #[test]
    fn test_non_straight_detection() {
        let hole_cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community_cards = vec![
            Card::new(Suit::Clubs, Rank::Jack),
            Card::new(Suit::Diamonds, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Seven),
        ];
        
        let evaluation = evaluate_hand(&hole_cards, &community_cards);
        // Should be high card (Ace high), not a straight
        assert_eq!(evaluation.rank, HandRank::HighCard);
    }
}
