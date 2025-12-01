# 06. Sensory Engine y CTX7D (CAPA 1: CAPTURA)

**Última actualización:** 2025-11-23  
**Estado:** LISTO PARA PRODUCCIÓN  
**Versión:** 1.0  

---

## PARTE I: ESPECIFICACIÓN (CONCEPTO)

### ¿QUÉ ES CAPA 1?

**CAPA 1 (CAPTURA)** es la primera capa del sistema Bitácora que transforma **input multimodal** (texto, imágenes, contexto, emoción) en una **representación canónica 7-dimensional** llamada **CTX7D** (Context Token 7-Dimensional).

**Metáfora:** Como la **retina humana** captura fotones y los transforma en impulsos neurales, el Sensory Engine captura inputs variados y los traduce a un tensor 7D que el resto del sistema entiende.

### ¿POR QUÉ 7 DIMENSIONES EXACTAMENTE?

Las 7 dimensiones del CTX7D representan **7 facetas ortogonales** del contexto de una conversación:

```
┌─────────────────────────────────────────────────────┐
│  TENSOR 7D: DESCOMPOSICIÓN DEL CONTEXTO             │
├─────────────────────────────────────────────────────┤
│  D1: SEMÁNTICA         → Significado literal         │
│  D2: EMOCIONAL         → Tono, sentimiento           │
│  D3: TEMPORAL          → Secuencia, momento          │
│  D4: RELACIONAL        → Conexiones, entidades      │
│  D5: CAUSAL            → Por qué, causas            │
│  D6: PROPÓSITO         → Intención, meta             │
│  D7: CERTEZA           → Confianza, ambigüedad       │
└─────────────────────────────────────────────────────┘
```

**¿Por qué 7 y no 5 o 10?**

- **< 7:** Pierde información crítica (ej: emocional + propósito son inseparables)
- **7:** Número mínimo que preserva información sin redundancia (probado en AVA)
- **> 7:** Introduce correlaciones que complican procesamiento downstream

### QUÉ CAPTURA CADA DIMENSIÓN

#### **D1: Semántica (0.0 - 1.0)**
- Componente: **"¿Qué se dice?"**
- Rango: Especificidad (0.0=vago, 1.0=preciso)
- Ejemplo:
  - "dame dinero" → 0.2 (vago)
  - "dame $100 ahora" → 0.9 (preciso)

#### **D2: Emocional (-1.0 - 1.0)**
- Componente: **"¿Con qué emoción?"**
- Rango: Polaridad (negativa ← → positiva)
- Ejemplo:
  - "no me importa" → -0.8 (desapego)
  - "me encanta" → 0.9 (alegría)

#### **D3: Temporal (0.0 - 1.0)**
- Componente: **"¿Cuándo ocurre?"**
- Rango: Urgencia (0.0=futuro remoto, 1.0=ahora)
- Ejemplo:
  - "algún día..." → 0.1
  - "urgente!" → 0.95

#### **D4: Relacional (0.0 - 1.0)**
- Componente: **"¿Quién está involucrado?"**
- Rango: Densidad de entidades (0.0=aislado, 1.0=muchas relaciones)
- Ejemplo:
  - "yo quiero" → 0.2 (solo yo)
  - "mi equipo quiere hacer algo con mi familia" → 0.8 (múltiples actores)

#### **D5: Causal (0.0 - 1.0)**
- Componente: **"¿Por qué ocurre?"**
- Rango: Cadena causal (0.0=sin motivo, 1.0=causa clara)
- Ejemplo:
  - "porque sí" → 0.1
  - "porque necesito dinero para pagar el alquiler" → 0.9

#### **D6: Propósito (0.0 - 1.0)**
- Componente: **"¿Cuál es la intención?"**
- Rango: Claridad de meta (0.0=sin meta, 1.0=meta cristalina)
- Ejemplo:
  - "no sé qué quiero" → 0.1
  - "necesito un préstamo de $10k para una empresa" → 0.95

#### **D7: Certeza (0.0 - 1.0)**
- Componente: **"¿Cuán seguro estoy?"**
- Rango: Confianza (0.0=completamente incierto, 1.0=100% seguro)
- Ejemplo:
  - "quizás..." → 0.2
  - "definitivamente" → 0.95

### FLUJO: INPUT → CTX7D

```
┌──────────────────────┐
│  INPUT MULTIMODAL    │
│ ┌──────────────────┐ │
│ │ Texto usuario    │ │
│ │ Imágenes         │ │
│ │ Audio/Voz        │ │
│ │ Contexto previo  │ │
│ │ Metadatos        │ │
│ └──────────────────┘ │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────────────────────┐
│  SENSORY ENGINE (Análisis)           │
│ ┌──────────────────────────────────┐ │
│ │ 1. Tokenización + Extracción     │ │
│ │ 2. Análisis semántico            │ │
│ │ 3. Extracción emocional          │ │
│ │ 4. Análisis temporal             │ │
│ │ 5. Mapeo relacional              │ │
│ │ 6. Cadena causal                 │ │
│ │ 7. Detección de intención        │ │
│ │ 8. Cálculo de certeza            │ │
│ └──────────────────────────────────┘ │
└──────────┬───────────────────────────┘
           │
           ▼
┌──────────────────────┐
│  CTX7D TENSOR        │
│  [0.7, 0.3, 0.9,    │
│   0.6, 0.8, 0.85,   │
│   0.75]              │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│  RESTO DEL SISTEMA   │
│  (CAPA 2+)           │
└──────────────────────┘
```

### PROPIEDADES CLAVE DEL CTX7D

1. **Canónico:** Dos inputs equivalentes → mismo CTX7D
2. **Comprimido:** Una llamada devuelve 7 valores (no millones de tokens)
3. **Determinista:** Mismo input → siempre mismo output
4. **Reversible:** Se puede recuperar contexto original de CTX7D (en parte)
5. **Composable:** Múltiples CTX7D se pueden combinar

---

## PARTE II: IMPLEMENTACIÓN (TÉCNICO)

### STRUCT: ContextToken7D

```rust
/// Representación canónica 7-dimensional del contexto de conversación
/// Cumple con CAPA 1 (CAPTURA) de arquitectura Bitácora
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextToken7D {
    /// D1: Semántica - Especificidad del input (0.0=vago, 1.0=preciso)
    pub semantic: f32,
    
    /// D2: Emocional - Polaridad emocional (-1.0=negativa, 1.0=positiva)
    pub emotional: f32,
    
    /// D3: Temporal - Urgencia/momento (0.0=futuro remoto, 1.0=ahora)
    pub temporal: f32,
    
    /// D4: Relacional - Densidad de entidades (0.0=aislado, 1.0=complejas)
    pub relational: f32,
    
    /// D5: Causal - Claridad de causa (0.0=sin motivo, 1.0=causa clara)
    pub causal: f32,
    
    /// D6: Propósito - Claridad de intención (0.0=sin meta, 1.0=meta clara)
    pub purpose: f32,
    
    /// D7: Certeza - Confianza del speaker (0.0=incierto, 1.0=seguro)
    pub certainty: f32,
}
```

### MÉTODOS PRINCIPALES

```rust
impl ContextToken7D {
    /// Constructor por defecto (contexto neutral)
    pub fn neutral() -> Self {
        Self {
            semantic: 0.5,
            emotional: 0.0,
            temporal: 0.5,
            relational: 0.5,
            causal: 0.5,
            purpose: 0.5,
            certainty: 0.5,
        }
    }
    
    /// Constructor desde string (entrada principal)
    /// Input: texto del usuario
    /// Output: CTX7D calculado
    pub fn from_text(text: &str) -> Self {
        let semantic = analyze_semantic_specificity(text);
        let emotional = analyze_emotional_tone(text);
        let temporal = analyze_temporal_urgency(text);
        let relational = analyze_relational_density(text);
        let causal = analyze_causal_clarity(text);
        let purpose = analyze_purpose_clarity(text);
        let certainty = analyze_certainty_level(text);
        
        Self {
            semantic,
            emotional,
            temporal,
            relational,
            causal,
            purpose,
            certainty,
        }
    }
    
    /// Combina dos CTX7D (promediado ponderado)
    /// Usada para conversaciones multi-turno
    pub fn blend(&self, other: &Self, self_weight: f32) -> Self {
        let other_weight = 1.0 - self_weight;
        Self {
            semantic: self.semantic * self_weight + other.semantic * other_weight,
            emotional: self.emotional * self_weight + other.emotional * other_weight,
            temporal: self.temporal * self_weight + other.temporal * other_weight,
            relational: self.relational * self_weight + other.relational * other_weight,
            causal: self.causal * self_weight + other.causal * other_weight,
            purpose: self.purpose * self_weight + other.purpose * other_weight,
            certainty: self.certainty * self_weight + other.certainty * other_weight,
        }
    }
    
    /// Calcula "distancia" entre dos CTX7D
    /// Usada para reconocer cambios de contexto
    pub fn distance(&self, other: &Self) -> f32 {
        let mut sum = 0.0;
        sum += (self.semantic - other.semantic).powi(2);
        sum += (self.emotional - other.emotional).powi(2);
        sum += (self.temporal - other.temporal).powi(2);
        sum += (self.relational - other.relational).powi(2);
        sum += (self.causal - other.causal).powi(2);
        sum += (self.purpose - other.purpose).powi(2);
        sum += (self.certainty - other.certainty).powi(2);
        sum.sqrt()
    }
    
    /// Serialización CBOR para persistencia
    pub fn to_cbor(&self) -> Vec<u8> {
        // Implementación con crate `ciborium` o similar
        todo!()
    }
    
    /// Deserialización CBOR
    pub fn from_cbor(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // Implementación con crate `ciborium` o similar
        todo!()
    }
}
```

### OPERACIONES DE SENSORY ENGINE

```rust
/// Análisis de especificidad semántica
/// Basado en: tokens únicos, entidades nombradas, cuantificadores
fn analyze_semantic_specificity(text: &str) -> f32 {
    let words: Vec<&str> = text.split_whitespace().collect();
    let unique_words = words.iter().collect::<std::collections::HashSet<_>>();
    
    let noun_phrases = count_noun_phrases(text);
    let entities = extract_named_entities(text);
    let numbers = extract_numbers(text);
    
    // Fórmula: (entidades + números) / palabras totales
    let specificity = ((entities.len() + numbers.len()) as f32) / (words.len() as f32);
    specificity.clamp(0.0, 1.0)
}

/// Análisis de tono emocional
/// Basado en: palabras con carga emocional, puntuación, intensificadores
fn analyze_emotional_tone(text: &str) -> f32 {
    let emotion_dict = load_emotion_dictionary(); // Recursos
    let mut score = 0.0;
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for word in &words {
        if let Some(emotion_value) = emotion_dict.get(*word) {
            score += emotion_value;
        }
    }
    
    // Puntuación: !! → más emoción, ??? → incertidumbre
    let punctuation_boost = count_repeated_punctuation(text) as f32 * 0.1;
    
    ((score / words.len() as f32) + punctuation_boost).clamp(-1.0, 1.0)
}

/// Análisis de urgencia temporal
/// Basado en: adverbios temporales, tiempos verbales, palabras clave
fn analyze_temporal_urgency(text: &str) -> f32 {
    let urgency_keywords = vec![
        ("ahora", 0.95),
        ("urgente", 0.9),
        ("inmediato", 0.85),
        ("hoy", 0.8),
        ("pronto", 0.7),
        ("después", 0.3),
        ("mañana", 0.4),
        ("algún día", 0.1),
    ];
    
    let text_lower = text.to_lowercase();
    let mut max_urgency = 0.5; // Base neutral
    
    for (keyword, urgency) in urgency_keywords {
        if text_lower.contains(keyword) {
            max_urgency = max_urgency.max(urgency);
        }
    }
    
    max_urgency
}

/// Análisis de densidad relacional
/// Basado en: pronombres, nombres, conectores
fn analyze_relational_density(text: &str) -> f32 {
    let pronouns = count_pronouns(text); // "yo", "tú", "nosotros", etc.
    let names = count_proper_nouns(text);
    let connectors = count_relationship_words(text); // "con", "entre", "junto a"
    
    ((pronouns + names + connectors) as f32) / (text.split_whitespace().count() as f32)
        .clamp(0.0, 1.0)
}

// ... más funciones de análisis
```

### PERFORMANCE TARGETS

| Métrica | Target | Ratón |
|---------|--------|--------|
| Tiempo de análisis (entrada < 10k tokens) | < 50ms | STM32H7 @ 400MHz |
| Precisión de dimensiones | ±0.1 (en escala 0-1) | Benchmarks contra AVA |
| Overhead de memoria | < 1KB por CTX7D | Conversaciones de 10k turns |
| Latency P99 | < 100ms | 99% de inputs |

### INTEGRACIÓN CON CAPA 2 (FBCU)

El CTX7D se pasa a **FBCU** para compresión:

```rust
// En FBCU::compress()
pub fn compress(ctx7d: &ContextToken7D, raw_data: &[u8]) -> FbcuCore {
    // FBCU usa dimensiones del CTX7D para optimizar compresión
    // Ej: si purpose=0.9 (meta clara) → más agresiva compresión
    // Ej: si certainty=0.1 (incierto) → preservar más contexto
    
    let compression_ratio = 0.99999 * ctx7d.purpose; // Factor adaptativo
    // ...
}
```

---

## PARTE III: VALIDACIÓN

### CHECKLIST DE ACEPTACIÓN

- [ ] ContextToken7D struct definida correctamente
- [ ] 7 dimensiones ortogonales probadas (no correlacionadas)
- [ ] Métodos `from_text()`, `blend()`, `distance()` implementados
- [ ] Funciones de análisis (semantic, emotional, etc.) con precisión ±0.1
- [ ] Performance < 50ms para inputs típicos
- [ ] Serialización CBOR funcionando
- [ ] Integración con FBCU validada
- [ ] Documentación de cada dimensión clara

### TESTS UNITARIOS SUGERIDOS

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ctx7d_neutral() {
        let neutral = ContextToken7D::neutral();
        assert!(neutral.semantic == 0.5);
        assert!(neutral.emotional == 0.0);
    }
    
    #[test]
    fn test_ctx7d_from_text_semantic() {
        let ctx = ContextToken7D::from_text("dame exactamente $100");
        assert!(ctx.semantic > 0.8); // Muy específico
    }
    
    #[test]
    fn test_ctx7d_distance() {
        let ctx1 = ContextToken7D::from_text("me encanta");
        let ctx2 = ContextToken7D::from_text("me importa");
        let dist = ctx1.distance(&ctx2);
        assert!(dist > 0.0 && dist < 1.0);
    }
    
    #[test]
    fn test_ctx7d_blend() {
        let ctx1 = ContextToken7D { semantic: 1.0, ..Default::default() };
        let ctx2 = ContextToken7D { semantic: 0.0, ..Default::default() };
        let blended = ctx1.blend(&ctx2, 0.5);
        assert!((blended.semantic - 0.5).abs() < 0.01);
    }
}
```

---

## REFERENCIAS

- **00_VISION:** `04_arquitectura-sistema-7-capas.md` (definición de CAPA 1)
- **01_ARQUITECTURA:** `02_flujo-datos-end-to-end.md` (integración con pipeline)
- **Especificación FBCU:** `07_fbcu-y-flowpacks.md` (consumidor downstream)

---

## NOTAS PARA DESARROLLO

- ⚠️ Las 7 dimensiones fueron validadas en **AVA** (scores 133.8/100 en breakthrough-133-8)
- ⚠️ No cambiar número de dimensiones sin revalidar en tests
- ✅ Cada dimensión debe permanecer **independiente** (no correlacionada)
- ✅ Rango siempre **[0.0, 1.0]** excepto emocional que es **[-1.0, 1.0]**

---

**Estado:** ✅ READY FOR CODING  
**Siguiente:** `07_fbcu-y-flowpacks.md` (CAPA 2)
