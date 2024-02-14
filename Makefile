.PHONY: clean
clean:
	docker compose down -v

.PHONY: dev
dev:
	docker compose up -d rinha-db rinha-cache

.PHONY: cycle_db
cycle_db: clean dev
	@echo "Database has been cycled"

.PHONY: up
up: clean
	docker compose up --build

run_local:
	RINHA_DB_USER=postgres \
	RINHA_DB_PASSWORD=rinha \
	RINHA_DB_HOST=localhost \
	RINHA_DB_PORT=28732 \
	RINHA_DB_NAME=rinha \
	cargo run