# Topic Templates (MTT-DSL)

## Propósito

Templates YAML para definir **UserDefinedTopics** en el sistema TopicGraph. Cada usuario tiene su propio conjunto de topics personalizados, almacenados en VoxelDB.

## Ubicación en VoxelDB

```
VoxelDB/users/{user_id}/topics/{topic_name}.yaml
```

## Formato del Template

```yaml
metadata:
  name: string                    # Nombre del topic (user-defined)
  created_by: string              # user_id
  created_at: datetime            # ISO 8601
  version: string                 # Semantic versioning
  parent_topic: string?           # Topic padre (opcional)
  
detection:
  keywords: string[]              # Palabras clave para detección
  phrases: string[]               # Frases comunes
  embedding_similarity_threshold: float  # 0.0-1.0 (default: 0.75)
  
interest_weight:
  explicit: float                 # 0.0-1.0 (usuario expresó interés)
  implicit: float                 # 0.0-1.0 (frecuencia menciones)
  temporal: float                 # 0.0-1.0 (recencia)
  emotional: float                # 0.0-1.0 (sentimiento)
  combined: float                 # Calculado automáticamente
  
response_style:
  formality: float                # -1.0 a 1.0
  detail_level: float             # 0.0-1.0 (bajo a alto)
  include_examples: bool
  include_visuals: bool
  tone_adaptation: string         # Estilo de respuesta
  
related_topics:
  - topic: string
    connection: enum              # Complementary | Hierarchical | Sequential | Contrasting | Forbidden
    reason: string
    strength: float               # 0.0-1.0
    
learning_path:
  current_level: enum             # Beginner | Intermediate | Advanced | Expert
  topics_completed: string[]
  topics_next: string[]
```

## Ejemplos

Ver archivos en este directorio:
- `example_ceramica.yaml` - Topic de artesanía (Eduardo)
- `example_rust.yaml` - Topic de programación (Eduardo)
- `example_microprocesadores.yaml` - Topic técnico moderado (Eduardo)

## Isolation Modes

Los topics pueden tener diferentes modos de aislamiento:

- **Strict**: NUNCA mezclar con otros topics (ej: Espiritualidad ⊥ Tecnología)
- **Soft**: Permitir si usuario conecta explícitamente
- **Exploratory**: Sugerir conexiones pero pedir confirmación

Especificar en template con:
```yaml
isolation:
  mode: "Strict" | "Soft" | "Exploratory"
  forbidden_topics: [topic_id1, topic_id2, ...]
```

## Auto-Discovery Flow

1. Sistema detecta topic nuevo (similarity < 0.6 con existentes)
2. Extrae keywords del mensaje
3. Sugiere nombre al usuario
4. Usuario confirma/modifica nombre
5. Sistema crea template inicial
6. Template se refina con más interacciones
