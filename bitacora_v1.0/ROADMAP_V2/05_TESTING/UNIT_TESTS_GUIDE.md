# 🧪 Unit Tests Guide: Testing Unitario en Bitácora

**Archivo:** `ROADMAP_V2/05_TESTING/UNIT_TESTS_GUIDE.md`  
**Versión:** 1.0  
**Fecha:** 2025-10-26  
**Propósito:** Guía completa de testing unitario para componentes críticos e importantes

---

## 🎯 PROPÓSITO

El testing unitario en Bitácora sigue el principio **"test what matters"**: cada función pública debe tener tests que validen su comportamiento esperado, edge cases y failure modes.

---

## 📊 ESTRUCTURA DE TESTS

```mermaid
graph TB
    subgraph "ORGANIZACIÓN"
        A[src/component/mod.rs] --> B[Código producción]
        A --> C[#[cfg test] mod tests]
        C --> D[Tests unitarios]
    end
    
    subgraph "TIPOS DE TESTS"
        D --> E[Happy Path]
        D --> F[Edge Cases]
        D --> G[Error Handling]
        D --> H[Property-Based]
    end
    
    subgraph "ASERCIONES"
        E --> I[assert_eq!]
        F --> J[assert!]
        G --> K[assert matches Err]
        H --> L[proptest!]
    end
    
    style A fill:#3498db,stroke:#2980b9,stroke-width:2px,color:#fff
    style C fill:#e74c3c,stroke:#c0392b,stroke-width:2px,color:#fff
    style I fill:#27ae60,stroke:#229954,stroke-width:2px,color:#fff
```

---

## 🏗️ TEMPLATE DE TEST MODULE

```rust
// src/cells/telescopedb.rs (ejemplo)

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    // Helper: crear DB temporal para cada test
    fn setup_temp_db() -> (TelescopeDB, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = TelescopeDB::new(&db_path).unwrap();
        (db, temp_dir)
    }
    
    #[test]
    fn test_insert_and_retrieve_pixel() {
        let (db, _temp) = setup_temp_db();
        
        // Arrange
        let pixel = Pixel {
            position: PixelCoord { x: 10, y: 20 },
            color: LAB { l: 50.0, a: 10.0, b: -5.0 },
            timestamp: 1234567890,
        };
        
        // Act
        let pixel_id = db.insert_pixel(&pixel).unwrap();
        let retrieved = db.get_pixel(pixel_id).unwrap();
        
        // Assert
        assert_eq!(retrieved.position, pixel.position);
        assert!((retrieved.color.l - pixel.color.l).abs() < 0.001);
        assert_eq!(retrieved.timestamp, pixel.timestamp);
    }
    
    #[test]
    fn test_insert_duplicate_pixel_idempotent() {
        let (db, _temp) = setup_temp_db();
        
        let pixel = Pixel {
            position: PixelCoord { x: 5, y: 5 },
            color: LAB { l: 30.0, a: 0.0, b: 0.0 },
            timestamp: 100,
        };
        
        // Insertar dos veces
        let id1 = db.insert_pixel(&pixel).unwrap();
        let id2 = db.insert_pixel(&pixel).unwrap();
        
        // Debe retornar mismo ID (content-addressable)
        assert_eq!(id1, id2);
        
        // Debe haber solo 1 entrada
        assert_eq!(db.count_pixels().unwrap(), 1);
    }
    
    #[test]
    fn test_query_pixels_in_region() {
        let (db, _temp) = setup_temp_db();
        
        // Insertar grid 10x10
        for y in 0..10 {
            for x in 0..10 {
                db.insert_pixel(&Pixel {
                    position: PixelCoord { x, y },
                    color: LAB { l: 50.0, a: 0.0, b: 0.0 },
                    timestamp: 0,
                }).unwrap();
            }
        }
        
        // Query región 2x2
        let region = BoundingBox {
            min: PixelCoord { x: 3, y: 3 },
            max: PixelCoord { x: 5, y: 5 },
        };
        
        let pixels = db.query_region(&region).unwrap();
        
        // Debe retornar 4 píxeles (3-4, 3-4) inclusive
        assert_eq!(pixels.len(), 4);
    }
    
    #[test]
    #[should_panic(expected = "invalid color value")]
    fn test_insert_invalid_color_panics() {
        let (db, _temp) = setup_temp_db();
        
        let invalid_pixel = Pixel {
            position: PixelCoord { x: 0, y: 0 },
            color: LAB { l: f32::NAN, a: 0.0, b: 0.0 },
            timestamp: 0,
        };
        
        db.insert_pixel(&invalid_pixel).unwrap(); // Debe paniquear
    }
    
    #[test]
    fn test_delete_pixel() {
        let (db, _temp) = setup_temp_db();
        
        let pixel = Pixel {
            position: PixelCoord { x: 1, y: 1 },
            color: LAB { l: 40.0, a: 0.0, b: 0.0 },
            timestamp: 0,
        };
        
        let id = db.insert_pixel(&pixel).unwrap();
        assert!(db.get_pixel(id).is_ok());
        
        db.delete_pixel(id).unwrap();
        
        assert!(matches!(db.get_pixel(id), Err(Error::PixelNotFound)));
    }
}
```

---

## 🎯 COMPONENTES CRÍTICOS: TESTS OBLIGATORIOS

### **1. TelescopeDB Tests**

```rust
// src/cells/telescopedb.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spherical_coordinate_conversion() {
        let cartesian = PixelCoord { x: 100, y: 200 };
        let spherical = cartesian.to_spherical();
        
        // r debe ser > 0
        assert!(spherical.r > 0.0);
        
        // θ debe estar en [0, 2π]
        assert!(spherical.theta >= 0.0 && spherical.theta <= 2.0 * PI);
        
        // φ debe estar en [0, π]
        assert!(spherical.phi >= 0.0 && spherical.phi <= PI);
        
        // Round-trip debe recuperar original
        let recovered = spherical.to_cartesian();
        assert_eq!(recovered.x, cartesian.x);
        assert_eq!(recovered.y, cartesian.y);
    }
    
    #[test]
    fn test_concurrent_inserts() {
        use std::sync::Arc;
        use std::thread;
        
        let (db, _temp) = setup_temp_db();
        let db = Arc::new(db);
        
        let mut handles = vec![];
        
        for i in 0..10 {
            let db_clone = Arc::clone(&db);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    let pixel = Pixel {
                        position: PixelCoord { x: i, y: j },
                        color: LAB { l: 50.0, a: 0.0, b: 0.0 },
                        timestamp: (i * 100 + j) as u64,
                    };
                    db_clone.insert_pixel(&pixel).unwrap();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Debe haber 1000 píxeles (10 threads × 100 pixels)
        assert_eq!(db.count_pixels().unwrap(), 1000);
    }
}
```

---

### **2. VoxelDB Tests**

```rust
// src/cells/voxeldb.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_octree_insertion_and_retrieval() {
        let voxeldb = VoxelDB::new_in_memory().unwrap();
        
        let coord = OctreeCoord { x: 128, y: 64, z: 32 };
        let data = vec![1, 2, 3, 4, 5];
        
        voxeldb.insert(coord, data.clone()).unwrap();
        
        let retrieved = voxeldb.get(coord).unwrap();
        assert_eq!(retrieved, data);
    }
    
    #[test]
    fn test_octree_space_partitioning() {
        let voxeldb = VoxelDB::new_in_memory().unwrap();
        
        // Insertar en diferentes octantes
        let coords = vec![
            OctreeCoord { x: 0, y: 0, z: 0 },     // Octante 0
            OctreeCoord { x: 255, y: 0, z: 0 },   // Octante 1
            OctreeCoord { x: 0, y: 255, z: 0 },   // Octante 2
            OctreeCoord { x: 255, y: 255, z: 255 }, // Octante 7
        ];
        
        for (i, coord) in coords.iter().enumerate() {
            voxeldb.insert(*coord, vec![i as u8]).unwrap();
        }
        
        // Verificar que cada uno está en su octante
        for (i, coord) in coords.iter().enumerate() {
            let octant = voxeldb.get_octant(*coord);
            assert_eq!(voxeldb.get(*coord).unwrap(), vec![i as u8]);
        }
    }
    
    #[test]
    fn test_knn_search() {
        let voxeldb = VoxelDB::new_in_memory().unwrap();
        
        // Insertar 100 puntos aleatorios
        for i in 0..100 {
            let coord = OctreeCoord {
                x: (i * 17) % 256,
                y: (i * 31) % 256,
                z: (i * 47) % 256,
            };
            voxeldb.insert(coord, vec![i]).unwrap();
        }
        
        // Buscar 5 vecinos más cercanos a (128, 128, 128)
        let query = OctreeCoord { x: 128, y: 128, z: 128 };
        let neighbors = voxeldb.knn_search(query, 5).unwrap();
        
        assert_eq!(neighbors.len(), 5);
        
        // Verificar que están ordenados por distancia
        for i in 1..neighbors.len() {
            let dist_prev = neighbors[i - 1].distance;
            let dist_curr = neighbors[i].distance;
            assert!(dist_prev <= dist_curr);
        }
    }
}
```

---

### **3. FBCU Core Tests**

```rust
// src/cells/fbcu_core.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fbcu_compression_lossless() {
        let block = create_solid_block(LAB { l: 50.0, a: 10.0, b: -5.0 });
        
        let config = QuantizationConfig { quality: 1.0 }; // Lossless
        let tree = FBCUTree::build_from_block(&block);
        let compressed = tree.serialize(&config);
        
        // Reconstruir
        let deserialized = SerializedFBCU::deserialize(&compressed).unwrap();
        let decoder = HuffmanEncoder::new();
        let reconstructed = deserialized.rebuild_tree(&decoder);
        
        // Verificar que es idéntico
        for (orig, recon) in block.pixels.iter().zip(reconstructed.levels.last().unwrap().nodes.iter()) {
            assert!((orig.color.l - recon.l).abs() < 0.01);
            assert!((orig.color.a - recon.a).abs() < 0.01);
            assert!((orig.color.b - recon.b).abs() < 0.01);
        }
    }
    
    #[test]
    fn test_fbcu_compression_ratio() {
        let block = create_gradient_block();
        
        let config = QuantizationConfig { quality: 0.95 };
        let tree = FBCUTree::build_from_block(&block);
        let compressed = tree.serialize(&config);
        
        let original_size = block.pixels.len() * std::mem::size_of::<LAB>();
        let compressed_size = compressed.len();
        
        let ratio = original_size as f64 / compressed_size as f64;
        
        tracing::info!("Compression ratio: {:.2}:1", ratio);
        assert!(ratio >= 2.0); // Mínimo 2:1 para gradientes
    }
    
    #[test]
    fn test_fbcu_homogeneous_block_skip() {
        let block = create_solid_block(LAB { l: 50.0, a: 0.0, b: 0.0 });
        let stats = block.compute_statistics();
        
        // Bloque homogéneo → no comprimir con FBCU completo
        assert!(stats.homogeneity > 0.95);
        assert!(!should_compress_block(&stats));
    }
    
    fn create_solid_block(color: LAB) -> PixelBlock {
        let pixels = (0..64).map(|i| Pixel {
            position: PixelCoord { x: i % 8, y: i / 8 },
            color,
            timestamp: 0,
        }).collect();
        
        PixelBlock {
            origin: PixelCoord { x: 0, y: 0 },
            size: 8,
            pixels,
        }
    }
    
    fn create_gradient_block() -> PixelBlock {
        let pixels = (0..64).map(|i| {
            let x = i % 8;
            let y = i / 8;
            let t = (x + y) as f32 / 14.0;
            
            Pixel {
                position: PixelCoord { x, y },
                color: LAB {
                    l: 30.0 + t * 40.0,
                    a: -20.0 + t * 40.0,
                    b: 20.0 - t * 40.0,
                },
                timestamp: 0,
            }
        }).collect();
        
        PixelBlock {
            origin: PixelCoord { x: 0, y: 0 },
            size: 8,
            pixels,
        }
    }
}
```

---

### **4. Context Token 7D Tests**

```rust
// src/cells/context_token_7d.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ctx7d_creation_from_text() {
        let text = "Debugging ownership issues in Rust";
        let ctx7d = ContextToken7D::from_text(text).unwrap();
        
        // Tensor debe tener 7 dimensiones
        assert!(ctx7d.tensor.semantic > 0.0);
        assert!(ctx7d.tensor.temporal >= 0.0 && ctx7d.tensor.temporal <= 1.0);
        assert!(ctx7d.tensor.spatial >= 0.0);
        
        // Metadata debe contener el texto
        assert_eq!(ctx7d.metadata.original_text, text);
    }
    
    #[test]
    fn test_tensor_normalization() {
        let mut tensor = Tensor7D {
            semantic: 150.0,  // Fuera de rango
            temporal: -0.5,   // Negativo
            spatial: 0.5,
            harmonic: 0.8,
            resonant: 0.3,
            emergent: 0.9,
            void_potential: 0.1,
        };
        
        tensor.normalize();
        
        // Todas las dimensiones deben estar en [0, 1]
        assert!(tensor.semantic >= 0.0 && tensor.semantic <= 1.0);
        assert!(tensor.temporal >= 0.0 && tensor.temporal <= 1.0);
        assert!(tensor.spatial >= 0.0 && tensor.spatial <= 1.0);
    }
    
    #[test]
    fn test_tensor_distance() {
        let t1 = Tensor7D {
            semantic: 0.5,
            temporal: 0.5,
            spatial: 0.5,
            harmonic: 0.5,
            resonant: 0.5,
            emergent: 0.5,
            void_potential: 0.5,
        };
        
        let t2 = Tensor7D {
            semantic: 0.6,
            temporal: 0.6,
            spatial: 0.6,
            harmonic: 0.6,
            resonant: 0.6,
            emergent: 0.6,
            void_potential: 0.6,
        };
        
        let distance = t1.distance_to(&t2);
        
        // Distancia euclidiana en 7D
        let expected = ((0.1_f64.powi(2) * 7.0).sqrt());
        assert!((distance - expected).abs() < 0.001);
    }
}
```

---

### **5. Sensory Engine Tests**

```rust
// src/cells/sensory_engine.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_processing() {
        let sensory = SensoryEngine::new();
        let input = UserInput::Text("Hello world".to_string());
        
        let normalized = sensory.process(input).unwrap();
        
        assert_eq!(normalized.modality, Modality::Text);
        assert!(normalized.tokens.len() > 0);
    }
    
    #[test]
    fn test_audio_transcription() {
        let sensory = SensoryEngine::new();
        let audio_bytes = load_test_audio(); // WAV mock
        let input = UserInput::Audio(audio_bytes);
        
        let normalized = sensory.process(input).unwrap();
        
        assert_eq!(normalized.modality, Modality::Audio);
        assert!(!normalized.transcription.unwrap().is_empty());
    }
    
    #[test]
    fn test_multimodal_processing() {
        let sensory = SensoryEngine::new();
        let input = UserInput::Multimodal {
            text: Some("Look at this image".to_string()),
            image: Some(load_test_image()),
            audio: None,
        };
        
        let normalized = sensory.process(input).unwrap();
        
        assert_eq!(normalized.modality, Modality::Multimodal);
        assert!(normalized.image_features.is_some());
    }
}
```

---

## 🧮 PROPERTY-BASED TESTING

Para casos donde hay infinitas combinaciones de inputs:

```rust
use proptest::prelude::*;

#[cfg(test)]
mod proptests {
    use super::*;
    
    proptest! {
        #[test]
        fn test_pixel_serialization_roundtrip(
            x in 0u32..1920,
            y in 0u32..1080,
            l in 0.0f32..100.0,
            a in -128.0f32..127.0,
            b in -128.0f32..127.0,
        ) {
            let pixel = Pixel {
                position: PixelCoord { x, y },
                color: LAB { l, a, b },
                timestamp: 12345,
            };
            
            let serialized = serde_cbor::to_vec(&pixel).unwrap();
            let deserialized: Pixel = serde_cbor::from_slice(&serialized).unwrap();
            
            prop_assert_eq!(deserialized.position, pixel.position);
            prop_assert!((deserialized.color.l - pixel.color.l).abs() < 0.01);
        }
        
        #[test]
        fn test_octree_always_finds_inserted(
            x in 0usize..256,
            y in 0usize..256,
            z in 0usize..256,
            data in prop::collection::vec(any::<u8>(), 1..100),
        ) {
            let voxeldb = VoxelDB::new_in_memory().unwrap();
            let coord = OctreeCoord { x, y, z };
            
            voxeldb.insert(coord, data.clone()).unwrap();
            let retrieved = voxeldb.get(coord).unwrap();
            
            prop_assert_eq!(retrieved, data);
        }
    }
}
```

---

## 📊 COVERAGE METRICS

### **Target de Cobertura**

```yaml
Components:
  CRITICOS:
    min_coverage: 90%  # TelescopeDB, VoxelDB, FBCU, Sensory, CTX7D
    
  IMPORTANTES:
    min_coverage: 80%  # HubSpoke, MTT-DSL, Expertise, Routier
    
  UTILITIES:
    min_coverage: 70%  # Utils, helpers
```

### **Ejecutar con Coverage**

```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Ejecutar tests con coverage
cargo tarpaulin --out Html --output-dir coverage/

# Ver reporte
firefox coverage/index.html
```

---

## 🧪 COMANDOS ÚTILES

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests de un módulo específico
cargo test telescopedb::tests

# Ejecutar un test específico
cargo test test_insert_and_retrieve_pixel

# Ver output de println! en tests
cargo test -- --nocapture

# Ejecutar tests en paralelo con límite
cargo test -- --test-threads=4

# Ejecutar solo tests que fallen
cargo test -- --failed

# Ejecutar property tests con más casos
PROPTEST_CASES=10000 cargo test
```

---

## 📚 REFERENCIAS

- **Rust Book - Testing:** https://doc.rust-lang.org/book/ch11-00-testing.html
- **Proptest Guide:** https://github.com/proptest-rs/proptest
- **DA-024:** Testing requirements (90% coverage críticos)
- **BREAKTHROUGH_DETECTION.md:** Testing score contribuye 20/100 pts

---

**Estado:** 📋 Especificación completa  
**Criticidad:** 🔴 ALTA - Base de calidad del proyecto  
**Próxima implementación:** Durante Fase 1 (paralelo a desarrollo)

---

*Generado: 2025-10-26*  
*Sistema Bitácora v1.0 - Guía de Testing*  
*"Test what matters, test it well"* 🧪
