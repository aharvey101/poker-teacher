# Poker Teacher Game

A Rust-based poker game built with Bevy ECS that teaches poker rules through interactive gameplay.

## Project Goals

- **Educational Focus**: Teach poker rules and strategy through gameplay
- **Simple Graphics**: Blocky, geometric style for clarity
- **Interactive Learning**: Contextual hints and rule explanations
- **Progressive Difficulty**: Start simple, build complexity

## Current Status

✅ **Phase 1: Foundation**
- Basic Bevy app setup
- Core data structures (Card, Player, GameState)
- 3-player setup (1 human, 2 AI)

🚧 **In Progress: Phase 2**
- Game logic implementation
- Basic UI rendering
- Hand evaluation system

## Quick Start

```bash
# Run the game
cargo run

# Run tests
cargo test

# Build for release
cargo build --release
```

## Architecture

### Core Components
- **Card System**: Standard 52-card deck with shuffle/deal mechanics
- **Player System**: Human and AI players with chips and actions
- **Game State**: State machine for game flow (dealing, betting, showdown)
- **Teaching System**: Contextual hints and rule explanations

### Technology Stack
- **Engine**: Bevy 0.14 (ECS, rendering, input)
- **Language**: Rust (performance, safety)
- **Graphics**: 2D sprites and geometric shapes
- **AI**: Rule-based decision trees

## Development Plan

See [POKER_GAME_CHECKLIST.md](POKER_GAME_CHECKLIST.md) for detailed development roadmap.

### Current Priorities
1. Implement basic card rendering
2. Add game flow systems
3. Create simple UI for human player actions
4. Implement basic AI decision making

## Contributing

This is a learning project, but suggestions and improvements are welcome!

## License

MIT License - See LICENSE file for details.
