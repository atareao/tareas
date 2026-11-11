# HANDOFF

## Estado

Repo `atareao/tareas` con **gitflow configurado y CI verde**, sin vulnerabilidades
conocidas. Última release: **v0.1.0** (`main` = `development`, sincronizados).

- Rama de integración: `development` · Producción: `main`.
- Workflows: `ci.yml` (fmt/clippy/test + lint/build + cargo audit/pnpm audit),
  `release-prepare.yml`, `release.yml` (imagen GHCR + GitHub Release).
- Alertas Dependabot: **0 abiertas** (59 resueltas; las 58+1 se descartaron con
  `fix_started` porque Dependabot aún no había recalculado — la gráfica de
  dependencias ya resuelve las versiones parcheadas).
- `cargo audit` y `pnpm audit` limpios.

## Tarea pendiente / siguiente paso

1. **Añadir el secreto `GH_PAT`** (Settings → Secrets → Actions) para que el push
   del tag dispare `release.yml` automáticamente. Sin él, `release-prepare.yml`
   usa `GITHUB_TOKEN`, la release se etiqueta igual pero la imagen hay que
   relanzarla a mano (`gh workflow run release.yml --ref vX.Y.Z`). El `v0.1.0`
   se publicó así.
2. PRs de Dependabot abiertos: **#11** (react/react-dom/@types/react) y **#12**
   (typescript 5.9.3 → 6.0.3). Revisar y mergear si el CI pasa.
3. Issue **#8**: SQL roto en el modelo de etiquetado (`Tag`/`TaskTag`, código no
   expuesto en la API).
4. Follow-up: eliminar `server/vendor/sqlx-sqlite` cuando sqlx amplíe el rango de
   `libsqlite3-sys` (ver `docs/seguridad-dependencias.md`).

## Referencias

- `docs/gitflow.md`, `docs/seguridad-dependencias.md`
- `plans/PLAN-001-ARCHIVED.md`
