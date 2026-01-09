# Repository Guidelines

## Project Structure & Module Organization
- `crates/` holds workspace crates:
  - `crates/domain/`: domain entities and errors.
  - `crates/application/`: use-cases and service ports.
  - `crates/infrastructure/`: SQLx repositories and DB wiring.
  - `crates/api/`: Axum HTTP layer.
- `migrations/` contains SQLx migrations.
- `compose.yaml` runs local PostgreSQL.

## Build, Test, and Development Commands
- `cargo build`: build all workspace crates.
- `cargo run -p api`: run the API server (default :8080).
- `cargo test`: run workspace tests.
- `cargo fmt`: format code with rustfmt.
- `cargo clippy --all-targets --all-features -- -D warnings`: lint with Clippy, deny warnings.
- `make check`: run fmt, clippy, and tests (see `Makefile`).
- `make db-up` / `make db-down`: start/stop local Postgres via Docker Compose.

## Coding Style & Naming Conventions
- Rust edition 2021; follow standard Rust naming (snake_case for functions/modules, CamelCase for types).
- Indentation: 4 spaces; format with `cargo fmt`.
- Prefer `Result`-based flow and keep DDD layering boundaries intact (domain → application → infrastructure → api).
- Spelling is checked via `cspell.config.yaml` when used; keep identifiers readable.

## Testing Guidelines
- Use Rust’s built-in test framework (`#[test]` / `#[tokio::test]` as needed).
- Place tests near the code or in `tests/` if added later; name tests after behavior (e.g., `creates_user_with_valid_input`).
- Run `cargo test` locally before PRs; no explicit coverage target is defined.

## Commit & Pull Request Guidelines
- Git history does not show a strict commit format; use concise, imperative summaries (e.g., "add user repository").
- PRs should include: brief description, rationale, and test commands run.
- Link related issues and include migration notes when `migrations/` changes.

## Configuration & Local Setup
- Required env vars for DB: `DB_USERNAME`, `DB_PASSWORD`, `DB_HOST`, `DB_PORT`, `DB_DBNAME`.
- Use `docker compose up -d` (or `make db-up`) to start the database before running the API.
