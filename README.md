# Rust DDD Monorepo

A Rust-first, DDD-oriented backend workspace using Axum and SQLx with PostgreSQL. The original web app is out of scope; this workspace focuses on the API and domain logic.

## Quick Start

```bash
# build all crates
cargo build

# run the API
cargo run -p api
```

## Development Workflow

1) Start PostgreSQL:
```bash
docker compose up -d
```

2) Set environment variables (example):
```bash
export DB_USERNAME=postgres
export DB_PASSWORD=postgres
export DB_HOST=localhost
export DB_PORT=5432
export DB_DBNAME=mydb
```

3) Run the API:
```bash
cargo run -p api
```

4) Lint/format/test:
```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### DevContainer

```bash
cp .devcontainer/compose.override.yaml.sample .devcontainer/compose.override.yaml
```

Open the folder in VS Code and choose “Reopen in Container”. The container will build the toolchain and run `cargo build`.

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

## Migrations (SQLx)

Migrations live in `rust-fp/migrations/`.

```bash
sqlx migrate add <name>  # create a new migration
sqlx migrate run         # apply migrations
```

## Architecture (DDD)

- **domain**: pure types and errors
- **application**: use-case orchestration via repository traits
- **infrastructure**: SQLx implementations and external wiring
- **api**: Axum routes + middleware

This preserves the original layered architecture while using Rust idioms and `Result`-based flow.
