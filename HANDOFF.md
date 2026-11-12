# HANDOFF

## Estado

Repo `atareao/tareas` con **gitflow configurado y CI verde**, sin vulnerabilidades
conocidas. `main` y `development` sincronizados.

- Última release: **v0.1.1** (tag + GitHub Release + imagen GHCR).
- Rama de integración: `development` · Producción: `main`.
- Workflows: `ci.yml` (fmt/clippy/test + lint/build + cargo audit/pnpm audit),
  `release-prepare.yml`, `release.yml` (imagen GHCR + GitHub Release).
- Alertas Dependabot: **0 abiertas**. `cargo audit` y `pnpm audit` limpios.
- Sin PRs abiertos.

## Convención importante: merge del PR de release

El PR `development → main` debe mergearse con **merge commit** (`gh pr merge <n> --merge`),
**no** con `--squash`. Con squash, los commits individuales no llegan a `main` y
`git-cliff` no genera entradas: el CHANGELOG de la release sale vacío (sólo
`Miscellaneous Tasks - Release vX`). Es el motivo de que el CHANGELOG de v0.1.0 y
v0.1.1 esté escaso (se squash-mergearon); v0.1.2 en adelante debería ser correcto.
Referencia: `atareao/shuul` conserva los commits en `main` (CHANGELOG rico).

## Tarea pendiente / siguiente paso

1. **Secreto `GH_PAT`: configurado** (scopes `repo` + `workflow`; valor en crypta
   como `gh_pat`). Ya no hace falta relanzar `release.yml` a mano: el push del tag
   dispara la imagen automáticamente (v0.1.0 y v0.1.1 se publicaron a mano *antes*
   de añadir el secreto; la próxima release lo comprobará de extremo a extremo).
2. Issue **#8**: SQL roto en el modelo de etiquetado (`Tag`/`TaskTag`, código no
   expuesto en la API).
3. Follow-up: eliminar `server/vendor/sqlx-sqlite` cuando sqlx amplíe el rango de
   `libsqlite3-sys` (ver `docs/seguridad-dependencias.md`).
4. Considerar actualizar `README.md`/`docs` cuando se estabilicen React 19 y
   TypeScript 6 (recién subidos; CI verde, sin cambios funcionales).

## Referencias

- `docs/gitflow.md`, `docs/seguridad-dependencias.md`
- `plans/PLAN-001-ARCHIVED.md`
