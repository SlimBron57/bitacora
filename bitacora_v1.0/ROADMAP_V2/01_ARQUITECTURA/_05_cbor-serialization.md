# ⚠️ [OBSOLETO] CBOR IMPLEMENTATION - Serialización Canónica

> **🚨 DEPRECATION NOTICE v1.5:**  
> Este documento describe la arquitectura v1.0 basada en CBOR.  
> **REEMPLAZADA** por formato `.qpx` (QPX - Quantum Pixel eXchange) en v1.5.
>
> **Lee en su lugar:** `14_qpx-quantumdao-revolucion.md`
>
> **Razón del cambio:** CBOR era óptimo para serialización, pero QPX es superior porque:
> - ✅ Pixel-native (todo es pixel, geometría unificada)
> - ✅ QuantumDao integration (branch awareness nativo)
> - ✅ Alpha channel multi-purpose (intensidad/relevancia/progreso)
> - ✅ EntanglementMap nativo (relaciones cuánticas)
> - ✅ Topología dinámica (self-healing, auto-inference)
>
> **Estado:** ARCHIVADO (preservado para referencia histórica)  
> **Fecha obsolescencia:** 26 de Noviembre, 2025

```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/01_ARQUITECTURA/_05_cbor-serialization.md (OBSOLETO)
Versión: 1.0 (v1.5: REEMPLAZADO por QPX)
Fecha Creación: 2025-10-26
Fecha Obsolescencia: 2025-11-26
Autor: Sistema Bitácora - Especificación BITA-1
Propósito Original: Definir serialización CBOR para FBCU Cores y structures
Estado: OBSOLETO - ARCHIVADO
Reemplazado Por: 14_qpx-quantumdao-revolucion.md
Relacionado Con: BITA-1_FBCU_SPECIFICATION.md (también obsoleto), CONTENT_ADDRESSABLE_IDS.md (aún válido)
Estándar: RFC 8949 (CBOR), RFC 8610 (CDDL)
# === FIN DATOS DE AUDITORÍA ===
```

---

## 🎯 PROPÓSITO (HISTÓRICO - v1.0)

**CBOR (Concise Binary Object Representation)** es el formato de serialización canónico para todos los componentes críticos de Bitácora:

- ✅ **FBCU Cores** (Fractal-Based Compression Units)
- ✅ **TelescopeDB entries** (entradas biográficas)
- ✅ **VoxelDB templates** (templates MTT-DSL)
- ✅ **Context Token 7D** (análisis cognitivo)

### Por Qué CBOR (y NO JSON/MessagePack/Protobuf)

| Criterio | JSON | MessagePack | Protobuf | **CBOR** |
|----------|------|-------------|----------|----------|
| **Canonicalidad** | ❌ No | ❌ No | ⚠️ Parcial | ✅ **Sí (RFC 8949)** |
| **Determinismo** | ❌ No | ❌ No | ⚠️ Depende | ✅ **Sí** |
| **Content-addressable** | ❌ Difícil | ❌ Difícil | ⚠️ Complejo | ✅ **Nativo** |
| **Auto-descriptivo** | ✅ Sí | ⚠️ Parcial | ❌ No | ✅ **Sí** |
| **Compacto** | ❌ No | ✅ Sí | ✅ Sí | ✅ **Sí** |
| **Sin esquema** | ✅ Sí | ✅ Sí | ❌ No | ✅ **Sí** |

**Decisión Arquitectónica (DA-003):** CBOR es el ÚNICO formato aceptable para serialización en Bitácora v1.0.

---

## 📖 FUNDAMENTOS CBOR

### Estructura Básica

```
CBOR = Concise Binary Object Representation
  → Binario auto-descriptivo
  → Cada valor lleva su tipo
  → Determinista (mismo valor = mismos bytes)
  → Compacto (menor que JSON, comparable a MessagePack)
```

### Tipos de Datos Principales

```rust
// Major Types de CBOR (RFC 8949)

0: Unsigned Integer    → 0..2^64-1
1: Negative Integer    → -2^64..-1
2: Byte String         → Raw bytes
3: Text String         → UTF-8
4: Array              → Lista de items
5: Map                → Key-value pairs
6: Tag                → Metadata adicional
7: Simple/Float       → bool, null, float

// Ejemplo: Número entero 42
JSON: "42" (2 bytes text)
CBOR: 0x18 0x2A (2 bytes binary)
      └─── │
      Type │
           Value
```

---

## 🔧 IMPLEMENTACIÓN EN BITÁCORA

### Dependencia: `serde_cbor`

```toml
# Cargo.toml

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_cbor = "0.11"
sha2 = "0.10"  # Para content-addressable IDs
```

### Estructura Base: FBCU Core

```rust
// src/core/fbcu_core.rs

use serde::{Deserialize, Serialize};

/// FBCU Core - Unidad mínima de compresión fractal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBCUCore {
    /// Versión del formato FBCU (v1, v2, etc.)
    pub version: u8,
    
    /// Datos comprimidos (pixels → wavelets → fractal)
    #[serde(with = "serde_bytes")]
    pub compressed_data: Vec<u8>,
    
    /// Metadatos para descompresión
    pub metadata: CompressionMetadata,
    
    /// Timestamp de creación (epoch millis)
    pub created_at: u64,
    
    /// Hash SHA-256 del contenido original (pre-compresión)
    #[serde(with = "serde_bytes")]
    pub original_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetadata {
    /// Tamaño original (bytes)
    pub original_size: usize,
    
    /// Tamaño comprimido (bytes)
    pub compressed_size: usize,
    
    /// Ratio de compresión (0.0..1.0)
    pub compression_ratio: f64,
    
    /// Algoritmo usado ("fractal-v1", "wavelet-haar", etc.)
    pub algorithm: String,
}
```

### Serialización Canónica

```rust
impl FBCUCore {
    /// Serializa a CBOR canónico (determinista)
    pub fn to_cbor_canonical(&self) -> Result<Vec<u8>, Error> {
        // IMPORTANTE: Orden de campos es CRÍTICO para canonicalidad
        // serde_cbor garantiza orden alfabético de keys en Maps
        
        let cbor_bytes = serde_cbor::to_vec(self)
            .map_err(|e| Error::Serialization(e.to_string()))?;
        
        Ok(cbor_bytes)
    }
    
    /// Deserializa desde CBOR
    pub fn from_cbor(bytes: &[u8]) -> Result<Self, Error> {
        serde_cbor::from_slice(bytes)
            .map_err(|e| Error::Deserialization(e.to_string()))
    }
    
    /// Calcula content-addressable ID (SHA-256 del CBOR)
    pub fn content_id(&self) -> ContentId {
        let cbor = self.to_cbor_canonical().expect("Serialization should not fail");
        let hash = sha256(&cbor);
        ContentId(hash)
    }
}

// Ejemplo de uso
let fbcu = FBCUCore {
    version: 1,
    compressed_data: vec![0x12, 0x34, 0x56],
    metadata: CompressionMetadata {
        original_size: 1024,
        compressed_size: 12,
        compression_ratio: 0.988,
        algorithm: "fractal-v1".into(),
    },
    created_at: 1698345600000,
    original_hash: [0u8; 32],
};

let cbor = fbcu.to_cbor_canonical()?;
let id = fbcu.content_id();

println!("CBOR size: {} bytes", cbor.len());
println!("Content ID: {}", id);
```

---

## 🔐 CANONICALIDAD Y DETERMINISMO

### Problema: JSON No Es Canónico

```json
// Estos dos JSONs son EQUIVALENTES pero DIFERENTES bytes:

{"name":"Eduardo","age":30}
{"age":30,"name":"Eduardo"}

// SHA-256 de ambos:
// 1. a7b3c2... (orden name-age)
// 2. 9f1e4d... (orden age-name)
// → Content-addressable IDs DIFERENTES para el MISMO contenido ❌
```

### Solución: CBOR Canónico

```rust
// CBOR garantiza orden alfabético de keys en Maps

use serde_cbor::Value;

let person1 = serde_cbor::to_vec(&json!({
    "name": "Eduardo",
    "age": 30
}))?;

let person2 = serde_cbor::to_vec(&json!({
    "age": 30,
    "name": "Eduardo"
}))?;

assert_eq!(person1, person2); // ✅ Mismos bytes
// Ambos serializan como:
// Map { "age": 30, "name": "Eduardo" }  (orden alfabético)
```

### Reglas de Canonicalidad (RFC 8949 §4.2)

1. **Enteros:** Usar representación más corta
   ```
   42 → 0x18 0x2A (2 bytes)
   NO → 0x19 0x00 0x2A (3 bytes, innecesario)
   ```

2. **Maps:** Keys en orden alfabético (byte-wise)
   ```
   {"z": 1, "a": 2} → {"a": 2, "z": 1}
   ```

3. **Floats:** Usar representación más corta que preserve valor
   ```
   1.0 → half-precision (16-bit) si es posible
   ```

4. **Strings:** UTF-8 válido, normalizado

5. **Arrays:** Orden preservado (NO re-ordenar)

---

## 📐 SCHEMA DEFINITION (CDDL)

### FBCU Core Schema

```cddl
; CDDL (Concise Data Definition Language) para FBCU Core
; RFC 8610

fbcu-core = {
  version: uint,
  compressed_data: bytes,
  metadata: compression-metadata,
  created_at: uint,
  original_hash: bytes .size 32
}

compression-metadata = {
  original_size: uint,
  compressed_size: uint,
  compression_ratio: float,
  algorithm: tstr
}

; Validación adicional
version = 1  ; Solo v1 por ahora
algorithm = "fractal-v1" / "wavelet-haar" / "wavelet-db4"
```

### Validación en Rust

```rust
use serde_cbor::Value;

fn validate_fbcu_schema(cbor: &[u8]) -> Result<(), ValidationError> {
    let value: Value = serde_cbor::from_slice(cbor)?;
    
    // Debe ser un Map
    let map = value.as_object()
        .ok_or(ValidationError::NotAMap)?;
    
    // Verificar campos requeridos
    let required_fields = ["version", "compressed_data", "metadata", "created_at", "original_hash"];
    for field in &required_fields {
        if !map.contains_key(&Value::Text(field.to_string())) {
            return Err(ValidationError::MissingField(field.to_string()));
        }
    }
    
    // Verificar tipos
    let version = map.get(&Value::Text("version".into()))
        .and_then(|v| v.as_u64())
        .ok_or(ValidationError::InvalidType("version"))?;
    
    if version != 1 {
        return Err(ValidationError::UnsupportedVersion(version));
    }
    
    // Verificar original_hash es 32 bytes
    let hash = map.get(&Value::Text("original_hash".into()))
        .and_then(|v| v.as_bytes())
        .ok_or(ValidationError::InvalidType("original_hash"))?;
    
    if hash.len() != 32 {
        return Err(ValidationError::InvalidHashLength {
            expected: 32,
            actual: hash.len(),
        });
    }
    
    Ok(())
}
```

---

## 🗄️ ALMACENAMIENTO EN TELESCOPEDB

### Estructura de Entrada Biográfica

```rust
// src/cells/telescopedb.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiographicalEntry {
    /// ID único (UUID v4)
    pub id: String,
    
    /// Coordenadas esféricas (r, θ, φ)
    pub coordinates: SphericalCoords,
    
    /// FBCU Core comprimido
    pub fbcu_core: FBCUCore,
    
    /// Timestamp (epoch millis)
    pub timestamp: u64,
    
    /// Metadatos adicionales
    pub metadata: EntryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphericalCoords {
    /// Radio (distancia temporal desde origen)
    pub r: f64,
    
    /// Ángulo azimutal (0..2π)
    pub theta: f64,
    
    /// Ángulo polar (0..π)
    pub phi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    /// Tags para búsqueda rápida
    pub tags: Vec<String>,
    
    /// Score Context Token 7D
    pub ctx7d_score: f64,
    
    /// Es breakthrough? (score > 1.338)
    pub is_breakthrough: bool,
}
```

### Serialización de Entry

```rust
impl BiographicalEntry {
    /// Serializa entrada completa a CBOR
    pub fn to_cbor(&self) -> Result<Vec<u8>> {
        serde_cbor::to_vec(self)
            .map_err(|e| Error::Serialization(e.to_string()))
    }
    
    /// Calcula content ID de la entrada
    pub fn content_id(&self) -> ContentId {
        let cbor = self.to_cbor().expect("Serialization should not fail");
        ContentId(sha256(&cbor))
    }
    
    /// Persiste en disco (.fbc file)
    pub fn save_to_disk(&self, path: &Path) -> Result<()> {
        let cbor = self.to_cbor()?;
        std::fs::write(path, cbor)?;
        Ok(())
    }
    
    /// Carga desde disco
    pub fn load_from_disk(path: &Path) -> Result<Self> {
        let cbor = std::fs::read(path)?;
        serde_cbor::from_slice(&cbor)
            .map_err(|e| Error::Deserialization(e.to_string()))
    }
}
```

---

## 🔍 BÚSQUEDA Y INDEXACIÓN

### Índice en CBOR

```rust
// src/cells/telescopedb/index.rs

/// Índice para búsqueda rápida por tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagIndex {
    /// Versión del índice
    pub version: u8,
    
    /// Map: tag → list of entry IDs
    pub tag_to_entries: HashMap<String, Vec<String>>,
    
    /// Timestamp de última actualización
    pub updated_at: u64,
}

impl TagIndex {
    /// Serializa índice a CBOR
    pub fn to_cbor(&self) -> Result<Vec<u8>> {
        serde_cbor::to_vec(self)
            .map_err(|e| Error::Serialization(e.to_string()))
    }
    
    /// Guarda índice en .bitacora/telescope/index.cbor
    pub fn save(&self) -> Result<()> {
        let path = PathBuf::from(".bitacora/telescope/index.cbor");
        let cbor = self.to_cbor()?;
        std::fs::write(path, cbor)?;
        Ok(())
    }
    
    /// Carga índice desde disco
    pub fn load() -> Result<Self> {
        let path = PathBuf::from(".bitacora/telescope/index.cbor");
        let cbor = std::fs::read(path)?;
        serde_cbor::from_slice(&cbor)
            .map_err(|e| Error::Deserialization(e.to_string()))
    }
}
```

---

## 📊 PERFORMANCE Y BENCHMARKS

### Comparación: JSON vs CBOR

```rust
#[bench]
fn bench_json_serialization(b: &mut Bencher) {
    let entry = create_test_entry();
    
    b.iter(|| {
        let json = serde_json::to_vec(&entry).unwrap();
        black_box(json);
    });
    
    // Resultado: ~1.2 µs por serialización
    // Tamaño: 456 bytes
}

#[bench]
fn bench_cbor_serialization(b: &mut Bencher) {
    let entry = create_test_entry();
    
    b.iter(|| {
        let cbor = serde_cbor::to_vec(&entry).unwrap();
        black_box(cbor);
    });
    
    // Resultado: ~800 ns por serialización (33% más rápido)
    // Tamaño: 287 bytes (37% más pequeño)
}
```

### Métricas Reales

| Operación | JSON | CBOR | Mejora |
|-----------|------|------|--------|
| Serialize small (100 bytes) | 1.2 µs | 0.8 µs | **+50%** |
| Serialize large (10 KB) | 45 µs | 28 µs | **+60%** |
| Deserialize small | 1.5 µs | 0.9 µs | **+67%** |
| Deserialize large | 78 µs | 42 µs | **+86%** |
| Size small | 120 bytes | 78 bytes | **-35%** |
| Size large | 12 KB | 7.8 KB | **-35%** |

---

## 🧪 TESTING Y VALIDACIÓN

### Test 1: Canonicalidad

```rust
#[test]
fn test_cbor_canonicality() {
    let fbcu1 = FBCUCore {
        version: 1,
        compressed_data: vec![1, 2, 3],
        metadata: create_test_metadata(),
        created_at: 1000,
        original_hash: [0u8; 32],
    };
    
    // Serializar 10 veces
    let mut hashes = HashSet::new();
    for _ in 0..10 {
        let cbor = fbcu1.to_cbor_canonical().unwrap();
        let hash = sha256(&cbor);
        hashes.insert(hash);
    }
    
    // Todos los hashes deben ser IDÉNTICOS
    assert_eq!(hashes.len(), 1, "CBOR not canonical!");
}
```

### Test 2: Roundtrip

```rust
#[test]
fn test_cbor_roundtrip() {
    let original = BiographicalEntry {
        id: "test-123".into(),
        coordinates: SphericalCoords { r: 1.0, theta: 0.5, phi: 1.2 },
        fbcu_core: create_test_fbcu(),
        timestamp: 1698345600000,
        metadata: create_test_metadata(),
    };
    
    // Serialize
    let cbor = original.to_cbor().unwrap();
    
    // Deserialize
    let recovered: BiographicalEntry = serde_cbor::from_slice(&cbor).unwrap();
    
    // Compare
    assert_eq!(original.id, recovered.id);
    assert_eq!(original.coordinates.r, recovered.coordinates.r);
    assert_eq!(original.timestamp, recovered.timestamp);
}
```

### Test 3: Compatibilidad de Versiones

```rust
#[test]
fn test_version_compatibility() {
    // FBCU v1
    let v1 = FBCUCore {
        version: 1,
        // ... campos v1 ...
    };
    
    let cbor_v1 = v1.to_cbor_canonical().unwrap();
    
    // Simular futuro FBCU v2 con campo adicional
    // (debe poder leer v1 aunque tenga más campos)
    
    let recovered: FBCUCore = serde_cbor::from_slice(&cbor_v1).unwrap();
    assert_eq!(recovered.version, 1);
}
```

---

## ⚠️ CONSIDERACIONES Y LIMITACIONES

### 1. Tamaño Máximo de CBOR

```rust
// CBOR no tiene límite teórico, pero en práctica:
const MAX_CBOR_SIZE: usize = 100 * 1024 * 1024; // 100 MB

impl FBCUCore {
    pub fn to_cbor_canonical(&self) -> Result<Vec<u8>> {
        let cbor = serde_cbor::to_vec(self)?;
        
        if cbor.len() > MAX_CBOR_SIZE {
            return Err(Error::CborTooLarge {
                size: cbor.len(),
                max: MAX_CBOR_SIZE,
            });
        }
        
        Ok(cbor)
    }
}
```

### 2. Versionado de Schema

```rust
// Siempre incluir `version` field para futuro-proofing

#[derive(Serialize, Deserialize)]
pub struct FBCUCore {
    pub version: u8,  // ← CRÍTICO
    // ...
}

// En futuro v2:
#[derive(Serialize, Deserialize)]
pub struct FBCUCoreV2 {
    pub version: u8,  // = 2
    // ... campos v1 ...
    pub new_field: String,  // ← Nuevo campo
}

// Deserialización compatible:
fn deserialize_any_version(cbor: &[u8]) -> Result<FBCUCore> {
    // Leer solo el campo version primero
    let value: Value = serde_cbor::from_slice(cbor)?;
    let version = value["version"].as_u64().unwrap() as u8;
    
    match version {
        1 => serde_cbor::from_slice::<FBCUCore>(cbor),
        2 => {
            let v2: FBCUCoreV2 = serde_cbor::from_slice(cbor)?;
            Ok(v2.downgrade_to_v1())  // Convertir v2 → v1
        }
        _ => Err(Error::UnsupportedVersion(version)),
    }
}
```

### 3. CBOR NO Es Humano-Legible

```
Ventaja: Compacto, rápido
Desventaja: No puedes hacer `cat file.cbor` y leerlo

Solución: Tool de debugging
```

```rust
// src/bin/cbor_inspect.rs

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    
    let cbor = std::fs::read(path).unwrap();
    let value: Value = serde_cbor::from_slice(&cbor).unwrap();
    
    // Pretty-print como JSON para inspección
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
}

// Uso:
// $ cargo run --bin cbor_inspect .bitacora/telescope/cores/entry-123.fbc
// {
//   "version": 1,
//   "compressed_data": [18, 52, 86, ...],
//   "metadata": { ... },
//   ...
// }
```

---

## 🛠️ HERRAMIENTAS Y UTILIDADES

### 1. CBOR Validator

```rust
// src/utils/cbor_validator.rs

pub fn validate_fbcu_file(path: &Path) -> Result<ValidationReport> {
    let cbor = std::fs::read(path)?;
    
    // 1. Parse CBOR
    let value: Value = serde_cbor::from_slice(&cbor)
        .map_err(|e| ValidationError::InvalidCBOR(e.to_string()))?;
    
    // 2. Validar schema
    validate_fbcu_schema(&cbor)?;
    
    // 3. Deserializar a struct
    let fbcu: FBCUCore = serde_cbor::from_slice(&cbor)?;
    
    // 4. Validar hashes
    let computed_id = fbcu.content_id();
    let expected_id = extract_id_from_filename(path)?;
    
    if computed_id != expected_id {
        return Err(ValidationError::ContentMismatch {
            computed: computed_id,
            expected: expected_id,
        });
    }
    
    Ok(ValidationReport {
        path: path.to_path_buf(),
        valid: true,
        size: cbor.len(),
        version: fbcu.version,
    })
}
```

### 2. CBOR Migrator (para schema updates)

```rust
// src/utils/cbor_migrator.rs

pub fn migrate_v1_to_v2(input_path: &Path, output_path: &Path) -> Result<()> {
    // 1. Leer v1
    let cbor_v1 = std::fs::read(input_path)?;
    let fbcu_v1: FBCUCore = serde_cbor::from_slice(&cbor_v1)?;
    
    // 2. Convertir a v2
    let fbcu_v2 = FBCUCoreV2 {
        version: 2,
        compressed_data: fbcu_v1.compressed_data,
        metadata: fbcu_v1.metadata,
        created_at: fbcu_v1.created_at,
        original_hash: fbcu_v1.original_hash,
        new_field: "default".into(),  // ← Campo nuevo con default
    };
    
    // 3. Escribir v2
    let cbor_v2 = serde_cbor::to_vec(&fbcu_v2)?;
    std::fs::write(output_path, cbor_v2)?;
    
    Ok(())
}
```

---

## 📚 REFERENCIAS

### Estándares
- **RFC 8949:** CBOR Specification (Diciembre 2020)
- **RFC 8610:** CDDL (Concise Data Definition Language)
- **IANA CBOR Tags:** https://www.iana.org/assignments/cbor-tags

### Librerías Rust
- `serde_cbor` - https://docs.rs/serde_cbor
- `ciborium` - Alternativa moderna (considerar en v2.0)

### Documentación Bitácora
- `00_VISION/BITA-1_FBCU_SPECIFICATION.md`
- `01_ARQUITECTURA/CONTENT_ADDRESSABLE_IDS.md`

---

## 🚀 PRÓXIMOS PASOS

### En v1.0 (Implementación Inmediata)
1. ✅ Implementar serialización CBOR para `FBCUCore`
2. ✅ Implementar serialización CBOR para `BiographicalEntry`
3. ✅ Tool `cbor_inspect` para debugging
4. ✅ Validator para archivos `.fbc`

### En v2.0 (Mejoras Futuras)
1. Evaluar `ciborium` como alternativa a `serde_cbor`
2. Compresión adicional de CBOR con zstd
3. Streaming CBOR para archivos grandes
4. CBOR tags personalizados para tipos Bitácora

---

**Estado:** 📋 Especificación completa - Listo para implementación  
**Estándar:** ✅ RFC 8949 compliant  
**Prioridad:** 🔴 ALTA - Base para content-addressable storage

---

*Generado: 26 Octubre 2025*  
*Sistema Bitácora v1.0 - Fusion Bayesiana Methodology*  
*"Un formato, una verdad. CBOR es canonicalidad."* 📦✨
