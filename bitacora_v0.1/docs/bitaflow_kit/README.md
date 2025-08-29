# BitaFlow (bfl) · DSL para Bitácora

**BitaFlow** es un lenguaje (DSL) para describir, versionar y componer **plantillas** y **flujos** de Bitácora.
- Legible para humanos (Markdown + directivas)
- Preciso para la AI (alias tipados, placeholders, includes)
- Validable por CI (regex, reglas)

## Componentes de este kit
- `docs/SPEC_ALIASES.md` — Especificación de alias (formato, regex, ejemplos).
- `docs/BitaFlow_LANGUAGE.md` — Gramática del lenguaje: archivos `.bt`/`.bfl`.
- `docs/TRANSLATION_SPEC.md` — Reglas para traducir `.md` ⇄ `.bt`.
- `docs/USER_GUIDE.md` — Guía de entrevista y análisis de viabilidad.
- `crates/bfl_translator/` — Crate Rust con parser/serializer y traductores.
- `vscode-extension/` — Extensión mínima para VSCode (coloreado y snippets).
- `examples/` — Ejemplos `.bt` y `.md`.
