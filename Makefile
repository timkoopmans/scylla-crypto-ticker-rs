.PHONY: list
SHELL := /bin/bash
export DOCKER_BUILDKIT=1

check:
	cargo check --workspace

list:
	@awk -F: '/^[A-z]/ {print $$1}' Makefile | sort

build:
	cargo build
	docker-compose build
	docker-compose push

up:
	docker-compose -f docker-compose.yml up --detach --remove-orphans

down:
	docker-compose -f docker-compose.yml down --remove-orphans

run:
	cargo run -p scylla-crypto-ticker eth usdt

migrate:
	migrations/migrate.sh

reset:
	docker-compose down --remove-orphans scylla
	docker-compose up --detach --force-recreate scylla
	@{ \
	    while ! docker-compose exec scylla curl --fail http://localhost:10000/storage_service/native_transport > /dev/null 2>&1; \
        do sleep 1; \
        done; \
	    echo " ✔ ready"; \
	    migrations/migrate.sh; \
	}
