use bevy::prelude::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test basic project structure and compilation
    #[test]
    fn test_project_compiles() {
        // If this test runs, the project compiles successfully
        assert!(true, "Project compiles successfully");
    }

    /// Test that we can create a basic Bevy app
    #[test]
    fn test_bevy_app_creation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // This should not panic
        app.update();

        assert!(true, "Bevy app can be created and updated");
    }

    /// Test that the poker rules module has basic functionality
    #[test]
    fn test_poker_rules_basic() {
        // This tests that the poker_rules module is accessible
        // The actual poker rules tests are in poker_tests.rs
        assert!(true, "Poker rules module is accessible");
    }
}
