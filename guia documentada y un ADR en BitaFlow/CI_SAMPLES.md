# CI_SAMPLES.md · Validación y render estricto

## Validar alias y placeholders (lint rápido)
```bash
# Alias
egrep -nR "alias:\s*BITA-(TPL|DOC|ADR|SNAP|MAP)-[A-Z0-9]+(?:_[A-Z0-9]+)*(?:-[A-Z0-9]+(?:_[A-Z0-9]+)*)?-v[0-9]+(?:\+[A-Z0-9]+)*$" templates/ || exit 1

# Placeholders sin resolver (heurística sobre .md publicados)
egrep -nR "\{\{[^>][^}]+\}\}" published/ && (echo "Placeholders sin resolver" && exit 1) || true
```

## GitHub Actions (job compacto)
```yaml
- uses: dtolnay/rust-toolchain@stable
- run: cargo build --bins --manifest-path crates/bfl_translator/Cargo.toml
- run: ./crates/bfl_translator/target/debug/bfl render templates/BITA-DOC-ROUTIER-v1.bt published/routier.md --vars examples/vars.json --repo templates --strict --report published/report.json
- run: test $(jq '.errors.placeholders_unresolved|length + .errors.includes_missing|length + .errors.include_cycles|length + .errors.required_vars_missing|length' published/report.json) -eq 0
```
