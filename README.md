# Tareas

Aplicación de gestión de tareas: **backend en Rust** ([axum](https://github.com/tokio-rs/axum) + [sqlx](https://github.com/launchbadge/sqlx)/SQLite) y **frontend en React + TypeScript** (Vite + MUI). El backend sirve la API y el frontend compilado desde un único binario.

## Estructura

| Directorio | Contenido |
|---|---|
| `server/` | Backend Rust: API HTTP, migraciones SQLite (`server/migrations/`). |
| `client/` | Frontend React + TypeScript. |
| `docs/` | Documentación por feature. |
| `plans/` | Planes de trabajo (el activo es el no `-ARCHIVED`). |

## Requisitos

- Rust estable (`cargo`)
- Node >= 22.13 y **pnpm** (el frontend se instala y compila con pnpm; `npm install` ignora los `overrides` de seguridad de `client/pnpm-workspace.yaml`)
- Opcional: [`just`](https://github.com/casey/just) para los atajos

## Puesta en marcha (desarrollo)

```bash
just dev        # compila el frontend, lo copia a server/static y arranca el backend
just clientr    # solo el frontend (vite dev server)
```

El backend escucha en `http://localhost:3000`.

### Variables de entorno

| Variable | Defecto | Descripción |
|---|---|---|
| `RUST_LOG` | `info` | Nivel de log (`tracing`). |
| `DB_URL` | `tareas.db` | Ruta / URL de la base SQLite. |
| `RUST_ENV` | — | En `production` las migraciones se leen de `<dir del binario>/migrations`. |

## Verificación

```bash
# Backend
cd server && cargo fmt --all -- --check
cd server && cargo clippy --all-targets -- -D warnings
cd server && cargo test
cd server && cargo audit

# Frontend
cd client && pnpm install --frozen-lockfile
cd client && pnpm run lint
cd client && pnpm run build
cd client && pnpm audit
```

## Docker

```bash
docker build -t tareas .
docker run --rm -p 3000:3000 -v tareas-data:/app tareas
```

Imagen multi-stage (`ghcr.io/atareao/tareas`): frontend con pnpm, backend Rust estático sobre musl y runtime Alpine sin root.

## Flujo de trabajo

Proyecto con **Git Flow** estricto (`main` / `development` / `feature/*` / `hotfix/*`) y conventional commits con gitmoji:

- [GIT_FLOW.md](./GIT_FLOW.md) — ramas, commits y releases.
- [docs/gitflow.md](./docs/gitflow.md) — CI/CD, `vampus`, `git-cliff` e imagen.
- [docs/seguridad-dependencias.md](./docs/seguridad-dependencias.md) — política de dependencias y auditorías.

## Licencia

MIT — ver [LICENSE](./LICENSE).
