# syntax=docker/dockerfile:1

# ── Frontend ────────────────────────────────────────────────────────────────
# Se construye con pnpm y el lockfile versionado (npm ignoraría pnpm-lock.yaml
# y, con él, los `overrides` de seguridad del package.json).
FROM node:22-slim AS client-builder
WORKDIR /client-builder
RUN npm install -g pnpm@12
COPY ./client/package.json ./client/pnpm-lock.yaml ./client/pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY ./client ./
RUN pnpm run build

# ── Backend ─────────────────────────────────────────────────────────────────
FROM rust:1-alpine3.21 AS server-builder
WORKDIR /server-builder
RUN apk update && apk add --no-cache musl-dev
# Cargo.lock está versionado: `--locked` garantiza un build reproducible.
COPY ./server ./
RUN cargo build --release --locked

# ── Runtime ─────────────────────────────────────────────────────────────────
FROM alpine:3.21
RUN apk add --no-cache ca-certificates tzdata \
    && addgroup -S app \
    && adduser -S -G app app
WORKDIR /app
COPY --from=server-builder /server-builder/target/release/server ./server
# En producción la app lee las migraciones de `<dir del binario>/migrations`
# (RUST_ENV=production) y sirve el frontend desde `./static`.
COPY --from=server-builder /server-builder/migrations ./migrations
COPY --from=client-builder /client-builder/dist/ ./static/
# La BD SQLite por defecto (`tareas.db`) se crea en el WORKDIR: debe ser
# escribible por el usuario sin privilegios.
RUN chown -R app:app /app
ENV RUST_LOG=info \
    RUST_ENV=production
USER app
EXPOSE 3000
ENTRYPOINT ["./server"]
