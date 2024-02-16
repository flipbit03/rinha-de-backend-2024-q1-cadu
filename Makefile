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
	RINHA__HTTP__PORT=9999 \
	RINHA__WORKERS=32 \
	RINHA__DB__USER=postgres \
	RINHA__DB__PASSWORD=rinha \
	RINHA__DB__HOST=localhost \
	RINHA__DB__PORT=28732 \
	RINHA__DB__POOL_SIZE=32 \
	RINHA__DB__NAME=rinha \
	RINHA__REDIS__HOST=localhost \
	RINHA__REDIS__PORT=16379 \
	RINHA__REDIS__DB=0 \
	RINHA__REDIS__POOL_SIZE=32 \
	cargo run