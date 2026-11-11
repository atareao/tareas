# Gitflow y CI/CD

## Qué es

Configuración de flujo de trabajo del repositorio `tareas` según el estándar
habitual de los proyectos de `atareao`: Git Flow con conventional commits +
gitmoji, versionado semántico automático (vampus), changelog (git-cliff) y
workflows de GitHub Actions para CI, preparación de release y publicación de la
imagen en GHCR.

## Ficheros implicados

| Fichero | Función |
|---|---|
| `.vampus.yml` | Versión actual y ficheros donde se propaga (`server/Cargo.toml`). |
| `cliff.toml` | Configuración de git-cliff (grupos por tipo, sin emojis). |
| `GIT_FLOW.md` | Documentación de ramas y flujo diario. |
| `AGENTS.md` | Instrucciones del proyecto (comandos, convenciones). |
| `.github/workflows/ci.yml` | PR/push a `main`/`development`: Rust (fmt, clippy `-D warnings`, test) + React (pnpm lint, build). |
| `.github/workflows/release-prepare.yml` | push a `main`: bump vampus, changelog, tag `vX.Y.Z`, sync de `development`. |
| `.github/workflows/release.yml` | tag `v*`: imagen a `ghcr.io/atareao/tareas` + GitHub Release. |
| `.github/dependabot.yml` | Actualizaciones semanales de cargo (`/server`) y npm (`/client`) hacia `development`. |
| `Dockerfile` | Build multi-stage: pnpm (frontend) + cargo musl (backend) + runtime alpine sin root. |
| `.dockerignore` | Excluye `node_modules`, `dist`, `target`, BD SQLite del contexto de build. |

## Cómo funciona

### Ramas
`main` (producción, dispara release) · `development` (integración) ·
`feature/*` (desde `development`) · `hotfix/*` (desde `main`).

### Release
1. PR `development` → `main`.
2. `release-prepare.yml` decide por comparación de versión/tag:
   - `skip`: `v$CURRENT` existe y HEAD es ese tag.
   - `bump`: `v$CURRENT` existe pero HEAD tiene commits nuevos → `vampus upgrade`.
   - `tag`: `v$CURRENT` no está etiquetada → sólo changelog + tag.
3. Tras el bump, sincroniza `server/Cargo.lock` (`cargo update --package server`)
   para que el tag siga compilando con `--locked`.
4. Se crea el tag → `release.yml` construye la imagen y la GitHub Release.
5. `development` se sincroniza con `main`.

### Imagen
- Frontend construido con `pnpm install --frozen-lockfile` (respeta `overrides`).
- Backend `cargo build --release --locked` sobre musl (binario estático).
- Runtime: `alpine:3.21`, usuario `app` sin privilegios, `RUST_ENV=production`
  (la app lee `migrations/` junto al binario) y `static/` con el frontend.
- Variables: `RUST_LOG` (def. `info`), `RUST_ENV=production`, `DB_URL`
  (def. `tareas.db`, relativo al WORKDIR `/app`).

## Configuración necesaria en GitHub

| Secreto | Uso | ¿Obligatorio? |
|---|---|---|
| `GH_PAT` | Push del tag con un token que dispare `release.yml` y sincronizar `development`. | Recomendado. Sin él se usa `GITHUB_TOKEN`; la release se etiqueta igual pero la imagen hay que relanzarla a mano. |

Permisos: en *Settings → Actions → General*, permitir que los workflows tengan
`Read and write permissions`.

## Decisiones

- Se versiona `server/Cargo.lock` (es una aplicación; la imagen usa `--locked`).
- `target-branch: development` en Dependabot: en este repo `main` es de release.
- Build frontend con pnpm en vez de npm para que los `overrides` de seguridad
  lleguen también a la imagen.
- `RUST_ENV=production` + copia de `migrations/` al runtime: sin ellos la app
  hace `unwrap()` sobre `CARGO_MANIFEST_DIR` (inexistente en runtime) y no arranca.
- `alpine:3.28` del Dockerfile original no existe (máx. 3.24): corregido a `3.21`.

## Follow-ups

- Añadir tests (Vitest en `client`, tests de integración en `server`) y
  `pnpm run test` al job de frontend del CI.
- Añadir `pnpm audit --audit-level=high` como paso del CI.

## Merge del PR de release

El PR `development -> main` debe mergearse con **merge commit**
(`gh pr merge <n> --merge`), **no** con squash. `release-prepare.yml` genera el
CHANGELOG con `git-cliff` sobre la historia de `main`: si el release PR se
squashea, los commits individuales no llegan a `main` y la release queda sin
entradas (solo `Miscellaneous Tasks - Release vX`). Con merge commit la historia
se conserva y el CHANGELOG sale completo, como en `atareao/shuul`.
