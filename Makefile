.PHONY: start reload stop init db
init:
	sqlx database create
	sqlx migrate run
db:
	sqlx migrate add -r $(name)
start:
	systemctl start caddy
stop:
	systemctl stop caddy
reload:
	systemctl reload caddy