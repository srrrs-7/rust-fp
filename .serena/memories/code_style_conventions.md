# Code Style Conventions

- Use `Result<T, AppError>` for error flow
- Keep domain types pure and serializable
- Prefer small, composable functions
- Format with `cargo fmt`
- Lint with `cargo clippy --all-targets -- -D warnings`
