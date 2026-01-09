# Rust DDD Constitution

## Core Principles

### I. DDD Layering
Domain is pure and side-effect free. Application coordinates use-cases via traits. Infrastructure owns IO. API is a thin adapter.

### II. Functional Error Handling
Use `Result<T, AppError>` consistently. Avoid panics and hidden side effects.

### III. Explicit Boundaries
Cross-layer calls are one direction only: API -> Application -> Domain/Infrastructure.

## Quality Gates
- `cargo fmt` clean
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` for relevant crates

## Governance
This constitution overrides ad-hoc practices. Amendments require documenting rationale and scope.

**Version**: 1.0.0 | **Ratified**: 2025-01-01 | **Last Amended**: 2025-01-01
