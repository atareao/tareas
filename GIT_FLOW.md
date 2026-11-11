# Git Flow

Este proyecto sigue **Git Flow** con versionado semántico automático.

## Ramas

| Rama | Propósito | Base |
|---|---|---|
| `main` | Producción. Cada merge aquí dispara una release automática (version, changelog, tag e imagen). | — |
| `development` | Integración de features en curso. | `main` |
| `feature/*` | Nuevas funcionalidades. | `development` |
| `hotfix/*` | Correcciones urgentes a producción. | `main` |

## Flujo diario

### Features

```bash
git checkout development && git pull origin development
git checkout -b feature/mi-feature
git commit -m "✨ feat: add dark mode toggle"
git push origin feature/mi-feature
# Crear Pull Request a development
```

### Hotfixes

```bash
git checkout main && git pull origin main
git checkout -b hotfix/arreglo-critico
git commit -m "🚑️ hotfix: crash on empty input"
git push origin hotfix/arreglo-critico
# Crear Pull Request a main
# Después del merge, sincronizar development (lo hace release-prepare.yml)
```

### Releases

```bash
# PR: development → main
# Al mergear, CI automáticamente:
#   a) Detecta el bump type según los conventional commits
#   b) vampus actualiza la versión en server/Cargo.toml (y .vampus.yml)
#   c) git-cliff genera CHANGELOG.md
#   d) Crea el tag vX.Y.Z
#   e) release.yml compila la imagen y crea la GitHub Release
#   f) Sincroniza development con main
```

## Conventional Commits con Gitmoji

El formato determina el bump automático:

| Mensaje | Bump |
|---|---|
| `✨ feat: ...` | minor (0.Y.0) |
| `🐛 fix: ...` | patch (0.0.Z) |
| `♻️ refactor: ...` | patch (0.0.Z) |
| `📝 docs: ...` | patch (0.0.Z) |
| `💥 feat!: ...` o `BREAKING CHANGE` | major (X.0.0) |

### Gitmoji de referencia

| Tipo | Emoji |
|---|---|
| feat | ✨ |
| fix | 🐛 |
| hotfix | 🚑️ |
| docs | 📝 |
| refactor | ♻️ |
| perf | ⚡ |
| style | 💄 |
| test | ✅ |
| chore | 🔧 |
| ci | 👷 |
| revert | ⏪️ |
| deps | ⬆️ |
| breaking | 💥 |

## CI Workflows

- **`ci.yml`** — PR/push a main y development: formato Rust, lint, clippy, tests, lint y build del frontend.
- **`release-prepare.yml`** — push a main: bump de versión, changelog, tag, sincroniza development.
- **`release.yml`** — tag `v*`: construye y publica la imagen en GHCR y crea la GitHub Release.

## Secretos de GitHub necesarios

| Secreto | Propósito |
|---|---|
| `GH_PAT` | Personal Access Token con scope `repo`/`contents: write` para que el push del tag dispare `release.yml` y para sincronizar `development`. Opcional: sin él se usa `GITHUB_TOKEN` y la imagen hay que relanzarla a mano. |

## Resumen visual

```
main        ──hotfix──●────────────●────────────────●
                       \            /                /
development ──────────●──●──●────●──●──●──●────────●
                        \ /        \   /
feature     ──────────●  feature ──●
                      feature/foo   feature/bar
```
