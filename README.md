# hawkeye

A service for sending webhooks from scraped webpage data

# Development

## Prerequisites
You need to have the following installed:
- Docker & Docker Compose
- Cargo
- sea-orm-cli (`cargo install sea-orm-cli@0.9.2`)

## Run locally
```bash
docker-compose --env-file .env up -d --build
```

## Migrations
```bash
# Generate a new migration
sea-orm-cli migrate generate migration_name

# Re-generate database entities after running a migration
sea-orm-cli generate entity -u postgresql://hawkeye:secret@localhost:5433/hawkeyedb -s public -o hawkeye-entity/src/entity
```