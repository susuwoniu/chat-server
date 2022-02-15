.PHONY: start reload stop init db test keygen client run build serve migrate redeploy log init-templates create-user
init:
	sqlx database create
	sqlx migrate run
migrate:
	sqlx migrate run
create-admin:
	cargo run -- admin create
create-user:
	cargo run -- admin create-user
set-admin:
	cargo run -- admin set
init-templates:
	cargo run -- admin init-templates ./docs/product/post-template.yml
db:
	sqlx migrate add $(name)
start:
	cargo watch -x "run -- server start"
build:
	sqlx migrate run && cargo build --release
serve:
	systemctl restart chat
run:
	RUST_LOG=info cargo run -- server start
stop:
	systemctl stop chat
reload:
	systemctl reload chat
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