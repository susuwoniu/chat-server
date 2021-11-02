.PHONY: start reload stop init db test keygen client run build serve
init:
	sqlx database create
	sqlx migrate run
	cargo run -- init
db:
	sqlx migrate add -r $(name)
start:
	RUST_LOG=info cargo watch -x "run -- server"
build:
	cargo build --release
serve:
	systemctl restart chat
run:
	RUST_LOG=info cargo run -- server
stop:
	systemctl stop caddy
reload:
	systemctl reload caddy
test:
	cargo test -- --nocapture
keygen:
	cargo run -- keygen
client:
	cargo run -- client