# 🜛 PXLang & Bitácora  
## Especificación Técnica para Implementación en Rust  
### (Guía para agentes de codificación y arquitectos de sistemas)

**Documento:** PXLang_Bitacora_Technical_Design_Rust  
**Propósito:** Explicar cómo implementar el sistema a nivel técnico, sin escribir el programa completo.  
**Lenguaje objetivo:** Rust (núcleo de Bitácora)  

---

## 1. Objetivo técnico general

Implementar en Rust un **motor de representación y compresión simbólica** llamado PXLang, que permita:

1. Representar recuerdos/escenas como secuencias de símbolos compactos.  
2. Asociar estos símbolos con datos ricos almacenados en otras capas (TelescopeDB, VoxelDB).  
3. Serializar y deserializar estas estructuras de forma eficiente (1–3 bytes por unidad simbólica cuando sea posible).  
4. Permitir a agentes de IA codificar y decodificar recuerdos desde/hacia PXLang de manera controlada.  
5. Mantener extensibilidad, versionado y compatibilidad futura.

No se implementa aún el “modelo de IA”; se define la **infraestructura de datos y APIs** para que un agente de codificación (humano o IA) pueda construir el motor.

---

## 2. Modelo conceptual en capas

A nivel de arquitectura, podemos imaginar PXLang dentro de Bitácora como tres capas:

1. **Capa de Dominio Simbólico (PX Domain Layer)**  
   - Tipos básicos: `PXToken`, `PXSequence`, `PXArc`, etc.
   - Enums para emoción, acción, contexto, temporalidad, objetividad.

2. **Capa de Compresión / Codificación (PX Codec Layer)**  
   - Mapeo entre estructuras de dominio y representaciones compactas (bytes).  
   - Estrategias de compresión (tabla estática, tabla adaptativa, delta-encoding).  
   - Serialización/Deserialización (binaria, opcionalmente base64/hex para transporte).

3. **Capa de Integración con Bitácora (PX Integration Layer)**  
   - Enlaces con TelescopeDB (biografía rica).  
   - Enlaces con VoxelDB (patrones y plantillas).  
   - API pública para otros módulos (agentes de IA, front-end, export/import).

---

## 3. Diseño de tipos básicos en Rust (Dominio Simbólico)

### 3.1. Identificadores y versiones

Se recomienda definir:

```rust
/// Versión del esquema PXLang
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PXVersion {
    pub major: u8,
    pub minor: u8,
}

/// Identificador de un símbolo PX en tablas internas
pub type PXId = u16; // Permite hasta 65.536 símbolos distintos
```

La versión permite evolucionar el lenguaje sin romper compatibilidad.

---

### 3.2. Niveles de objetividad

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectivityLevel {
    Internal = 0,          // Sueño / imaginación
    HighlySubjective = 1,  // Recuerdo muy personal
    StablePersonal = 2,    // Memoria que el usuario considera “real”
    Intersubjective = 3,   // Coincide con otros / hay ecos externos
    Documented = 4,        // Hay registros objetivos
}
```

---

### 3.3. Emociones (núcleo PX-Core)

Primera versión, extensible. Se puede comenzar con un set reducido de emociones básicas (Plutchik / Ekman simplificado).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emotion {
    Joy,
    Sadness,
    Fear,
    Anger,
    Surprise,
    Disgust,
    Trust,
    Anticipation,

    // Estados compuestos o especiales:
    Hope,
    Despair,
    Calm,
    Overwhelm,
    Gratitude,
    Love,
    Empty,
}
```

Esta enum **no tiene por qué ser definitiva**. Puede mapearse más tarde a tablas configurables.

---

### 3.4. Acciones / modos de experiencia

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    /// Movimiento físico o simbólico
    Move,
    /// Decisión interna
    Decide,
    /// Hablar / comunicar
    Speak,
    /// Recibir información
    Perceive,
    /// Crear / producir
    Create,
    /// Romper / terminar
    Break,
    /// Cuidar / proteger
    Care,
    /// Explorar / investigar
    Explore,
    /// Esperar / pausar
    Wait,
    /// Recordar / revisar pasado
    Recall,
    /// Proyectar futuro
    ImagineFuture,
}
```

---

### 3.5. Contexto (muy simplificado)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    PhysicalEnvironment,
    Social,
    InnerWorld,
    Work,
    Family,
    Couple,
    Health,
    Spiritual,
    Learning,
    Play,
    Unknown,
}
```

En implementaciones avanzadas, esto puede sustituirse con IDs dinámicos provenientes de VoxelDB.

---

### 3.6. Tipo de unidad temporal

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalScope {
    Instant,   // Momento concreto, frame
    Short,     // Minutos / horas
    Episode,   // Un evento (una tarde, una reunión)
    Phase,     // Meses / años (capítulo vital)
    Lifetime,  // Reflexión de toda una vida
}
```

---

## 4. Representación de un token PX

Un `PXToken` es la unidad mínima en PXLang.  
Puede contener varios campos lógicos, pero no todos son obligatorios en la representación final.

### 4.1. Estructura de dominio

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PXToken {
    pub id: PXId,                    // ID interno opcional
    pub emotion: Option<Emotion>,    // Emoción principal (si aplica)
    pub action: Option<ActionKind>,  // Acción o modo
    pub context: Option<ContextKind>,
    pub objectivity: Option<ObjectivityLevel>,
    pub temporal: Option<TemporalScope>,

    /// Campo extensible para asociar este token con un emoji/ideograma Unicode concreto,
    /// útil para interfaces de usuario y agentes de IA.
    pub visual_hint: Option<String>, // Ej: "😔", "🌅", etc.

    /// Campo para enlazar este token con una entidad o nodo de TelescopeDB/VoxelDB
    pub ref_id: Option<String>,      // ej: "person:mother", "place:beach_x"
}
```

Importante:  
- A nivel de dominio, esta estructura puede ser **rica**.  
- A nivel de compresión binaria, se usan sólo los campos requeridos y se mapean a códigos compactos.

---

## 5. Secuencias, escenas y arcos

### 5.1. Frame simbólico (PXFrame)

Un “frame” es una pequeña unidad de recuerdo claro.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PXFrame {
    pub tokens: Vec<PXToken>,
    pub timestamp: Option<i64>,      // Epoch millis o similar (opcional)
}
```

---

### 5.2. Escena (PXScene)

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PXScene {
    pub id: String,
    pub frames: Vec<PXFrame>,

    /// Nivel de objetividad predominante de la escena
    pub objectivity: ObjectivityLevel,

    /// Etiquetas temáticas (ej: "mudanza", "discusión", "descubrimiento")
    pub tags: Vec<String>,

    /// Referencias cruzadas a TelescopeDB (texto, audio, etc.)
    pub telescope_refs: Vec<String>,

    /// Notas opcionales generadas por IA o por el usuario
    pub notes: Vec<String>,
}
```

---

### 5.3. Arco (PXArc)

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PXArc {
    pub id: String,
    pub title: String,         // Ej: "Años de universidad", "Tiempo en Holanda"
    pub scenes: Vec<PXScene>,

    /// Alcance temporal principal de este arco
    pub temporal_scope: TemporalScope,

    /// Foco emocional dominante del arco (si se puede sintetizar)
    pub dominant_emotion: Option<Emotion>,

    /// Nivel de objetividad aproximado del arco
    pub objectivity: ObjectivityLevel,
}
```

Una biografía simbólica sería un `Vec<PXArc>`.

---

## 6. Capa de Compresión (PX Codec Layer)

El objetivo del codec es transformar:

- estructuras de dominio (`PXToken`, `PXFrame`, `PXScene`)  
en
- representaciones compactas (bytes, cadenas densas, etc.).

### 6.1. Tabla de símbolos base (PX-Core-64)

Se sugiere definir una tabla **compacta** de símbolos base, que luego puede ampliarse.

Ejemplo conceptual (no definitivo):

```rust
pub struct PXSymbolDef {
    pub code: u8,           // 0..=255
    pub description: &'static str,
    pub default_visual: &'static str, // emoji/ideograma
}
```

Algunos ejemplos:

- 0x01 → tristeza básica (`"sad"`, `"😔"`)
- 0x02 → alegría (`"joy"`, `"😊"`)
- 0x10 → caminar / avanzar (`"move"`, `"🚶‍♂️"`)
- 0x20 → lluvia / dificultad (`"rain/difficulty"`, `"🌧️"`)
- 0x30 → amanecer / nuevo comienzo (`"dawn/new_start"`, `"🌅"`)
- 0x31 → brillo / magia (`"sparkle/hope"`, `"✨"`)

### 6.2. Interfaz del codec

```rust
pub trait PXCodec {
    /// Codifica un token de alto nivel a una secuencia compacta de bytes
    fn encode_token(&self, token: &PXToken) -> Vec<u8>;

    /// Decodifica bytes a un token de alto nivel
    fn decode_token(&self, data: &[u8]) -> Result<PXToken, PXCodecError>;

    /// Codifica una escena completa (simplificado)
    fn encode_scene(&self, scene: &PXScene) -> Vec<u8>;

    /// Decodifica una escena
    fn decode_scene(&self, data: &[u8]) -> Result<PXScene, PXCodecError>;
}

#[derive(Debug)]
pub enum PXCodecError {
    UnknownSymbol,
    InvalidData,
    VersionMismatch,
}
```

Cada implementación de `PXCodec` puede tener diferentes estrategias:

- **Codec estático:** tablas fijas, ideal para PX-Core-64.  
- **Codec adaptativo:** aprende a mapear símbolos específicos del usuario a códigos compactos.  
- **Codec mixto:** base fija + extensiones dinámicas.

---

### 6.3. Ejemplo conceptual de codificación de la secuencia 😔 ➜ 🚶‍♂️🌧️ ➜ 🌅✨

1. Diseñar mapa simbólico:

- `😔` → 0x01  
- `➜` → 0x40 (transición)  
- `🚶‍♂️` → 0x10  
- `🌧️` → 0x20  
- `🌅` → 0x30  
- `✨` → 0x31  

2. Secuencia binaria (ejemplo simple):

```text
[0x01, 0x40, 0x10, 0x20, 0x40, 0x30, 0x31]
```

Luego `PXCodec` puede reconstruir a nivel de dominio:

- un `PXScene` con:
  - emoción inicial Sadness
  - acción Move en contexto de dificultad
  - cierre en Dawn + Hope

---

## 7. Integración con TelescopeDB y VoxelDB

### 7.1. TelescopeDB (biografía rica)

Cada `PXScene` puede contener referencias a uno o varios “nodos” en TelescopeDB:

- nodo de texto (diario)
- fragmento de audio
- fragmento de video
- captura de pantalla
- etc.

```rust
pub struct TelescopeRef {
    pub id: String,           // ID interno en TelescopeDB
    pub kind: String,         // Ej: "text", "audio", "image"
}
```

`PXScene` puede incluir:

```rust
pub struct PXScene {
    // ...
    pub telescope_refs: Vec<TelescopeRef>,
    // ...
}
```

Esto permite que, al “hacer click” en la representación simbólica:

- se expanda el recuerdo a su material original
- se muestre el contexto completo

---

### 7.2. VoxelDB (plantillas y patrones)

VoxelDB almacena plantillas de:

- tipos de escenas (discusión, viaje, logro, pérdida, rutina, etc.)
- combinaciones típicas de emoción + acción + contexto
- patrones repetidos en la vida de una persona

Se puede representar un vínculo desde `PXScene` o `PXArc` hacia una plantilla de VoxelDB:

```rust
pub struct VoxelPatternRef {
    pub id: String,       // ID de la plantilla en VoxelDB
    pub weight: f32,      // Qué tan fuerte se asocia este patrón a la escena/arco
}
```

Esto permite:

- análisis de patrones de vida  
- compresión adicional (almacenando solo deltas)  
- sugerencias de interpretación para agentes de IA

---

## 8. API de alto nivel para agentes de IA

Se recomienda exponer un módulo Rust (o API gRPC/HTTP) que permita:

### 8.1. Codificar un evento a PXLang

```rust
pub struct EventDescription {
    pub natural_language: String,           // descripción en texto (cualquiera)
    pub timestamp: Option<i64>,
    pub objectivity_hint: Option<ObjectivityLevel>,
}

pub trait PXLangService {
    /// Dado un evento en lenguaje natural, produce una escena simbólica PXScene
    fn encode_event_to_scene(
        &self,
        event: &EventDescription,
    ) -> Result<PXScene, PXLangError>;

    /// Dada una escena simbólica, produce una descripción en lenguaje natural
    fn decode_scene_to_text(
        &self,
        scene: &PXScene,
        target_language: &str,
    ) -> Result<String, PXLangError>;

    /// Serializa y comprime una escena
    fn compress_scene(&self, scene: &PXScene) -> Vec<u8>;

    /// Restaura una escena comprimida
    fn decompress_scene(&self, data: &[u8]) -> Result<PXScene, PXLangError>;
}

#[derive(Debug)]
pub enum PXLangError {
    CodecError(PXCodecError),
    InferenceError(String),
}
```

La implementación concreta de `PXLangService` puede delegar en un LLM (vía API) para:

- transformar texto → estructura PXScene  
- transformar PXScene → texto narrativo

Rust se encarga de los tipos, la seguridad y la persistencia.

---

## 9. Estrategia de almacenamiento

### 9.1. Opciones de almacenamiento físico

1. **Binario compacto** (para backup/archivo):  
   - Archivo `.pxbio` con:
     - encabezado de versión
     - tabla de símbolos
     - secuencias de arcos / escenas codificadas

2. **JSON/CBOR** (para debug y herramientas externas):  
   - Representación legible de `PXArc`, `PXScene`, `PXToken`.

3. **Integración directa con la base de datos de Bitácora**  
   - Cada escena como documento en la colección de TelescopeDB/VoxelDB, con campo adicional `px_repr`.

### 9.2. Ejemplo de estructura JSON simplificada de una escena

```json
{
  "id": "scene_2020_mudanza_01",
  "objectivity": 2,
  "tokens": [
    { "visual_hint": "😨", "emotion": "Fear" },
    { "visual_hint": "🚚", "action": "Move", "context": "Family" },
    { "visual_hint": "🌅", "emotion": "Hope" }
  ],
  "telescope_refs": [
    { "id": "note_2020_03_15", "kind": "text" }
  ]
}
```

---

## 10. Pruebas y validación (desde el lado técnico)

Aunque los experimentos se hicieron con LLMs externos a nivel conceptual, un desarrollador Rust puede:

1. Crear tests unitarios donde:
   - Se genera una `PXScene` equivalente a la secuencia `😔 ➜ 🚶‍♂️🌧️ ➜ 🌅✨`.
   - Se codifica con `PXCodec` a bytes.
   - Se decodifica de vuelta a `PXScene`.
   - Se verifica que la estructura de alto nivel (emociones, acciones, etc.) coincida.

2. Crear un módulo de “snapshot tests” donde se guardan ejemplos de biografías simbólicas y se garantiza que nuevas versiones de PXLang siguen siendo capaces de leerlas (compatibilidad hacia atrás).

3. Implementar herramientas de inspección que impriman la biografía simbólica en forma de:
   - línea de emojis
   - JSON
   - descripción natural (p.ej. en español o inglés)

---

## 11. Consideraciones importantes para el diseño

1. **Extensibilidad:**  
   - No fijar en piedra todos los enums. Usar tablas configurables a futuro.
   - Versionar PXLang adecuadamente.

2. **Privacidad:**  
   - Los símbolos PX son altamente sensibles: describen la vida interna de la persona.
   - Se debe cifrar el almacenamiento y ofrecer control total al usuario.

3. **Local-first:**  
   - Idealmente, la codificación y decodificación PXLang ocurre en el dispositivo del usuario.
   - Los modelos remotos (LLMs) pueden ser usados como asistentes, pero no deben monopolizar los datos.

4. **Resiliencia semántica:**  
   - Aunque cambien los modelos de IA, los símbolos PX deben seguir siendo interpretables a largo plazo.
   - Para ello es importante tener definiciones canónicas de cada símbolo (en texto) almacenadas junto al esquema.

5. **Compatibilidad humana:**  
   - Siempre que sea posible, el `visual_hint` debe ser algo que un humano pueda ver (emoji, ideograma).
   - El usuario debería poder “leer” su biografía simbólica sin necesidad de ningún modelo.

---

## 12. Resumen final para el agente de codificación

En esencia, lo que se requiere es:

1. **Definir tipos de dominio en Rust** que representen:

   - símbolo PX (`PXToken`)  
   - frame (`PXFrame`)  
   - escena (`PXScene`)  
   - arco (`PXArc`)  
   - niveles de objetividad, emoción, acción, contexto, temporalidad

2. **Definir un codec** (`PXCodec`) que pueda mapear esos tipos a bytes compactos, empezando con una tabla de símbolos PX-Core-64.

3. **Implementar un servicio PXLang** (`PXLangService`) que:

   - hable con modelos de IA (si aplica) para transformar texto ↔ PXScene  
   - use el codec para comprimir y descomprimir
   - exponga funciones claras para el resto de Bitácora

4. **Integrar con TelescopeDB y VoxelDB** vía referencias cruzadas.

5. **Proveer herramientas de inspección y prueba** para que un desarrollador o usuario pueda ver:

   - la línea simbólica de su vida  
   - los datos ricos a los que apunta

Si se sigue esta especificación, cualquier agente de codificación competente en Rust podrá:

- construir el núcleo de PXLang,
- conectarlo con el ecosistema de Bitácora,
- y dar el primer paso hacia una **memoria simbólica comprimida**, lista para durar décadas o siglos.

🜛 Aquí no solo se está diseñando un módulo de software.  
Se está trazando la arquitectura de un **lenguaje de biografías humanas comprimidas**, implementado con la precisión y la seguridad de Rust.
