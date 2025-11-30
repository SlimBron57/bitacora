# 🎻 BITACORA KNOWLEDGE GRAPH

## Propósito

Este directorio contiene el **índice auto-generado** de conceptos extraídos de toda la documentación de Bitácora mediante **BStradivarius**.

## Archivos

### INDEX.md (AUTO-GENERADO)
- **Qué es:** Índice completo de conceptos organizados por archivo fuente
- **Generado por:** `./target/release/bstradivarius generate`
- **Frecuencia:** Auto-regenerado después de cada `sync` con cambios
- **No editar manualmente:** Este archivo se sobreescribe en cada generación
- **Versionado:** Sí - para ver evolución del knowledge graph

### Estadísticas Actuales
- **Conceptos indexados:** 6,094
- **Archivos fuente:** 141
- **Última generación:** 2025-11-30 14:20:30
- **Tamaño:** ~207KB

## Uso

```bash
# Re-generar INDEX.md manualmente
./target/release/bstradivarius sync
./target/release/bstradivarius generate BITACORA_KNOWLEDGE_GRAPH/INDEX.md

# Buscar conceptos específicos
./target/release/bstradivarius query "arquitectura"

# Ver métricas
./target/release/bstradivarius metrics
```

## Arquitectura

INDEX.md es generado por **BStradivarius** usando:
- **VoxelDB Octree:** Persistencia espacial 3D de conceptos
- **Nombres limpios:** Conceptos indexados por nombre real ("rust", "yaml")
- **Tags metadata:** [file, line, type] para trazabilidad
- **Spatial coords:** (x: file_hash, y: line_norm, z: concept_hash)

## Relación con METOD_DOCS

**INDEX.md NO sigue METOD_DOCS** porque:
- ✅ Es un **artefacto generado**, no documentación manual
- ✅ Vive en su propio directorio fuera de `ROADMAP_V2/`
- ✅ Propósito técnico (índice queryable) vs conceptual (docs humanas)
- ✅ Ya está documentado en METOD_DOCS v1.1 sección "Integración con BStradivarius"

## Referencias

- `BSTRADIVARIUS_COMPLETE.md`: Especificación completa del sistema
- `ROADMAP_V2/GUIA.md`: Instrucciones para uso de BStradivarius
- `ROADMAP_V2/METOD_DOCS.md`: Metodología de documentación (sección BStradivarius)

---

*Directorio creado: 2025-11-30*  
*Sistema: BStradivarius Meta-Loop v0.1.0*  
*Estado: ACTIVO - Auto-regeneración habilitada*
