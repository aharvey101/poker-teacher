## Commands

- **Run:** `cargo run`
- **Test:** `cargo test`
- **Build:** `cargo build`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`
- **Check:** `cargo check`

## Code Style

- **Formatting:** Use `cargo fmt` to format the code.
- **Linting:** Use `cargo clippy` to catch common mistakes.
- **Naming:** Follow Rust conventions (`snake_case` for variables and functions, `PascalCase` for types).
- **Error Handling:** Use `Result` and `Option` for error handling. Avoid using `panic!`.
- **Types:** The project uses Bevy's ECS. Define data as components and systems for logic.
- **Imports:** Group imports by crate.
