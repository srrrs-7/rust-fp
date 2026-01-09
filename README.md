# Rust DDD Monorepo

A Rust-first, DDD-oriented backend workspace using Axum and SQLx with PostgreSQL. The original web app is out of scope; this workspace focuses on the API and domain logic.

## Quick Start

```bash
# build all crates
cargo build

# run the API
cargo run -p api
```

## Project Structure

```
 rust-fp/
 ├── crates/
 │   ├── domain/          # Domain entities + error types
 │   ├── application/     # Use-cases and service ports
 │   ├── infrastructure/  # SQLx repositories + DB wiring
 │   └── api/             # Axum HTTP layer
 ├── compose.yaml         # Local Postgres
 └── .devcontainer/       # Dev container setup
```

## Commands

```bash
cargo build                # build all crates
cargo run -p api            # run API on :8080
cargo test                 # run tests
cargo fmt                  # format
cargo clippy --all-targets # lint
```

## Configuration

Set the following environment variables for database access:

- `DB_USERNAME`
- `DB_PASSWORD`
- `DB_HOST`
- `DB_PORT` (default 5432)
- `DB_DBNAME`

Use `docker compose up -d` to start PostgreSQL locally.

## Architecture (DDD)

- **domain**: pure types and errors
- **application**: use-case orchestration via repository traits
- **infrastructure**: SQLx implementations and external wiring
- **api**: Axum routes + middleware

This preserves the original layered architecture while using Rust idioms and `Result`-based flow.
