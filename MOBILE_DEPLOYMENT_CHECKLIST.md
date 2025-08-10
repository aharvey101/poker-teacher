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

- [ ] **Input System:**
    - [x] Create a new Bevy system to handle `TouchInput` events.
    - [x] Map touch coordinates to UI elements (buttons, sliders).
    - [x] Remove or disable keyboard/mouse-specific input systems.

- [ ] **UI Responsiveness:**
    - [x] Refactor UI components to use relative positioning (e.g., `Val::Percent`) instead of fixed pixels.
    - [ ] Test layouts on various screen sizes and aspect ratios.
    - [ ] (Optional) Create distinct layouts for portrait and landscape modes.

- [ ] **Performance Optimization:**
    - [ ] Optimize and compress textures for mobile devices.
    - [ ] Reduce the size and complexity of audio assets.
    - [ ] Profile the game on target devices and simplify performance-intensive shaders or systems.

## 4. Build, Test, & Deploy


- [ ] **iOS:**
    - [ ] Connect an iOS device or start a simulator.
    - [ ] Run `cargo mobile run ios` to build and launch the app.
    - [ ] Test all features on the device/simulator.

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
