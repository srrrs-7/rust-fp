.PHONY: build run run-bin test fmt clippy check db-up db-down hooks migrate-add migrate-run init-firewall

build:
	cargo build

run:
	cargo run -p api

run-bin: build
	./target/debug/api

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

hooks:
	git config core.hooksPath "$$(pwd)/.githooks"
	chmod +x .githooks/pre-commit .githooks/pre-push
	@echo "Git hooks installed to $$(pwd)/.githooks"

migrate-add:
	@if [ -z "$(name)" ]; then echo "Usage: make migrate-add name=your_migration_name"; exit 1; fi
	sqlx migrate add "$$(date +%Y%m%d%H%M%S)_$(name)"

migrate-run:
	@if [ -z "$$DATABASE_URL" ]; then \
		if [ -z "$$DB_USERNAME" ] || [ -z "$$DB_PASSWORD" ] || [ -z "$$DB_HOST" ] || [ -z "$$DB_DBNAME" ]; then \
			echo "Set DATABASE_URL or DB_USERNAME/DB_PASSWORD/DB_HOST/DB_DBNAME (and optional DB_PORT)."; \
			exit 1; \
		fi; \
		DB_PORT="$${DB_PORT:-5432}"; \
		export DATABASE_URL="postgresql://$${DB_USERNAME}:$${DB_PASSWORD}@$${DB_HOST}:$${DB_PORT}/$${DB_DBNAME}"; \
	fi; \
	sqlx migrate run

init-firewall:
	./.devcontainer/init-firewall.sh
