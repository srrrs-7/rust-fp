# Project Overview

## Purpose
Rust DDD backend workspace focused on API + domain logic for task management.

## Tech Stack
- Language: Rust 2021
- Runtime: Tokio
- Web: Axum
- DB: PostgreSQL via SQLx
- Logging: tracing + tracing-subscriber

## Structure
- `crates/domain`: entities, value objects, AppError
- `crates/application`: use-cases and repository traits
- `crates/infrastructure`: SQLx repositories, DB config
- `crates/api`: Axum routes + middleware

## Entry Point
- `crates/api/src/main.rs`
