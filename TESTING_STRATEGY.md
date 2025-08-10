# Mobile Poker Game Testing Strategy

This document outlines the comprehensive testing strategy for the teach-poker mobile game.

## 🎯 Testing Overview

The game has several key areas that need testing:

### 1. **Core Game Logic**
- ✅ Poker hand evaluation and ranking
- ✅ Card dealing and deck management
- ✅ Player management and game state
- ✅ Betting system and pot management
- ✅ AI decision making

### 2. **Mobile UI Components**
- 📱 Touch input handling
- 📱 Mobile-optimized layouts
- 📱 Responsive design
- 📱 Performance on mobile devices

### 3. **Integration & Systems**
- 🔗 Bevy ECS system interactions
- 🔗 Game state transitions
- 🔗 Audio and animation systems
- 🔗 Teaching and tutorial systems

## 🛠️ Testing Tools

### Quick Testing (Development)
```bash
# Quick development tests
./test_game.sh quick

# Or run the interactive test runner
cargo run --bin test_runner
```

### Comprehensive Testing
```bash
# Run all tests
./test_game.sh

# Specific test categories
./test_game.sh unit          # Unit tests only
./test_game.sh integration   # Integration tests only
./test_game.sh mobile        # Mobile build tests
./test_game.sh performance   # Performance tests
```

### Manual Testing
```bash
# Test game startup and basic functionality
cargo run

# Test simple version for debugging
cargo run --bin test_runner
```

## 📋 Test Categories

### Unit Tests (`cargo test`)
- **Poker Rules**: Hand evaluation, rankings, comparisons
- **Card System**: Deck shuffling, dealing, card representation
- **Player Logic**: Chip management, betting actions
- **AI Behavior**: Decision making, personality traits

### Integration Tests (`tests/`)
- **Game Flow**: Complete game from setup to showdown
- **System Integration**: Bevy systems working together
- **State Management**: Proper state transitions
- **UI Components**: Mobile UI creation and behavior

### Performance Tests
- **Hand Evaluation Speed**: 1000+ evaluations in <100ms
- **Deck Operations**: Shuffling and dealing performance
- **Memory Usage**: No memory leaks during gameplay
- **Frame Rate**: Smooth 60fps on mobile devices

### Mobile-Specific Tests
- **Build Targets**: iOS and Android compilation
- **Touch Input**: Gesture recognition and response
- **Screen Sizes**: Various mobile screen dimensions
- **Battery Usage**: Optimized power consumption

## 🐛 Known Issues & Testing Focus

Based on the current code analysis, focus testing on:

### High Priority Issues
1. **Unused Code**: Many UI functions are not connected
2. **Mobile UI**: Not fully integrated with game logic
3. **Touch Input**: May not be properly handling mobile gestures
4. **Error Handling**: Deck exhaustion and edge cases

### Medium Priority Issues
1. **Performance**: Many unused constants and functions
2. **Code Quality**: 64 compiler warnings to address
3. **Animation System**: Framework exists but not connected
4. **Audio System**: Using placeholder emojis instead of real audio

## 🚀 Test Execution

### Automated Testing Pipeline
```bash
# 1. Prerequisites check
./test_game.sh

# 2. Code quality
cargo clippy
cargo fmt --check

# 3. All test suites
cargo test --all

# 4. Mobile builds
cargo build --target aarch64-apple-ios-sim
cargo build --target aarch64-apple-ios

# 5. Performance verification
cargo test --release performance
```

### Manual Testing Checklist

#### Game Startup
- [ ] Game launches without crashes
- [ ] Mobile UI renders correctly
- [ ] All players are created properly
- [ ] Initial game state is correct

#### Game Flow
- [ ] Cards are dealt properly
- [ ] Betting rounds work correctly
- [ ] AI players make reasonable decisions
- [ ] Hand evaluation is accurate
- [ ] Pot distribution works

#### Mobile UI
- [ ] Touch inputs are responsive
- [ ] Buttons are appropriately sized
- [ ] Text is readable on mobile
- [ ] Layouts work on different screen sizes
- [ ] Performance is smooth

#### Edge Cases
- [ ] All players fold
- [ ] All-in scenarios
- [ ] Deck exhaustion
- [ ] Invalid input handling
- [ ] Network interruption (future)

## 📊 Test Metrics

### Code Coverage Goals
- **Core Logic**: >90% coverage
- **UI Components**: >70% coverage
- **Integration**: >80% coverage
- **Edge Cases**: >60% coverage

### Performance Targets
- **Startup Time**: <3 seconds
- **Frame Rate**: 60fps sustained
- **Memory Usage**: <100MB on mobile
- **Hand Evaluation**: <1ms per hand

## 🔧 Debugging Tools

### Development Tools
```bash
# Interactive test runner with menu
cargo run --bin test_runner

# Real-time game testing
cargo run

# Performance profiling
cargo build --release
# Use platform-specific profiling tools
```

### Logging and Diagnostics
- Game state transitions are logged
- AI decisions are explained
- Performance metrics are tracked
- Error conditions are captured

## 📚 Testing Resources

### Documentation
- `POKER_GAME_CHECKLIST.md` - Feature completion status
- `MOBILE_DEPLOYMENT_CHECKLIST.md` - Mobile-specific requirements
- `test_report.md` - Generated after running tests

### Scripts and Tools
- `test_game.sh` - Comprehensive testing script
- `run_ios_sim.sh` - iOS simulator testing
- `test_runner` binary - Interactive testing tool

## 🎯 Next Steps

1. **Fix Critical Issues**: Address unused code and integration gaps
2. **Enhance Mobile Testing**: Add device-specific test scenarios
3. **Improve Coverage**: Add tests for uncovered code paths
4. **Automate CI/CD**: Set up continuous integration
5. **User Testing**: Get feedback from actual poker players

---

**Quick Start**: Run `./test_game.sh quick` for immediate feedback during development, or `cargo run --bin test_runner` for an interactive testing experience.
