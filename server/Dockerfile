FROM rust:1.90-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY wordbin-types ./wordbin-types
COPY server        ./server

WORKDIR /app/server

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/server/target \
    cargo build --release \
    && cp target/release/wordbin-server /usr/local/bin/wordbin-server

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --uid 10001 app


WORKDIR /app/server

COPY --from=builder /usr/local/bin/wordbin-server /usr/local/bin/wordbin-server
COPY server/App.toml ./App.toml

RUN chown -R app:app /app/server

USER app
EXPOSE 3000

CMD ["wordbin-server"]