# iOS UI Testing Summary for Teach Poker

This document summarizes the UI tests created for the Teach Poker iOS app.

## Available Tests

### 1. Simple UI Test (`./simple_ui_test.sh`)
**Recommended for beginners** ✨

The most basic UI test possible:
- Builds the app from scratch
- Installs on iOS simulator
- Launches the app
- Takes screenshots at different stages
- Tests basic touch interaction
- Verifies app remains responsive

**Usage:**
```bash
./simple_ui_test.sh
```

**Output:**
- `simple_ui_test_result.png` - Initial app state
- `simple_ui_test_after_tap.png` - After interaction
- `simple_ui_test_final.png` - Final app state

### 2. Minimal UI Test (`./minimal_ui_test.sh`)
**Quick verification test**

Fast test that checks if the app works:
- Builds only if needed
- Quick launch verification
- Single screenshot capture
- Interactive pause for manual testing

**Usage:**
```bash
./minimal_ui_test.sh
```

### 3. Basic UI Test (`./basic_ui_test.sh`)
**Command-line verification**

Tests app launch with process monitoring:
- Builds and installs app
- Launches with process verification
- Takes screenshot for verification
- Interactive completion

**Usage:**
```bash
./basic_ui_test.sh
```

### 4. XCTest UI Tests (`./run_ui_tests.sh`)
**Advanced automated testing** (Currently in development)

Comprehensive test suite using Apple's XCTest framework:
- Multiple test cases for different UI elements
- Automated button interaction testing
- Stability testing over time
- Detailed test result reporting
- Screenshot attachments for each test

**Note:** This requires some additional setup to configure the test scheme properly.

## Test Requirements

### Prerequisites
- Xcode installed with iOS Simulator
- Rust toolchain with iOS targets
- `cargo-mobile2` for iOS builds
- iOS Simulator running (iPhone 16 Pro recommended)

### iOS Targets Required
```bash
rustup target add aarch64-apple-ios-sim
```

## Test Results

All tests generate screenshots that can be manually inspected to verify:
- ✅ Mobile UI layout renders correctly
- ✅ Poker game elements are visible (cards, buttons, pot display)
- ✅ Touch interactions work without crashes
- ✅ App remains stable during usage

## Mobile UI Features Tested

The tests verify these mobile UI components from `mobile_ui.rs`:
- **Top Section**: AI players and game info
- **Middle Section**: Community cards and teaching panel
- **Bottom Section**: Player cards and betting controls
- **Betting Buttons**: FOLD, CALL, RAISE
- **Raise Controls**: +$5, -$5 adjustment buttons
- **Game Information**: Pot display, game phase

## Quick Start

For the simplest possible test, run:
```bash
./simple_ui_test.sh
```

This will build your app, launch it in the iOS simulator, and generate screenshots showing that your mobile poker UI works correctly.

## Troubleshooting

If tests fail:
1. Ensure iOS Simulator is available and booted
2. Check that Rust iOS targets are installed
3. Verify Xcode command line tools are working
4. Try running individual test steps manually

## Next Steps

- Review generated screenshots to verify UI appearance
- Run tests after UI changes to ensure nothing breaks
- Extend XCTest suite for more specific poker game logic testing
- Add performance testing for animations and card rendering

---

**Created for the Teach Poker mobile app - the simplest UI tests to verify your iOS poker game works! 🃏📱**
