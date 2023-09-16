include .env

.PHONY: watch.linter
watch.linter:
	cargo watch -s 'pnpm format'

.PHONY: watch.tailwind
watch.tailwind:
	pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch

.PHONY: watch.debug
watch.debug:
	cargo watch -x run

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
