# AGENTS.md — tareas

Aplicación de gestión de tareas: backend en **Rust (axum + sqlx/SQLite)** en
`server/` y frontend en **React + TypeScript (Vite + MUI)** en `client/`.

## Git Flow

Este proyecto sigue **Git Flow** estricto. Ver [GIT_FLOW.md](./GIT_FLOW.md):

- Ramas: `main` (producción), `development` (integración), `feature/*` (desde
  `development`) y `hotfix/*` (desde `main`).
- Nunca hacer commits directos a `main` ni `development`: todo entra por PR.
- Conventional commits con gitmoji (`✨ feat:`, `🐛 fix:`, `👷 ci:`, `⬆️ deps:`, …).

## Comandos

| Acción | Comando |
|---|---|
| Levantar todo en dev | `just dev` |
| Sólo frontend en dev | `just clientr` |
| Backend (build) | `just buildr` |
| Tests backend | `cd server && cargo test` |
| Formato backend | `cd server && cargo fmt --all` |
| Lint backend | `cd server && cargo clippy --all-targets -- -D warnings` |
| Lint frontend | `cd client && pnpm run lint` |
| Build frontend | `cd client && pnpm run build` |
| Auditar vulnerabilidades | `cd server && cargo audit` · `cd client && pnpm audit` |

## Convenciones

- Backend: tipos en `server/src/models/`, handlers en `server/src/http/`.
- Frontend: componentes en `client/src/components/`, modelos de API en
  `client/src/models/`.
- `server/Cargo.lock` **se versiona** (es una aplicación, no una librería).
- El frontend **se instala y se construye con `pnpm`**; `npm install` ignora los
  `overrides` de seguridad de `client/pnpm-workspace.yaml`.
- `server/vendor/sqlx-sqlite` es un parche temporal de seguridad (ver
  `docs/seguridad-dependencias.md`); no editar sin leer su README.
- CI: ver [docs/gitflow.md](./docs/gitflow.md) y
  [docs/seguridad-dependencias.md](./docs/seguridad-dependencias.md).
