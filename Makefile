.PHONY: check build test reader api

check:
	cargo check

build:
	cargo build

test:
	cargo test

book ?= Genesis
chapter ?= 1
verse ?= 1
run:
	@cargo build
	@DB_HOST=localhost DB_USER=jonathanwhittle DB_DATABASE=texts ./target/debug/text-insights -B $(book) -C $(chapter) -V $(verse)

greek:
	@DB_HOST=localhost DB_USER=jonathanwhittle DB_DATABASE=texts cargo run --bin greek

