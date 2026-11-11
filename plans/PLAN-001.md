# PLAN-001 — Bootstrap gitflow + resolución de vulnerabilidades

- **Issue:** [#5](https://github.com/atareao/tareas/issues/5)
- **Rama de integración:** `development`
- **Referencia de convenciones:** repo `atareao/shuul` / `atareao/presu`

## Objetivos

### 1. Configurar gitflow «como habitual»
- [x] Crear rama `development` y publicarla.
- [x] `.vampus.yml` (gestión de versión sobre `server/Cargo.toml`).
- [x] `cliff.toml` (changelog con gitmoji / conventional commits).
- [x] `.github/workflows/ci.yml` (Rust: fmt + clippy -D warnings + test; React: pnpm lint + build).
- [x] `.github/workflows/release-prepare.yml` (bump + changelog + tag + sync de `development`).
- [x] `.github/workflows/release.yml` (imagen GHCR + GitHub Release).
- [x] `.github/dependabot.yml` (cargo `/server` + npm `/client`, target `development`).
- [x] `GIT_FLOW.md` y `AGENTS.md` del proyecto.
- [x] Versionar `server/Cargo.lock` (la imagen se construye con `--locked`).
- [x] `.dockerignore`.
- [ ] Endurecer `Dockerfile` (pnpm reproducible + usuario no root).

### 2. Mergear el PR abierto de Dependabot
- [x] PR #4 (vite 6.0.7 → 6.0.14) retargeteado a `development` y mergeado.

### 3. Resolver las 59 alertas de Dependabot (frontend)
- [ ] Subir dependencias directas a la última estable (vite, eslint, typescript-eslint, typescript, MUI, emotion…).
- [ ] `pnpm.overrides` para transitivas que sigan vulnerables.
- [ ] `pnpm audit` = 0 vulnerabilidades.

### 4. Dejar el CI en verde
- [ ] `cargo fmt --all -- --check` OK.
- [ ] `cargo clippy --all-targets -- -D warnings` OK.
- [ ] `pnpm run lint` OK (7 errores ESLint actuales).
- [ ] `pnpm run build` OK.

### 5. Cierre
- [ ] PR `feature/*` → `development`, CI verde, merge.
- [ ] PR `development` → `main` (release) y comprobación de que las alertas pasan a 0.
- [ ] `docs/gitflow.md`.
