.PHONY: clean
clean:
	docker compose down -v

.PHONY: dev
dev:
	docker compose up -d rinha-db

.PHONY: cycle_db
cycle_db: clean dev
	@echo "Database has been cycled"

.PHONY: up
up: clean
	docker compose up --build

.PHONY: upd
upd: clean
	docker compose up --build -d

export RINHA_DB_POOL_SIZE=32
export RINHA_API_WORKERS=8
export RINHA_DB_USER=postgres
export RINHA_DB_PASSWORD=rinha
export RINHA_DB_HOST=localhost
export RINHA_DB_PORT=28732
export RINHA_DB_NAME=rinha


.PHONY: run_local
run_local:
	RINHA_HTTP_PORT=9990 \
	cargo run

.PHONY: run_local2
run_local2:
	RINHA_HTTP_PORT=9991 \
	cargo run

.PHONY: haproxy_local
haproxy_local:
	haproxy -f configs/haproxy.cfg