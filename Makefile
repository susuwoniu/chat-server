.PHONY: start reload stop init db test keygen client run
init:
	sqlx database create
	sqlx migrate run
	cargo run -- init
db:
	sqlx migrate add -r $(name)
start:
	cargo watch -x "run -- server"
run:
	cargo run -- server
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