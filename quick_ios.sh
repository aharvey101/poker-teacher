#!/bin/bash

# Quick iOS Simulator deployment script
# Only rebuilds if source files have changed

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Quick iOS Simulator Deployment${NC}"

APP_NAME="teach-poker"
SCHEME_NAME="teach-poker_iOS"
BUNDLE_ID="local.teachPoker"

# Check if we need to rebuild Rust library
RUST_MODIFIED=$(find src -name "*.rs" -newer target/aarch64-apple-ios-sim/debug/lib${APP_NAME}.a 2>/dev/null | wc -l)

if [ "$RUST_MODIFIED" -gt 0 ] || [ ! -f "target/aarch64-apple-ios-sim/debug/lib${APP_NAME}.a" ]; then
    echo -e "${YELLOW}📦 Rebuilding Rust library (source changes detected)...${NC}"
    cargo build --target aarch64-apple-ios-sim
else
    echo -e "${GREEN}✅ Rust library up to date${NC}"
fi

# Navigate to Xcode project
cd gen/apple

# Check if we need to rebuild Xcode project
XCODE_MODIFIED=$(find . -name "*.swift" -o -name "*.m" -o -name "*.mm" -newer "build/Build/Products/debug-iphonesimulator/Teach Poker.app/Teach Poker" 2>/dev/null | wc -l)

if [ "$XCODE_MODIFIED" -gt 0 ] || [ ! -d "build/Build/Products/debug-iphonesimulator/Teach Poker.app" ]; then
    echo -e "${YELLOW}🔨 Building Xcode project...${NC}"
    xcodebuild -project ${APP_NAME}.xcodeproj -scheme ${SCHEME_NAME} -configuration debug -destination 'generic/platform=iOS Simulator' -derivedDataPath build
else
    echo -e "${GREEN}✅ Xcode project up to date${NC}"
fi

# Install and launch on simulator
echo -e "${BLUE}📱 Installing on simulator...${NC}"
xcrun simctl install booted "build/Build/Products/debug-iphonesimulator/Teach Poker.app"

echo -e "${BLUE}🎮 Launching app...${NC}"
xcrun simctl launch booted $BUNDLE_ID

echo -e "${GREEN}✅ App launched successfully!${NC}"
echo -e "${YELLOW}💡 Use 'xcrun simctl list devices' to see all simulators${NC}"
