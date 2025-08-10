# Bevy Mobile Deployment Checklist

This checklist outlines the steps required to port the Bevy-based poker game to iOS.

## 1. Environment Setup & Toolchains


- [x] **Install iOS Toolchain (macOS only):**
    - [x] Install Xcode from the Mac App Store.
    - [x] Install the iOS Rust target: `rustup target add aarch64-apple-ios`.

- [x] **Install `cargo-mobile`:**
    - [x] Run `cargo install cargo-mobile` to get the build tool.

## 2. Project Configuration

- [ ] **Configure `Cargo.toml` for `cargo-mobile`:**
    - [x] Add `[package.metadata.ios]` section with `bundle_identifier` and `bundle_name`.

- [ ] **Initialize Mobile Projects:**
    - [ ] Run `cargo mobile init` to generate the native `ios` project folder.

## 3. Code & Asset Adaptations

- [x] **Input System:**
    - [x] Create a new Bevy system to handle `TouchInput` events.
    - [x] Map touch coordinates to UI elements (buttons, sliders).
    - [x] Remove or disable keyboard/mouse-specific input systems.
    - [x] Create unified input system that handles both touch and mouse input.
    - [x] Fix button component queries to work with touch events.
    - [x] Add debug logging for touch events and button detection.

- [ ] **UI Responsiveness:**
    - [x] Refactor UI components to use relative positioning (e.g., `Val::Percent`) instead of fixed pixels.
    - [ ] Test layouts on various screen sizes and aspect ratios.
    - [ ] (Optional) Create distinct layouts for portrait and landscape modes.
    - [ ] **Card Visibility Improvements:**
        - [ ] Make player's hole cards larger and more prominent
        - [x] Add card back designs for hidden cards instead of blank rectangles
        - [ ] Improve card spacing and positioning for better mobile readability
    - [ ] **UI Polish & Mobile UX:**
        - [x] Add visual feedback for button presses (highlight/press states)
        - [x] Increase button sizes for easier touch interaction
        - [x] Add rounded corners and modern mobile styling to buttons
        - [x] Improve text readability with better contrast and sizing
        - [ ] Add card animations for dealing, revealing, and folding
    - [ ] **Game Information Display:**
        - [x] Make pot amount more prominent with better typography
        - [x] Improve player information layout (name, chips, status)
        - [ ] Add visual indicators for current player turn
        - [ ] Display betting amounts more clearly during betting rounds

- [ ] **Performance Optimization:**
    - [ ] Optimize and compress textures for mobile devices.
    - [ ] Reduce the size and complexity of audio assets.
    - [ ] Profile the game on target devices and simplify performance-intensive shaders or systems.
    - [ ] **Mobile-Specific Optimizations:**
        - [ ] Implement texture atlasing for cards to reduce draw calls
        - [ ] Add dynamic quality settings based on device performance
        - [ ] Optimize UI rendering for mobile GPUs

## 4. Build, Test, & Deploy


- [x] **iOS:**
    - [x] Connect an iOS device or start a simulator.
    - [x] Run `cargo mobile run ios` to build and launch the app.
    - [x] Test all features on the device/simulator.
    - [x] Create CLI scripts for building and deploying to simulator.
    - [x] Set up Cargo aliases for quick iOS builds.

- [ ] **App Store Preparation:**
    - [ ] Create app icons in various required resolutions.
    - [ ] Prepare screenshots and marketing text for store listings.
    - [ ] Configure signing certificates for both platforms.

## 5. Platform-Specific Polish

- [ ] **Handle App Lifecycle Events:**
    - [x] Implement systems to handle app pausing/resuming (e.g., when a call comes in).

- [ ] **Platform Integrations (Optional):**
    - [ ] Integrate with Game Center (iOS) for achievements or leaderboards.
    - [x] Add haptic feedback for button presses.
    - [ ] **Enhanced Mobile Features:**
        - [ ] Add sound effects for card dealing, button presses, and game events
        - [ ] Implement swipe gestures for alternative navigation
        - [ ] Add pinch-to-zoom for better card visibility
        - [ ] Support device rotation with adaptive layouts
        - [ ] Add vibration patterns for different game events (wins, losses, etc.)

## 6. Visual & UX Improvements (Based on Current iOS Build)

- [ ] **Card System Enhancements:**
    - [x] Replace blank card placeholders with proper card back designs
    - [ ] Implement smooth card flip animations for reveals
    - [ ] Add card shadows and depth for better visual hierarchy
    - [ ] Increase player hole card size for better visibility

- [ ] **Button & Interaction Polish:**
    - [x] Add button press animations and visual feedback
    - [x] Implement proper button states (normal, pressed, disabled)
    - [x] Add subtle gradient or shadow effects to buttons
    - [x] Ensure minimum touch target size (44pt on iOS)

- [ ] **Typography & Information Display:**
    - [x] Improve readability of pot amount and player chips
    - [x] Add better visual separation between game elements
    - [ ] Implement proper color scheme for day/night modes
    - [ ] Add icons for betting actions (fold, call, raise)

- [ ] **Game State Communication:**
    - [ ] Add clearer turn indicators (highlighting current player)
    - [ ] Show betting history in a more prominent way
    - [ ] Add animation for pot updates and chip movements
    - [ ] Implement better game phase indicators
