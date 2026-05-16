<p align="center">
  <img src="docs/wordmark.png" width="320" alt="wordbin" />
</p>

# wordbin

> One place to drop English words you don't know yet.

<p align="center">
  <img src="docs/screenshots/popup.png" width="240" alt="Capture popup" />
  <img src="docs/screenshots/words.png" width="240" alt="Words list" />
  <img src="docs/screenshots/settings.png" width="240" alt="Settings" />
</p>

I encounter unknown words everywhere — subtitles, browser tabs, podcasts, books. Every tool I tried solved one source but not the others. I ended up with words scattered across a phone app, a browser extension, and a notebook that I never actually reviewed.

Wordbin is a self-hosted daemon that gives me a single endpoint to send words from anywhere. A browser extension for the web, a CLI for the terminal, whatever else later. Everything lands in one SQLite file.

---

## What it is

- **wordbin-server** — Axum daemon, exposes a REST API, stores words in SQLite; ships an admin CLI for token issuance
- **wordbin-extension** — Browser extension (Leptos/WASM) to capture words while browsing
- **wordbin-types** — Shared types between server and clients

## What it is not

- A flashcard app (yet)
- A SaaS
- Stable

---

## Architecture

```
┌──────────────┐                           ┌──────────────┐
│  admin CLI   │   token issue (offline)   │              │
│ (server bin) │ ────────────────────────▶ │              │
└──────────────┘                           │              │
                                           │   Server     │
┌──────────────┐  POST /word/add           │   (Axum)     │ ──▶ SQLite
│  Extension   │  Authorization: Bearer    │              │
│ (Leptos/WASM)│ ────────────────────────▶ │              │
└──────────────┘                           └──────────────┘
        ▲                                         ▲
        └──── shared types: wordbin-types ────────┘
```

---

## Features

- Capture words from any browser tab — source auto-fills from hostname
- Three pages — quick capture, full word list, settings
- 4 UI languages — en / ru / de / fr
- OpenAPI docs auto-generated, Swagger UI at `/swagger-ui`
- Single binary, SQLite as the only runtime dependency

---

## Stack

- **Server** — Rust, Axum, SQLite via sqlx
- **Extension** — Rust, Leptos, WASM
- **Config** — TOML via figment
- **Docs** — Swagger UI via utoipa

---

## Running the server

### From source

```bash
git clone https://github.com/Vancoola/wordbin
cd wordbin

# .env at workspace root
echo 'DATABASE_URL=sqlite:words.db' > .env

cargo run -p wordbin-server
```

Run all `cargo` and `sqlx` commands from the workspace root — the `.env` is resolved relative to the current working directory.

### Migrations

Migrations live in `server/migrations/`. The runtime applies them on startup, so a fresh `cargo run` is enough for normal use. For ad-hoc operations a root-level `Makefile` wraps `sqlx-cli`:

```bash
make migrate                        # apply pending migrations
make migrate-add name=add_tokens    # create a new migration
make db-reset                       # wipe words.db and re-apply
```

`sqlx-cli` is a separate install: `cargo install sqlx-cli --no-default-features --features sqlite`.

### Docker

Build context is the workspace root, not `server/` — the sibling `wordbin-types` crate is needed:

```bash
git clone https://github.com/Vancoola/wordbin
cd wordbin
docker build -f server/Dockerfile -t wordbin-server .

# Pre-create the SQLite file — without this, Docker creates a directory at the
# bind path and SQLite fails with "unable to open database file".
touch words.db

docker run --rm -p 3000:3000 \
  -v "$PWD/words.db:/app/server/words.db" \
  wordbin-server
```

`words.db` lives in the current directory and survives container restarts.

### Config

Read from `server/App.toml` and **baked into the image at build time** — change the port or log level before `docker build`.

```toml
[server]
port = 3000
host = "0.0.0.0"
tracing_level = "INFO"
```

Swagger UI: `http://localhost:3000/swagger-ui`.

---

## Authentication

All `/word` endpoints require a JWT in the `Authorization: Bearer <token>` header. Tokens are issued by the server binary's admin subcommand and stored hashed in SQLite.

Two roles:

- **admin** — full access, intended for CLI / admin tools
- **user** — data access only, intended for the browser extension and other clients

### Issuing a token

```bash
# never expires
cargo run -p wordbin-server -- token issue admin --name "my-laptop"

# expires in 30 days
cargo run -p wordbin-server -- token issue user --name "chrome-extension" --ttl-days 30
```

The plaintext token is printed once on stdout — save it, it's not recoverable later (only the hash is stored).

In Docker:

```bash
docker run --rm -v "$PWD/words.db:/app/server/words.db" \
  wordbin-server token issue user --name "chrome-extension"
```

## Building the extension

Use Makefile:

```bash
cd wordbin-extension
make build
```

Or:

```bash
cd wordbin-extension
wasm-pack build --target web --out-dir dist/pkg --release
cp manifest.json index.html popup.css bootstrap.js dist/ && cp -r icons dist/
```

Then load `wordbin-extension/dist/` into Chrome:

1. Open `chrome://extensions`
2. Toggle **Developer mode** (top right)
3. Click **Load unpacked**, select the `dist/` directory

---

## Roadmap

- [x] Capture endpoint + extension popup
- [x] Settings page with i18n
- [x] Words list view
- [ ] Spaced-repetition review flow (table already in schema)
- [x] Auth — JWT (admin / user roles)
- [x] Admin CLI — token issuance
- [ ] Telegram client (bot)
- [ ] CLI client
- [x] Docker image
- [ ] Tauri client 

---

## Status

Active development since May 2026. Schema will change. Built for myself — issues and PRs welcome, no guarantees.
