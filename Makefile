.PHONY: check build test reader api

check:
	cargo check

build:
	cargo build

test:
	cargo test

clean:
	cargo clean

db_host ?= localhost
db_user ?= jonathanwhittle
db_database ?= texts
bin ?= greek
run:
	@DB_HOST=$(db_host) DB_USER=$(db_user) DB_DATABASE=$(db_database) cargo run --bin $(bin)

