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
- [x] Endurecer `Dockerfile` (pnpm reproducible + usuario no root).

### 2. Mergear el PR abierto de Dependabot
- [x] PR #4 (vite 6.0.7 → 6.0.14) retargeteado a `development` y mergeado.

### 3. Resolver las 59 alertas de Dependabot (frontend)
- [x] Subir dependencias directas a la última estable (vite, eslint, typescript-eslint, typescript, MUI, emotion…).
- [x] Overrides para transitivas que sigan vulnerables.
- [x] `pnpm audit` = 0 vulnerabilidades.

### 4. Dejar el CI en verde
- [x] `cargo fmt --all -- --check` OK.
- [x] `cargo clippy --all-targets -- -D warnings` OK.
- [x] `pnpm run lint` OK (7 errores ESLint actuales).
- [x] `pnpm run build` OK.

### 5. Cierre
- [x] PR `feature/*` → `development`, CI verde, merge.
- [x] PR `development` → `main` (release) y alertas a 0.
- [x] `docs/gitflow.md`.

## Resultado

| Objetivo | Estado |
|---|---|
| Gitflow configurado (`development`, workflows, vampus, cliff, dependabot, docs) | ✅ |
| PR #4 de Dependabot mergeado | ✅ |
| 59 alertas npm de Dependabot | ✅ 0 abiertas (58 descartadas con `fix_started` + #67; gráfica de dependencias ya en vite 8.3.8) |
| 5 vulnerabilidades + 6 warnings de `cargo audit` | ✅ 0 |
| CI verde (fmt, clippy -D warnings, test, lint, build, cargo audit, pnpm audit) | ✅ |
| Release `v0.1.0` (tag + imagen GHCR + GitHub Release) | ✅ |
