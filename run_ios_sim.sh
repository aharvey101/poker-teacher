#!/bin/bash

# iOS Simulator Runner Script for Teach Poker
# This script builds and runs the app on iOS Simulator via CLI

set -e  # Exit on any error

# Configuration
SIMULATOR_NAME="iPhone 16 Pro"
SIMULATOR_UUID="CD8DC375-A3EB-4121-A312-C47CDDB0F98D"
PROJECT_PATH="gen/apple/teach-poker.xcodeproj"
SCHEME_NAME="teach-poker_iOS"
BUNDLE_ID="local.teachPoker"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 iOS Simulator Build & Run Script${NC}"
echo "=================================="

# Function to print colored output
print_step() {
    echo -e "${GREEN}✓${NC} $1"
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if Xcode is installed
if ! command -v xcodebuild &> /dev/null; then
    print_error "Xcode is not installed or xcodebuild is not in PATH"
    exit 1
fi

# Check if iOS Simulator is available
if ! command -v xcrun &> /dev/null; then
    print_error "xcrun is not available"
    exit 1
fi

print_step "Building Rust library for iOS Simulator..."
cargo build --target aarch64-apple-ios-sim

print_step "Building Xcode project for iOS Simulator..."
xcodebuild -project "$PROJECT_PATH" \
           -scheme "$SCHEME_NAME" \
           -destination "platform=iOS Simulator,name=$SIMULATOR_NAME" \
           build

# Get the built app path
DERIVED_DATA_PATH=$(xcodebuild -project "$PROJECT_PATH" -showBuildSettings | grep "BUILT_PRODUCTS_DIR" | head -1 | awk '{print $3}')
APP_PATH="$DERIVED_DATA_PATH/Teach Poker.app"

print_info "Built app at: $APP_PATH"

print_step "Booting iOS Simulator..."
xcrun simctl boot "$SIMULATOR_UUID" || print_info "Simulator already booted"

print_step "Installing app on simulator..."
xcrun simctl install "$SIMULATOR_UUID" "$APP_PATH"

print_step "Launching app..."
PROCESS_ID=$(xcrun simctl launch "$SIMULATOR_UUID" "$BUNDLE_ID" | awk '{print $2}')

print_step "App launched successfully! Process ID: $PROCESS_ID"
print_info "You can now test touch input on the simulator"

# Optional: Open Simulator.app if not already open
if ! pgrep -f "Simulator.app" > /dev/null; then
    print_info "Opening Simulator.app..."
    open -a Simulator
fi

echo ""
echo -e "${BLUE}🎮 Testing Instructions:${NC}"
echo "1. Wait for the game to start (cards should be dealt)"
echo "2. Tap the betting buttons at the bottom: FOLD, CALL, RAISE"
echo "3. Check the logs for touch input messages"
echo ""
echo -e "${BLUE}📱 Useful Commands:${NC}"
echo "• View app logs: xcrun simctl spawn $SIMULATOR_UUID log stream --predicate 'subsystem contains \"teach_poker\"'"
echo "• Kill app: xcrun simctl terminate $SIMULATOR_UUID $BUNDLE_ID"
echo "• Uninstall app: xcrun simctl uninstall $SIMULATOR_UUID $BUNDLE_ID"
echo "• Shutdown simulator: xcrun simctl shutdown $SIMULATOR_UUID"
