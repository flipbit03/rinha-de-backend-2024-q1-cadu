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

export RINHA__DB__POOL_SIZE=32
export RINHA__DB__USER=postgres
export RINHA__DB__PASSWORD=rinha
export RINHA__DB__HOST=localhost
export RINHA__DB__PORT=2345
export RINHA__DB__NAME=rinha
export RINHA__API__HTTP_PORT=9990
export RINHA__API__WORKERS=8

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