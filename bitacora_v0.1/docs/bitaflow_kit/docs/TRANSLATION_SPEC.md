# TRANSLATION_SPEC.md · Traducción `.md` ⇄ `.bt`

## `.md` → `.bt`
- Detectar metadatos → front‑matter
- Mapear secciones conocidas a placeholders/includes
- Conservar diagramas ASCII

## `.bt` → `.md`
- Resolver placeholders
- Expandir includes
- Emitir Markdown limpio

**Firmas (Rust):**
```rust
pub fn md_to_bt(md: &str, target_alias: &str) -> Result<String, Error>;
pub fn bt_to_md(bt: &str, ctx: Resolver) -> Result<String, Error>;
```
