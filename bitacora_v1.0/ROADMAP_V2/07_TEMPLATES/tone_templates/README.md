# Tone Templates (MTT-DSL)

## Propósito

Templates YAML para definir **ToneClusters** en el sistema EmotionalSpace. Cada usuario tiene su propio conjunto de tonos emocionales personalizados, almacenados en VoxelDB.

## Ubicación en VoxelDB

```
VoxelDB/users/{user_id}/tones/{tone_name}.yaml
```

## Formato del Template

```yaml
metadata:
  name: string                    # Nombre del tono (user-defined)
  user_id: string
  discovered_at: datetime         # ISO 8601
  discovered_from: string         # Ejemplo original
  version: string
  
dimensions:
  valence: float                  # -1.0 a 1.0 (negativo a positivo)
  arousal: float                  # -1.0 a 1.0 (calmado a excitado)
  dominance: float                # -1.0 a 1.0 (sumiso a dominante)
  formality: float                # -1.0 a 1.0 (casual a formal)
  
cluster:
  center: [float, float, float, float]  # [v, a, d, f]
  radius: float                   # Tolerancia para matching
  
lexical_markers:
  strong_verbs: string[]
  commitment_phrases: string[]
  time_markers: string[]
  emotional_adjectives: string[]
  exclamation_count: int
  question_count: int
  uppercase_words: string[]
  
syntactic_patterns:
  - pattern: string
    confidence: float             # 0.0-1.0
    examples: string[]
    
response_adaptation:
  style: string                   # Estilo de respuesta
  energy_level: float             # 0.0-1.0
  formality_match: bool           # Igualar formality del usuario
  tone_adjustments: string[]      # Lista de ajustes a aplicar
```

## Modelo VAD+F

### Valence (Valencia)
- **+1.0**: Muy positivo (alegre, emocionado, satisfecho)
- **0.0**: Neutral
- **-1.0**: Muy negativo (triste, frustrado, enojado)

### Arousal (Activación)
- **+1.0**: Muy excitado (energético, ansioso, sorprendido)
- **0.0**: Neutral
- **-1.0**: Muy calmado (relajado, aburrido, fatigado)

### Dominance (Dominancia)
- **+1.0**: Muy dominante (asertivo, controlador, confiado)
- **0.0**: Neutral
- **-1.0**: Muy sumiso (pasivo, inseguro, preguntando)

### Formality (Formalidad)
- **+1.0**: Muy formal (profesional, técnico, "usted")
- **0.0**: Neutral
- **-1.0**: Muy casual (slang, emojis, contracciones)

## Ejemplos

Ver archivos en este directorio:
- `example_determinado.yaml` - Tono asertivo, alta energía (Eduardo)
- `example_nostalgico.yaml` - Tono reflexivo, baja energía (Eduardo)
- `example_curioso.yaml` - Tono exploratorio, moderado (Eduardo)

## Auto-Discovery Flow

1. Sistema calcula dimensiones VAD+F del mensaje
2. Busca cluster cercano en EmotionalSpace (dentro de radius)
3. Si NO encuentra: nuevo tono detectado
4. Sistema describe dimensiones al usuario
5. Usuario nombra el tono (ej: "Determinado")
6. Sistema crea ToneCluster con ese nombre
7. Template se refina con más ejemplos

## Response Adaptation Styles

- **direct_supportive**: Respuestas directas, pasos concretos, sin excusas
- **reflective_gentle**: Respuestas reflexivas, exploratorias, sin presión
- **energetic_enthusiastic**: Respuestas con alta energía, celebración
- **technical_expert**: Respuestas técnicas detalladas, sin simplificar
- **casual_friendly**: Respuestas informales, amigables, cercanas
