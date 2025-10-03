ARG RUN_ID

# ---------- Stage 1: Build Rust ----------
FROM rust:1.90-bullseye AS builder
ARG RUN_ID

# Install SQLite headers and OpenSSL dev for sqlx with TLS
RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config libssl-dev

WORKDIR /app

# Copy actual source and rebuild
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
ARG RUN_ID

WORKDIR /app

# Install runtime dependencies: SQLite, certs, and OpenSSL 1.1
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates libssl1.1 && apt-get clean

# Copy compiled binary and assets
COPY --from=builder /app/target/release/crust ./app
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/public ./public

# Rename files in public/styles and public/scripts
RUN for dir in public/styles public/scripts; do \
    if [ -d "$dir" ]; then \
    for file in "$dir"/*; do \
    [ -f "$file" ] || continue; \
    filename=$(basename -- "$file"); \
    name="${filename%.*}"; \
    ext="${filename##*.}"; \
    cp "$file" "$dir/$name.$RUN_ID.$ext"; \
    done; \
    fi; \
    done

CMD ["./app"]