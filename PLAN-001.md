
## Estado final

- Frontend: 59 → **0** alertas de Dependabot; `pnpm audit` limpio; `pnpm run lint` y `pnpm run build` verdes.
- Backend: 5 vulnerabilidades + 6 warnings de `cargo audit` → **0**; `cargo fmt`/`clippy -D warnings` verdes; **7 tests** añadidos.
- CI ampliado con `cargo audit` y `pnpm audit`.
- Issue [#8](https://github.com/atareao/tareas/issues/8) abierto para los bugs del modelo de etiquetado (código no expuesto).
- Docs: `docs/gitflow.md`, `docs/seguridad-dependencias.md`.
