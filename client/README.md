# client

Frontend de **Tareas**: React 18 + TypeScript + Vite + MUI.

**Gestor de paquetes: `pnpm`.** Instala y construye siempre con pnpm
(`pnpm install --frozen-lockfile`, `pnpm run build`); `npm install` **no** aplica
los `overrides` de seguridad declarados en `pnpm-workspace.yaml`.

| Script | Qué hace |
|---|---|
| `pnpm run dev` | Servidor de desarrollo (Vite). |
| `pnpm run build` | Type check (`tsc -b`) + build de producción. |
| `pnpm run lint` | ESLint. |
| `pnpm run preview` | Sirve el build de producción. |
| `pnpm audit` | Auditoría de vulnerabilidades (debe estar limpia). |

El backend sirve el contenido de `dist/` (copiado a `server/static/`). Ver el
[README raíz](../README.md) para la puesta en marcha completa.
