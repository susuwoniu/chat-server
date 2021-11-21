.PHONY: start reload stop init db test keygen client run build serve migrate redeploy log
init:
	sqlx database create
	sqlx migrate run
migrate:
	sqlx migrate run
create-admin:
	cargo run -- admin create
set-admin:
	cargo run -- admin set
db:
	sqlx migrate add $(name)
start:
	RUST_LOG=debug cargo watch -x "run -- server start"
build:
	sqlx migrate run && cargo build --release
serve:
	systemctl restart chat
run:
	RUST_LOG=info cargo run -- server start
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
redeploy:
	./scripts/redeploy.sh
log:
	journalctl -u chat -f