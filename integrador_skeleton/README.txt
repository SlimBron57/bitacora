integrador — Esqueleto de proyecto
=================================

Qué hace
--------
Valida dos tokens hexadecimales en un documento (la representación hexadecimal de `Bita` que es `42697461` y la secuencia hexadecimal de `cora` que es `636f7261`).
Si ambas coinciden con las referencias en `programa.meta.txt`, ejecuta la línea definida en `programa.run.txt`.

Estructura
----------
integrador/
├─ Cargo.toml
├─ src/main.rs
├─ programa.meta.txt
├─ programa.run.txt
├─ programa.logic.txt         (programa en texto para el segundo engranaje)
├─ in/
│  └─ doc_usuario.txt
└─ out/

Uso rápido
----------
cargo build --release
./target/release/integrador ./in/doc_usuario.txt

Archivos clave
--------------
- programa.meta.txt
  PALABRA1=42697461
  PALABRA2_ENC=636f7261

- programa.run.txt
  ./engranaje_segundo --input ./in --output ./out --base 16 --logic ./programa.logic.txt

- programa.logic.txt
  # DSL simple para el segundo engranaje (interpretado por engranaje_segundo)
  INPUT_DIR=./in
  OUTPUT_DIR=./out
  BASE=16
  MODE=per_pixel    # per_pixel | palette | histogram
  FORMAT=json       # json | csv | bin
  CHANNELS=RGBA     # RGBA | RGB
  ALPHA=keep        # keep | drop
  # El segundo engranaje debe leer esta lógica y realizar la conversión correspondiente.
