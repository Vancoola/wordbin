DB_URL := sqlite:words.db
MIGRATIONS := server/migrations

migrate:
	sqlx migrate run --source $(MIGRATIONS)

migrate-add:
	sqlx migrate add $(name) --source $(MIGRATIONS)

db-reset:
	rm -f words.db
	sqlx database create
	$(MAKE) migrate