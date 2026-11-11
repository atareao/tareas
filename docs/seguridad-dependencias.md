# Seguridad de dependencias

## Qué es

Puesta a cero de las vulnerabilidades conocidas del proyecto, tanto en el
frontend (npm/pnpm) como en el backend (Rust), y su vigilancia continua en CI.

Punto de partida: **59 alertas de Dependabot** (32 high, 22 moderate, 5 low),
todas del frontend, más **5 vulnerabilidades y 6 warnings** detectados por
`cargo audit` en el backend (invisibles para Dependabot porque
`server/Cargo.lock` no estaba versionado).

Resultado: **0 alertas** y **`pnpm audit` / `cargo audit` limpios**.

## Frontend (`client/`)

Subida de dependencias directas a la última estable:

| Paquete | Antes | Ahora |
|---|---|---|
| vite | 6.0.7 | 8.3.x |
| @vitejs/plugin-react | 4.3.4 | 6.x |
| eslint | 9.17.0 | 10.x |
| @eslint/js | 9.17.0 | 10.x |
| typescript-eslint | 8.18.2 | 8.72.x |
| typescript | 5.6.2 | 5.9.x |
| eslint-plugin-react-hooks | 5.0.0 | 7.x |
| eslint-plugin-react-refresh | 0.4.16 | 0.5.x |
| globals | 15.14.0 | 17.x |
| @mui/material + icons | 6.3.1 | 7.x |
| @emotion/styled | 11.14.0 | 11.14.1 |

vite 8 elimina las cadenas de `esbuild`/`rollup` vulnerables (usa rolldown).
React se mantiene en 18 (sin alertas).

### Overrides

Las transitivas que seguían afectadas se fijan en **`client/pnpm-workspace.yaml`**
(`@babel/core`, `@babel/helpers`, `@babel/runtime`, `browserslist`, `minimatch`,
`yaml`). ⚠️ **pnpm 12 ya no lee el campo `pnpm` de `package.json`**: los
`overrides` viven ahora en `pnpm-workspace.yaml`. Por eso hay que instalar con
**pnpm**; `npm install` los ignora.

## Backend (`server/`)

| Advisory | Crate | Antes | Ahora |
|---|---|---|---|
| RUSTSEC-2026-0007 | bytes | 1.9.0 | 1.12.x |
| RUSTSEC-2026-0459 | libsqlite3-sys | 0.30.1 | 0.40.0 |
| RUSTSEC-2023-0071 | rsa | 0.9.7 | eliminado |
| RUSTSEC-2026-0009 | time | 0.3.37 | 0.3.55 |
| RUSTSEC-2025-0055 | tracing-subscriber | 0.3.19 | 0.3.23 |

Subida de directas: `sqlx` 0.8 → **0.9**, `axum` 0.8.x, `tokio` 1.54, `tower-http`
0.6 → **0.7**, `tracing-subscriber` 0.3.23, `chrono`/`serde` al día. `sqlx` 0.9
cambió a un trait `SqlSafeStr` que prohíbe SQL dinámico con `&String`: las dos
consultas con `IN (...)` pasaron a `QueryBuilder` + `push_bind`.

### Vendor: `server/vendor/sqlx-sqlite`

`sqlx-sqlite` 0.9.0 (la última publicada) fija `libsqlite3-sys` en
`>=0.30.1, <0.38.0`, rango **enteramente afectado** por RUSTSEC-2026-0459 (CVE de
SQLite embebido < 3.54.0). Como ninguna versión publicada de sqlx puede
arrastrar `libsqlite3-sys >= 0.39`, se versiona una **copia local de
`sqlx-sqlite` 0.9.0 con una sola línea cambiada** en su `Cargo.toml`, enganchada
con `[patch.crates-io]`. Ver `server/vendor/sqlx-sqlite/README.md`.

**Follow-up:** borrar el vendor en cuanto sqlx publique una versión que amplíe el
rango (o migrar a `rusqlite`).

## Vigilancia en CI

`.github/workflows/ci.yml` ejecuta ahora `cargo audit` (job backend) y
`pnpm audit` (job frontend): una regresión de seguridad rompe el PR.

## Ficheros implicados

`client/package.json`, `client/pnpm-lock.yaml`, `client/pnpm-workspace.yaml`,
`server/Cargo.toml`, `server/Cargo.lock`, `server/vendor/sqlx-sqlite/**`,
`server/src/models/task.rs`, `.github/workflows/ci.yml`.

## Decisiones

- Subir vite 6 → 8 en vez de parchear la 6: resuelve de golpe 15 alertas y
  elimina esbuild/rollup.
- MUI 6 → 7 (no 9): la 9 rompe `PaperProps` de `Dialog`, imports profundos de
  iconos y los system props de `Box`, sin tocar el código actual.
- TypeScript 5.x (no 7): `typescript-eslint` 8.72 aún no admite TS 7.
- Vendorizar `sqlx-sqlite` en lugar de ignorar el advisory: la CVE es alcanzable
  (la app usa SQLite embebido).

## Follow-ups

- Eliminar `server/vendor/` cuando sqlx corrija el rango de `libsqlite3-sys`.
- Tests de integración HTTP en el backend (hoy sólo hay tests de modelos).
- Reglas del modelo de etiquetado: ver issue #8.
