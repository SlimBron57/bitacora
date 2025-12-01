# 🎻 BSTRADIVARIUS - MANUAL SIMPLE

> **Para humanos normales** (no programadores)

---

## ¿Qué es BStradivarius?

**Imagina que tienes 200 documentos** en tu proyecto. Cuando buscas algo, tienes que:
1. Abrir archivos uno por uno
2. Ctrl+F en cada uno
3. Recordar dónde viste algo hace 2 meses

**BStradivarius es tu bibliotecario robot** que:
- Lee TODOS tus documentos
- Recuerda dónde está cada concepto
- Te permite buscar INSTANTÁNEAMENTE

---

## Comandos (los únicos que necesitas)

### 1. Indexar tus documentos (hacer que BStradivarius los lea)

```bash
./target/release/bstradivarius sync
```

**¿Qué hace?**
- Lee todos los .md en ROADMAP_V2/
- Extrae títulos, referencias, conceptos importantes
- Los guarda en su "cerebro" (VoxelDB)
- Tarda ~1 segundo para 200 archivos

**¿Cuándo usarlo?**
- Después de crear/modificar documentos
- Una vez al día si trabajas mucho

---

### 2. Buscar algo (el comando más útil)

```bash
./target/release/bstradivarius query "arquitectura"
```

**¿Qué hace?**
- Busca "arquitectura" en TODOS los documentos
- Te muestra: archivo, línea, contexto
- Tarda <1 segundo (instantáneo)

**Ejemplos:**
```bash
# Buscar VoxelDB
./target/release/bstradivarius query "VoxelDB"
→ Resultado: 93 lugares donde se menciona

# Buscar ShuiDao
./target/release/bstradivarius query "ShuiDao"
→ Resultado: 60 lugares

# Buscar decisión arquitectónica
./target/release/bstradivarius query "DA-033"
→ Resultado: Documentos que mencionan DA-033
```

---

### 3. Ver un mapa de TODO (opcional)

```bash
cat BITACORA_KNOWLEDGE_GRAPH/INDEX.md
```

**¿Qué es?**
- Un archivo con TODOS los conceptos organizados
- Se genera automáticamente después de `sync`
- 6,000+ conceptos de 141 archivos

**Ejemplo:**
```markdown
### 📄 arquitectura-sistema.md
- Arquitectura 7 capas
- TelescopeDB
- VoxelDB
- CTX7D

### 📄 shuidao-cognitive-engine.md
- ShuiDao Phase 3b
- Intention Detection
- FlowPacks
```

---

### 4. Monitoreo automático (avanzado)

```bash
./target/release/bstradivarius watch
```

**¿Qué hace?**
- Vigila cambios en tus documentos
- Auto-indexa cuando editas algo
- Corre en background (Ctrl+C para parar)

**¿Cuándo usarlo?**
- Si trabajas todo el día en documentos
- Mantiene el índice siempre actualizado

---

## Flujo de trabajo típico

### Escenario 1: Trabajas en un documento nuevo

```bash
# 1. Creas: ROADMAP_V2/nuevo_componente.md
# 2. Escribes contenido (títulos, conceptos)
# 3. Indexas:
./target/release/bstradivarius sync

# 4. Ahora puedes buscar lo que acabas de escribir:
./target/release/bstradivarius query "nuevo_componente"
```

---

### Escenario 2: No recuerdas dónde explicaste algo

```bash
# ¿Dónde hablé de "intención"?
./target/release/bstradivarius query "intención"

# Resultado:
# 1. ShuiDao: Intention Detection (shuidao-cognitive-engine.md:45)
# 2. IntentionDetector Architecture (shuidao-intention-detection.md:123)
# 3. Intention Workflow (shuidao-intention-workflow.md:67)
```

---

### Escenario 3: Quieres un reporte de todo

```bash
# Generar INDEX.md actualizado
./target/release/bstradivarius sync

# Ver el reporte
cat BITACORA_KNOWLEDGE_GRAPH/INDEX.md

# O exportar a JSON (para análisis)
./target/release/bstradivarius export
# → Crea: bstradivarius_export.json
```

---

## Preguntas frecuentes

### ¿BStradivarius modifica mis documentos?

**NO.** Solo **lee** y **extrae** conceptos. Tus archivos .md están seguros.

---

### ¿Qué pasa si borro un documento?

```bash
# 1. Borras: ROADMAP_V2/viejo.md
# 2. Re-sincronizas:
./target/release/bstradivarius sync

# 3. BStradivarius olvida ese documento
# (actualmente deja "huérfanos" - será arreglado)
```

---

### ¿Puedo recuperar un documento borrado con BStradivarius?

**NO.** BStradivarius solo guarda:
- Títulos
- Referencias
- Ubicación (archivo:línea)

**NO guarda el contenido completo.**

**Analogía:** Google te dice que "python tutorial" está en python.org, pero NO reconstruye la página web completa.

---

### ¿Cuánto espacio ocupa?

```
6,000 conceptos = 25MB en disco
(6,080 archivos JSON)

Es ligero, no te preocupes.
```

---

### ¿Es rápido?

**SÍ.**
- Indexar 174 archivos: 0.9 segundos
- Buscar "arquitectura": <1 segundo
- Cargar 6,000 conceptos: <1 segundo

---

## Ejemplos prácticos (copia y pega)

```bash
# Indexar TODO
./target/release/bstradivarius sync

# Buscar arquitectura
./target/release/bstradivarius query "arquitectura"

# Buscar VoxelDB
./target/release/bstradivarius query "VoxelDB"

# Buscar decisión DA-033
./target/release/bstradivarius query "DA-033"

# Ver el índice completo
cat BITACORA_KNOWLEDGE_GRAPH/INDEX.md | head -100

# Exportar a JSON
./target/release/bstradivarius export
ls -lh bstradivarius_export.json
```

---

## Solución de problemas

### "No encuentra nada"

```bash
# ¿Hiciste sync primero?
./target/release/bstradivarius sync

# Ahora busca:
./target/release/bstradivarius query "lo-que-sea"
```

---

### "Muy lento"

```bash
# ¿Cuántos conceptos tienes?
./target/release/bstradivarius export
jq '.concepts | length' bstradivarius_export.json

# Si >50,000: considera limpiar archivos viejos
```

---

### "Dice 'already running'"

```bash
# Mata el proceso anterior:
pkill -f bstradivarius

# Intenta de nuevo:
./target/release/bstradivarius sync
```

---

## Resumen en 3 líneas

```bash
# 1. Indexa tus docs (una vez al día)
./target/release/bstradivarius sync

# 2. Busca lo que necesites (instantáneo)
./target/release/bstradivarius query "lo-que-sea"

# 3. Revisa el mapa completo (opcional)
cat BITACORA_KNOWLEDGE_GRAPH/INDEX.md
```

---

**¿Más preguntas?** Lee: `BSTRADIVARIUS_FIRE_TESTS.md` (técnico) o `BSTRADIVARIUS_FLOW_DIAGRAM.md` (diagramas)

*Manual creado: 2025-11-30*  
*Versión: 1.0 - Para humanos*  
*Nivel: Principiante*
