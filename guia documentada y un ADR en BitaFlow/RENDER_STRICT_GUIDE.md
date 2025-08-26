# RENDER_STRICT_GUIDE.md · Modo estricto de `bfl render`

## 0) Objetivo
Definir cómo implementaremos un **modo estricto** para `bfl render` que:
- **Falle** si quedan `{{placeholders}}` sin resolver.
- **Falle** si hay `{{> includes}}` rotos o ciclos de includes.
- **Genere** un reporte de diagnóstico estructurado (JSON) para CI.
- Mantenga **compatibilidad** con el modo actual (no estricto).

## 1) Alcance
Aplica al comando:
```
bfl render <input.bt> [output.md] [--vars vars.json] [--repo path] [--strict] [--fail-on-missing-include] [--require-vars k1,k2,...] [--report report.json] [--draft]
```
- `--strict`: activa reglas estrictas (ver §4).
- `--fail-on-missing-include`: convierte includes faltantes en error fatal (en `--strict` ya es implícito).
- `--require-vars`: lista de claves que deben existir en `--vars` (si falta, error).
- `--report`: ruta para escribir un JSON con hallazgos (si no se pasa, solo stdout/stderr).
- `--draft`: fuerza tolerancia (lo contrario de `--strict`), útil para previsualizar.

## 2) Terminología rápida
- **Placeholder**: `{{clave}}` que se sustituye desde `--vars`.
- **Include**: `{{> ALIAS }}` que expande el cuerpo de otra plantilla por alias.
- **Repo de plantillas**: carpeta(s) donde se buscan `.bt` por alias (`--repo` y defaults).

## 3) Requisitos funcionales
1. Expandir includes recursivos con **detección de ciclos** y marcas claras.
2. Sustituir placeholders con `--vars` (JSON). Dejar intactos si no hay valor **solo** en modo *draft*.
3. Reportar **faltantes**: placeholders sin resolver, includes rotos, ciclos, variables requeridas ausentes.
4. Salidas:
   - `output.md` (si se especifica), sin front‑matter, sin comentarios `;;`.
   - `--report` JSON con diagnóstico (ideal para CI).

## 4) Semántica de `--strict`
- **Error fatal (exit code != 0) si:**
  - Queda **al menos un** placeholder sin resolver.
  - Existe **algún include** no resuelto (missing) o **ciclo** de includes.
  - Falta **alguna** variable declarada en `--require-vars`.
- **No fatal en `--strict`:**
  - Variables **extra** en `--vars` que no se usaron (se listan como warning).
  - Espacios/estilos/encoding (se normaliza EOL a `
`).

## 5) Formato de reporte (`--report`)
Ejemplo:
```json
{
  "input": "templates/BITA-DOC-ROUTIER-v1.bt",
  "strict": true,
  "errors": {
    "placeholders_unresolved": ["resumen_breve", "next_1"],
    "includes_missing": ["BITA-TPL-DOD-v1"],
    "include_cycles": ["BITA-TPL-A-v1 -> BITA-TPL-B-v1 -> BITA-TPL-A-v1"],
    "required_vars_missing": ["fecha"]
  },
  "warnings": {
    "unused_vars": ["foo","bar"]
  },
  "metrics": {
    "placeholders_total": 17,
    "placeholders_resolved": 15,
    "includes_total": 3,
    "includes_resolved": 2,
    "duration_ms": 41
  }
}
```

## 6) Algoritmo (alto nivel)
1. **Cargar** `input.bt` y parsear front‑matter + cuerpo.
2. **Repo**: construir índice alias→doc para cada `--repo` y la carpeta del `input.bt`.
3. **Expandir includes** (loop):
   - Reemplazar `{{> ALIAS }}` por cuerpo de la plantilla destino.
   - Detectar **ciclos** con un stack `seen` (si `ALIAS` ya está, registrar ciclo).
   - Repetir hasta converger o superar límite de profundidad (p.ej., 32).
4. **Sustituir placeholders** con `--vars`:
   - Regex `{{\s*([a-zA-Z0-9_]+)\s*}}` (excluir includes).
   - Llevar conteo de resueltos / no resueltos.
5. **Validaciones strict**:
   - Si `--strict` y hay errores → exit code 1.
   - Si `--require-vars` y alguna falta → error.
6. **Reporte**: si `--report` → escribir JSON (ver §5).
7. **Emitir** Markdown final (sin `;;` comentarios, normalizar EOL).

## 7) Códigos de salida
- `0`: OK (o warnings no fatales).
- `1`: Error estricto (placeholders/includes/requeridos).
- `2`: Error de E/S (no se pudo leer/escribir, ruta inválida).
- `3`: Error de parseo (front‑matter YAML inválido).

## 8) Ejemplos de uso
```bash
# 1) Render estricto: falla si falta algo
bfl render routier.bt out.md --vars vars.json --repo templates --strict --report render_report.json

# 2) Draft: genera output aunque falten placeholders (no fatal)
bfl render routier.bt out.md --vars vars.json --repo templates --draft

# 3) Requerir claves críticas
bfl render routier.bt out.md --vars vars.json --require-vars fecha,topic --strict
```

## 9) Integración CI (GitHub Actions)
```yaml
name: Render BitaFlow
on: [push, pull_request]
jobs:
  render:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --bins --manifest-path crates/bfl_translator/Cargo.toml
      - run: |
          ./crates/bfl_translator/target/debug/bfl render templates/BITA-DOC-ROUTIER-v1.bt             published/routier.md             --vars examples/vars.json             --repo templates             --strict             --report published/render_report.json
      - name: Fail if render strict errors
        run: |
          jq '.errors | length' published/render_report.json
          test $(jq '.errors.placeholders_unresolved|length + .errors.includes_missing|length + .errors.include_cycles|length + .errors.required_vars_missing|length' published/render_report.json) -eq 0
```

## 10) Pruebas (matriz mínima)
| Caso | Descripción | Esperado |
|---|---|---|
| P1 | Sin placeholders, sin includes | OK |
| P2 | Placeholder resuelto | OK |
| P3 | Placeholder sin resolver (`--strict`) | Error |
| P4 | Include válido | OK |
| P5 | Include faltante (`--strict`) | Error |
| P6 | Ciclo de includes | Error |
| P7 | `--require-vars` faltante | Error |
| P8 | `--draft` con faltantes | OK con warnings |

## 11) Compatibilidad y rollout
- **Default**: se mantiene comportamiento laxo (sin `--strict`).
- Documentar flags en `README.md` y en `--help` del CLI.
- Añadir tests E2E en `tests/` para cada caso de la matriz.

## 12) Seguridad & robustez
- Aislar lectura del repo para evitar **path traversal**.
- Límite de profundidad de includes.
- Normalizar encoding (UTF‑8) y EOL (`\n`).
- No ejecutar código; esto es plantilla **no Turing‑complete**.

## 13) Roadmap
- Validación en tiempo real (extensión VSCode).
- `bfl lint` dedicado (sin render) para correr rápido en CI.
- Archivo de config `bflrc.json` en raíz del repo.

---

### Apéndice A: Esquema de `bflrc.json`
```json
{
  "strict": true,
  "repo": ["templates", ".bitacora/bitaflow"],
  "require_vars": ["fecha", "topic"],
  "fail_on_missing_include": true
}
```

### Apéndice B: Mensajes de error (estándar)
- `ERR_PLACEHOLDER_UNRESOLVED:<clave>`
- `ERR_INCLUDE_MISSING:<alias>`
- `ERR_INCLUDE_CYCLE:<chain>`
- `ERR_REQUIRED_VAR_MISSING:<clave>`
- `ERR_PARSE_FRONT_MATTER`
- `ERR_IO:<path>`
