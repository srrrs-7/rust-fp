.PHONY: build run test fmt clippy check db-up db-down

build:
	cargo build

run:
	cargo run -p api

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt clippy test

db-up:
	docker compose up -d

db-down:
	docker compose down
