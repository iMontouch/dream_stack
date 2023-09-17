include .env

.PHONY: lint
lint:
	pnpm format

.PHONY: tailwind
tailwind:
	pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css

.PHONY: debug
debug: generate tailwind lint
	cargo run



.PHONY: fresh
fresh:
	sea-orm-cli migrate fresh

.PHONY: migrate
migrate:
	sea-orm-cli migrate up

.PHONY: generate
generate: fresh migrate
	rm -rf entity/src
	sea-orm-cli generate entity --with-serde both --lib -o entity/src
