# wordbin

> One place to drop English words you don't know yet.

I encounter unknown words everywhere — subtitles, browser tabs, podcasts, books. Every tool I tried solved one source but not the others. I ended up with words scattered across a phone app, a browser extension, and a notebook that I never actually reviewed.

Wordbin is a self-hosted daemon that gives me a single endpoint to send words from anywhere. A browser extension for the web, a CLI for the terminal, whatever else later. Everything lands in one SQLite file.

---

## What it is

- **wordbin-server** — Axum daemon, exposes a REST API, stores words in SQLite
- **wordbin-extension** — Browser extension (Leptos/WASM) to capture words while browsing

## What it is not

- A flashcard app (yet)
- A SaaS
- Stable

---

## Stack

- **Server** — Rust, Axum, SQLite via sqlx
- **Extension** — Rust, Leptos, WASM
- **Config** — TOML via figment
- **Docs** — Swagger UI via utoipa

---

## Running the server

```bash
git clone https://github.com/Vancoola/wordbin
cd wordbin/wordbin-server
cargo run
```

Config is read from `App.toml` in the project root:

```toml
[server]
port = 3000
host = "0.0.0.0"
tracing_level = "INFO"
```

Swagger UI available at `http://localhost:3000/swagger-ui`.

---

## Status

Early development. Schema will change. Built for myself — issues and PRs welcome, no guarantees.

---

## License

Apache-2.0