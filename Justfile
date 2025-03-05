check:
	cargo check

build:
	cargo build

test:
	cargo test

clean:
	cargo clean

db_host := "localhost"
db_user := "jonathanwhittle"
db_database := "texts"

dev:
	docker compose up -d

load:
    cargo run -p loader

recog:
	cargo run -p recog

search text="Καὶ ἐγένετο μετὰ τὸ πατάξαι":
    cargo run -p search -- "{{text}}"

