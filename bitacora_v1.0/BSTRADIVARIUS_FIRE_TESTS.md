# 🔥 BSTRADIVARIUS - PRUEBAS DE FUEGO

> **Test Suite Completo** para validar indexación, búsqueda, limpieza y recuperación

---

## 🎯 RESPUESTAS A TUS PREGUNTAS

### 1️⃣ Ctrl+C en Watch Mode

**Respuesta:** Solo detiene el **watcher**, termina el proceso limpiamente.

```rust
// Loop en línea 136-190 de src/bin/bstradivarius.rs
loop {
    if let Some(event) = monitor.try_recv_event() {
        // Procesa eventos...
    } else {
        // Adaptive throttling...
    }
}
// ← Ctrl+C aquí termina el loop y el proceso
```

**Comportamiento:**
- ✅ VoxelDB ya está persistido en disco (25MB, 6,080 JSON)
- ✅ No hay pérdida de datos (cada insert() persiste inmediatamente)
- ✅ El proceso termina limpiamente con exit code 0
- ❌ NO hay señal de shutdown explícita (usa Ctrl+C del SO)

---

## 2️⃣ PRUEBAS DE FUEGO 🔥

### TEST 1: Consultar Temas Almacenados

```bash
# 1. Ver cuántos conceptos hay
./target/release/bstradivarius metrics

# 2. Buscar arquitectura
./target/release/bstradivarius query "arquitectura"

# 3. Buscar VoxelDB
./target/release/bstradivarius query "VoxelDB"

# 4. Buscar FlowPacks
./target/release/bstradivarius query "FlowPacks"

# 5. Buscar por prefijo DA-
./target/release/bstradivarius query "DA-"

# 6. Exportar TODO a JSON para análisis
./target/release/bstradivarius export
# → Genera: bstradivarius_export.json con 6,094 concepts
```

**Análisis del JSON:**
```bash
# Ver estructura
jq '.concepts | length' bstradivarius_export.json
# → 6094

# Conceptos únicos
jq '.concepts[].name' bstradivarius_export.json | sort -u | wc -l

# Archivos indexados
jq '.concepts[].tags[] | select(startswith("file:"))' bstradivarius_export.json | sort -u | wc -l

# Top 10 conceptos más frecuentes
jq -r '.concepts[].name' bstradivarius_export.json | sort | uniq -c | sort -rn | head -10
```

---

### TEST 2: Contar Proceso y Flujo (CASO REAL: ShuiDao)

**Objetivo:** Trazar cómo funciona el sistema de detección de intención ShuiDao.

```bash
# 1. Buscar todas las referencias a ShuiDao
./target/release/bstradivarius query "ShuiDao" > shuidao_refs.txt

# 2. Buscar arquitectura relacionada
./target/release/bstradivarius query "IntentionDetector"
./target/release/bstradivarius query "FlowPacks"
./target/release/bstradivarius query "水道"

# 3. Exportar para análisis
./target/release/bstradivarius export
jq '.concepts[] | select(.name | test("ShuiDao|Intention|FlowPacks"; "i"))' \
   bstradivarius_export.json > shuidao_graph.json

# 4. Ver archivos relacionados
jq -r '.tags[] | select(startswith("file:")) | .[5:]' shuidao_graph.json | sort -u

# Output esperado:
# 00_VISION/08_shuidao-cognitive-architecture.md
# 01_ARQUITECTURA/12_shuidao-intention-detection.md
# 04_IMPLEMENTACION/FLOWPACKS_IMPLEMENTATION_PLAN.md
# src/shuidao/mod.rs
# examples/test_shuidao_complete.rs
```

**Análisis de Flujo:**
```bash
# Ver líneas específicas donde aparece
jq -r '.[] | "\(.tags[] | select(startswith("file:")) | .[5:]):\(.tags[] | select(startswith("line:")) | .[5:]) → \(.name)"' \
   shuidao_graph.json | sort

# Ejemplo output:
# 00_VISION/08_shuidao-cognitive-architecture.md:45 → ShuiDao Architecture
# 00_VISION/08_shuidao-cognitive-architecture.md:120 → Intention Detection
# 01_ARQUITECTURA/12_shuidao-intention-detection.md:23 → IntentionDetector
# 04_IMPLEMENTACION/FLOWPACKS_IMPLEMENTATION_PLAN.md:180 → Phase 3b: ShuiDao
```

**Conectar el flujo:**
1. **Filosofía** (00_VISION) → Qué es ShuiDao, por qué existe
2. **Arquitectura** (01_ARQUITECTURA) → Cómo funciona IntentionDetector
3. **Implementación** (04_IMPLEMENTACION) → Plan de desarrollo
4. **Código** (src/shuidao/) → Implementación real
5. **Tests** (examples/) → Validación funcional

---

### TEST 3: Depuración de Información Irrelevante

**Problema:** Cientos de documentos con info repetida, mal documentada, obsoleta.

**Solución:** BStradivarius + análisis estadístico

```bash
# 1. Exportar TODO
./target/release/bstradivarius export

# 2. Detectar documentos con pocos conceptos (candidatos a eliminar/fusionar)
jq -r '.concepts[] | .tags[] | select(startswith("file:")) | .[5:]' \
   bstradivarius_export.json | sort | uniq -c | sort -n | head -20

# Output ejemplo:
#   1 ROADMAP_V2/test_watcher.md          ← Solo 1 concepto? Eliminar
#   2 ROADMAP_V2/OLD_DESIGN.md            ← Solo 2? Revisar
#   3 cleanup_temp/notes.md               ← Temporal? Eliminar
#   5 examples/test_old.rs                ← Obsoleto? Revisar

# 3. Detectar conceptos duplicados (mismo nombre, múltiples archivos)
jq -r '.concepts[] | "\(.name)|\(.tags[] | select(startswith("file:")) | .[5:])"' \
   bstradivarius_export.json | sort | awk -F'|' '{print $1}' | uniq -c | sort -rn | head -20

# Output ejemplo:
#  23 arquitectura    ← Aparece en 23 archivos! Consolidar?
#  15 VoxelDB         ← 15 archivos mencionan VoxelDB
#   8 DA-032          ← Debería estar en 1 solo lugar

# 4. Detectar documentos sin indexar (vacíos o sin markdown válido)
find ROADMAP_V2/ -name "*.md" > all_docs.txt
jq -r '.concepts[] | .tags[] | select(startswith("file:")) | .[5:]' \
   bstradivarius_export.json | sort -u > indexed_docs.txt
comm -23 all_docs.txt indexed_docs.txt > not_indexed.txt

# Estos documentos NO tienen conceptos extraíbles:
cat not_indexed.txt
# → Candidatos a eliminar o mejorar

# 5. Identificar backups y duplicados por nombre
find . -name "*.backup" -o -name "*.old" -o -name "*_v1.*" | while read f; do
    echo "Backup/old: $f"
done
```

**Estrategia de Limpieza:**

```bash
# PASO 1: Crear lista de archivos a revisar
cat > cleanup_candidates.txt << 'EOF'
# Archivos con <3 conceptos
ROADMAP_V2/test_watcher.md
ROADMAP_V2/OLD_DESIGN.md
cleanup_temp/notes.md

# Backups detectados
examples/test_old.rs.backup
src/voxeldb/mod.rs.old

# Sin indexar (vacíos?)
ROADMAP_V2/empty_doc.md
EOF

# PASO 2: Mover a cuarentena (no eliminar directamente)
mkdir -p QUARANTINE/$(date +%Y%m%d)
while read file; do
    if [ -f "$file" ]; then
        mv "$file" "QUARANTINE/$(date +%Y%m%d)/"
        echo "Moved: $file"
    fi
done < cleanup_candidates.txt

# PASO 3: Re-sync para actualizar índice
./target/release/bstradivarius sync

# PASO 4: Validar que no rompimos nada (buscar referencias)
./target/release/bstradivarius query "test_watcher"
# → Debería estar vacío si el archivo fue movido

# PASO 5: Si todo OK, eliminar cuarentena después de 1 semana
# rm -rf QUARANTINE/20251130/
```

---

### TEST 4: Prueba de Recuperación Temporal

**Objetivo:** Crear docs de prueba, validar indexación, eliminar sin dejar basura.

```bash
# ========================================
# FASE 1: CREAR DOCUMENTOS DE PRUEBA
# ========================================

mkdir -p /tmp/bstradivarius_test

# Doc 1: Arquitectura ficticia
cat > /tmp/bstradivarius_test/test_quantum_arch.md << 'EOF'
# Quantum Architecture Test

## Core Components
- [[quantum-processor]]
- **quantum-memory**
- `quantum-bus`

## Implementation Details
Este es un documento de prueba para validar:
1. Extracción de headings
2. Extracción de wikilinks
3. Extracción de términos en bold
4. Extracción de código inline

## References
- DA-999: Quantum Test Document
- Ver también: [[quantum-state]]
EOF

# Doc 2: Especificación ficticia
cat > /tmp/bstradivarius_test/test_spec.md << 'EOF'
# Test Specification

## Objetivo
Validar que BStradivarius indexa correctamente documentos temporales.

## Conceptos Clave
- **temporal-indexing**
- **test-validation**
- [[cleanup-strategy]]

## Flujo
1. Crear doc → 2. Sync → 3. Query → 4. Eliminar → 5. Verificar limpieza
EOF

# Doc 3: Código de ejemplo
cat > /tmp/bstradivarius_test/test_code.md << 'EOF'
# Code Example

```rust
// Test quantum processor
pub struct QuantumProcessor {
    state: QuantumState,
}
```

## Dependencies
- rust-quantum v1.0
- [[quantum-sdk]]
EOF

# ========================================
# FASE 2: MOVER A ROADMAP_V2 TEMPORAL
# ========================================

cp -r /tmp/bstradivarius_test ROADMAP_V2/_TEST_TEMP/

# ========================================
# FASE 3: SYNC Y VALIDAR INDEXACIÓN
# ========================================

echo "=== ANTES DE SYNC ==="
./target/release/bstradivarius query "quantum" | wc -l

echo "=== SYNC ==="
./target/release/bstradivarius sync

echo "=== DESPUÉS DE SYNC ==="
./target/release/bstradivarius query "quantum"
# Output esperado:
# 1. quantum-processor (test_quantum_arch.md:5)
# 2. quantum-memory (test_quantum_arch.md:6)
# 3. quantum-bus (test_quantum_arch.md:7)
# 4. quantum-state (test_quantum_arch.md:16)
# ...

./target/release/bstradivarius query "temporal-indexing"
# Output esperado:
# 1. temporal-indexing (test_spec.md:9)

./target/release/bstradivarius query "QuantumProcessor"
# Output esperado:
# 1. QuantumProcessor (test_code.md:5)

# ========================================
# FASE 4: EXPORTAR PARA ANÁLISIS
# ========================================

./target/release/bstradivarius export
jq '.concepts[] | select(.tags[] | contains("_TEST_TEMP"))' \
   bstradivarius_export.json > test_concepts.json

echo "=== CONCEPTOS INDEXADOS DE PRUEBA ==="
jq -r '.name' test_concepts.json | sort -u
# Output esperado:
# cleanup-strategy
# quantum-bus
# quantum-memory
# quantum-processor
# quantum-sdk
# quantum-state
# QuantumProcessor
# rust-quantum v1.0
# temporal-indexing
# test-validation

echo "=== TOTAL: $(jq -s 'length' test_concepts.json) conceptos ==="

# ========================================
# FASE 5: ELIMINAR Y VALIDAR LIMPIEZA
# ========================================

echo "=== ELIMINANDO DOCS DE PRUEBA ==="
rm -rf ROADMAP_V2/_TEST_TEMP/

echo "=== RE-SYNC PARA ACTUALIZAR ==="
./target/release/bstradivarius sync

echo "=== VERIFICAR QUE CONCEPTOS FUERON ELIMINADOS ==="
./target/release/bstradivarius query "quantum-processor"
# Output esperado: VACÍO (o solo conceptos reales si existían antes)

./target/release/bstradivarius query "temporal-indexing"
# Output esperado: VACÍO

# ========================================
# FASE 6: VERIFICAR PERSISTENCIA
# ========================================

# Los archivos JSON en VoxelDB deberían estar limpios
echo "=== ARCHIVOS EN VOXELDB ==="
ls -lh data/watcher_voxeldb/ | wc -l

# Si hay conceptos huérfanos (archivos eliminados pero JSON persiste):
echo "=== BUSCAR HUÉRFANOS ==="
jq -r '.concepts[] | .tags[] | select(startswith("file:")) | .[5:]' \
   bstradivarius_export.json | sort -u | while read f; do
    if [ ! -f "$f" ]; then
        echo "HUÉRFANO: $f (archivo eliminado pero concepto persiste)"
    fi
done

# Si hay huérfanos, necesitamos implementar garbage collection en VoxelDB
```

---

## 🧹 LIMPIEZA PROFUNDA: Comando `cleanup`

**Problema actual:** VoxelDB no tiene garbage collection automático.

**Solución propuesta:**

```bash
# NUEVO COMANDO: bstradivarius cleanup
./target/release/bstradivarius cleanup [--dry-run]

# ¿Qué hace?
# 1. Load all concepts from VoxelDB
# 2. Para cada concepto, verificar si el archivo fuente existe
# 3. Si NO existe → marcar para eliminación
# 4. Si --dry-run → solo reportar
# 5. Si NO --dry-run → eliminar JSON huérfanos
```

**Implementación necesaria:**

```rust
// src/bin/bstradivarius.rs
fn cmd_cleanup(dry_run: bool) -> Result<()> {
    CliFormatter::print_banner();
    CliFormatter::print_stage("Cleanup", "removing orphaned concepts...");
    
    let config = WatcherConfig::default();
    let mut indexer = ConceptIndexer::new(&config.voxel_db_path)?;
    
    // Get all concepts
    let all_concepts = indexer.query_concepts("")?;
    
    let mut orphaned = Vec::new();
    for concept in &all_concepts {
        if !concept.file.exists() {
            orphaned.push(concept);
        }
    }
    
    println!("   Found {} orphaned concepts", orphaned.len());
    
    if dry_run {
        println!("   (Dry run - no changes made)");
        for concept in &orphaned {
            println!("      ✗ {} ({}:{})", concept.concept, 
                concept.file.display(), concept.line);
        }
    } else {
        // TODO: Implement removal in VoxelDB
        println!("   Removing orphaned concepts...");
        // indexer.remove_concepts(&orphaned)?;
    }
    
    Ok(())
}
```

---

## 🎯 ESTRATEGIA: BStradivarius como Fuente de Verdad

### Principios de Pureza

1. **Solo indexar documentación oficial**
   ```bash
   # En config.toml
   watched_paths = [
       "ROADMAP_V2/00_VISION/",
       "ROADMAP_V2/01_ARQUITECTURA/",
       "ROADMAP_V2/02_COMPONENTES/",
       # NO incluir: cleanup_temp/, SANDBOX/, 00_BACKUPS/
   ]
   ```

2. **Prefijos para control**
   ```
   _TEMP.md       → No indexar (comienza con _)
   .backup        → No indexar (extensión backup)
   draft_*.md     → No indexar (draft en nombre)
   ```

3. **Validación periódica**
   ```bash
   # Cada semana
   ./target/release/bstradivarius cleanup --dry-run
   ./target/release/bstradivarius export
   # Analizar export: conceptos duplicados, huérfanos, inconsistencias
   ```

4. **Documentación estratificada**
   ```
   FILOSOFÍA    → 00_VISION/        (core inmutable)
   ARQUITECTURA → 01_ARQUITECTURA/  (estable)
   COMPONENTES  → 02_COMPONENTES/   (evolutivo)
   CÓDIGO       → src/              (indexar comments)
   ```

5. **Garbage collection automático**
   ```rust
   // En cmd_sync, después de indexar:
   if changed_files > 0 {
       cmd_cleanup(false)?;  // Limpiar huérfanos
       cmd_generate("INDEX.md")?;
   }
   ```

---

## 📊 DASHBOARD DE SALUD

```bash
# Script: bstradivarius_health.sh
#!/bin/bash

echo "🎻 BSTRADIVARIUS HEALTH CHECK"
echo "================================"

# 1. Total concepts
TOTAL=$(./target/release/bstradivarius export 2>/dev/null | jq '.concepts | length')
echo "Total Concepts: $TOTAL"

# 2. Files indexed
FILES=$(jq -r '.concepts[] | .tags[] | select(startswith("file:")) | .[5:]' \
   bstradivarius_export.json | sort -u | wc -l)
echo "Files Indexed: $FILES"

# 3. Orphaned (files deleted)
ORPHANED=$(jq -r '.concepts[] | .tags[] | select(startswith("file:")) | .[5:]' \
   bstradivarius_export.json | sort -u | while read f; do
    [ ! -f "$f" ] && echo "$f"
done | wc -l)
echo "Orphaned: $ORPHANED ⚠️"

# 4. Duplicates (same name, >10 occurrences)
DUPS=$(jq -r '.concepts[].name' bstradivarius_export.json | \
   sort | uniq -c | awk '$1 > 10 {print $2}' | wc -l)
echo "High Duplicates (>10): $DUPS"

# 5. VoxelDB size
VOXEL_SIZE=$(du -sh data/watcher_voxeldb/ | awk '{print $1}')
echo "VoxelDB Size: $VOXEL_SIZE"

echo ""
echo "✅ Health check complete"
```

---

## 🔥 PRUEBA DE FUEGO COMPLETA

```bash
#!/bin/bash
# test_bstradivarius_fire.sh

set -e

echo "🔥🔥🔥 BSTRADIVARIUS FIRE TEST 🔥🔥🔥"

# TEST 1: Sync completo
echo "TEST 1: Full sync"
time ./target/release/bstradivarius sync
echo "✅ PASS"

# TEST 2: Query existente
echo "TEST 2: Query 'VoxelDB'"
RESULTS=$(./target/release/bstradivarius query "VoxelDB" | grep -c "Context:" || true)
if [ "$RESULTS" -gt 0 ]; then
    echo "✅ PASS ($RESULTS results)"
else
    echo "❌ FAIL (no results)"
    exit 1
fi

# TEST 3: Crear doc temporal
echo "TEST 3: Create temporary doc"
mkdir -p /tmp/fire_test
echo "# Fire Test Doc

## Concepts
- **fire-test-concept**
- [[fire-test-link]]
" > /tmp/fire_test/fire.md

cp /tmp/fire_test/fire.md ROADMAP_V2/_FIRE_TEST.md
./target/release/bstradivarius sync > /dev/null 2>&1

FIRE_RESULTS=$(./target/release/bstradivarius query "fire-test-concept" | grep -c "Context:" || true)
if [ "$FIRE_RESULTS" -eq 1 ]; then
    echo "✅ PASS (indexed)"
else
    echo "❌ FAIL (not indexed)"
    exit 1
fi

# TEST 4: Eliminar y verificar
echo "TEST 4: Delete and verify cleanup"
rm ROADMAP_V2/_FIRE_TEST.md
./target/release/bstradivarius sync > /dev/null 2>&1

AFTER_DELETE=$(./target/release/bstradivarius query "fire-test-concept" | grep -c "Context:" || true)
if [ "$AFTER_DELETE" -eq 0 ]; then
    echo "✅ PASS (cleaned up)"
else
    echo "❌ FAIL (orphaned: $AFTER_DELETE)"
    exit 1
fi

# TEST 5: Export válido
echo "TEST 5: Valid export"
./target/release/bstradivarius export > /dev/null 2>&1
if jq empty bstradivarius_export.json 2>/dev/null; then
    echo "✅ PASS (valid JSON)"
else
    echo "❌ FAIL (invalid JSON)"
    exit 1
fi

echo ""
echo "🎉🎉🎉 ALL TESTS PASSED 🎉🎉🎉"
```

---

*Guía creada: 2025-11-30*  
*Sistema: BStradivarius Meta-Loop v0.1.0*  
*Propósito: Validación completa, limpieza y recuperación*
