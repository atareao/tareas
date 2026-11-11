# vendor/sqlx-sqlite

Copia local (vendor) de [`sqlx-sqlite` 0.9.0](https://crates.io/crates/sqlx-sqlite/0.9.0)
publicada en crates.io, con **un único cambio** en `Cargo.toml`:

```diff
 [dependencies.libsqlite3-sys]
-version = ">=0.30.1, <0.38.0"
+version = ">=0.39.0, <0.41.0"
```

## Por qué

`sqlx-sqlite` 0.9.0 (la última publicada) fija `libsqlite3-sys` en el rango
`>=0.30.1, <0.38.0`. Todas las versiones de `libsqlite3-sys` en ese rango están
afectadas por [RUSTSEC-2026-0459](https://rustsec.org/advisories/RUSTSEC-2026-0459)
(/CVE-2026-119375): el SQLite embebido (feature `bundled`) anterior a 3.54.0
puede devolver memoria no inicializada de la page-cache al abrir una base de
datos junto a un fichero `-wal` malicioso. La versión parcheada es
`libsqlite3-sys >= 0.39.0` (SQLite 3.54.0+).

El límite superior del rango lo pone `sqlx-sqlite`, así que **ninguna** versión
publicada de `sqlx` (0.8.x / 0.9.0) puede arrastrar la versión parcheada. Se
amplía el rango localmente y se inyecta con `[patch.crates-io]` desde
`server/Cargo.toml`.

## Cuándo eliminar esta carpeta

En cuanto exista una versión publicada de `sqlx-sqlite` cuyo requisito admita
`libsqlite3-sys >= 0.39` (lo habitual: cuando `sqlx` suba a `rusqlite 0.41` /
`libsqlite3-sys 0.39+`):

1. Borrar `server/vendor/sqlx-sqlite/`.
2. Quitar el bloque `[patch.crates-io]` de `server/Cargo.toml`.
3. `cargo update && cargo build`.

## Comprobación

```bash
cd server
cargo tree -i libsqlite3-sys   # debe mostrar >= 0.39.0
cargo audit                    # 0 vulnerabilidades
```
