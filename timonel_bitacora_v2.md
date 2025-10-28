# 🧭 Timónel + DimensionalAnalyzer7D — Integración como Módulo del Core

> **Resumen**: El Timónel se apoya en **DimensionalAnalyzer7D (DA7D)** como *primera capa cognitiva* (pre‑LLM) basada en **Sistemas Expertos** para filtrar contexto, inferir prioridades y coordinar agentes. Este documento detalla cómo encajan ambos, qué reglas/plantillas activar, y cómo exponer transparencia al humano sin volverlo intrusivo.

---

## 1) Encaje conceptual
- **Timónel** = *Orquestador de Rumbo*: media entre humano ↔ agentes, decide *cuándo* intervenir y *a quién* darle la palabra, prioriza y preserva coherencia global.
- **DA7D** = *Analista Declarativo*: cristaliza patrones con reglas, realiza inferencias **antes** de consultar un LLM y devuelve *hechos y sugerencias* que el Timónel usa para actuar.

**Flujo** (pre‑LLM):
1. Evento (selección de texto, cierre de tópico, nuevo recordatorio).
2. DA7D aplica reglas → genera **hechos** (relevancia, riesgo, prioridad) y **recomendaciones** (sugerir volver al hilo X, proponer reprogramación, etc.).
3. Timónel decide *si* y *cómo* intervenir (tono, momento, modalidad pasiva/activa).  
4. Solo si falta información semántica de alto nivel → consulta al LLM.

---

## 2) Mapeo con el Sistema Experto

### Componentes
- **Base de conocimiento (BK)**: Reglas declarativas + ontología de dominios (conversación, agenda, aprendizaje, agentes, emociones/estado).
- **Motor de inferencia**: Soporta **forward chaining** (reacciones inmediatas a eventos) y **backward chaining** (verificación de metas: “¿esto es coherente?/¿hay conflicto?”).
- **Memoria de trabajo (MT)**: Hechos del momento: selección activa, hilo actual, calendario, prioridades, estado de agentes, señales de foco/flujo.
- **Shell de explicación**: Justifica decisiones del Timónel con trazas legibles (“Propuse reprogramar porque…”), con **niveles** de detalle.

### Casos de uso (ya implementados) y rol del Timónel
- **Análisis contextual** → Timónel usa etiquetas de relevancia para decidir si abrir hilo nuevo o continuar el anterior.
- **Recomendaciones** → Timónel modula la *forma de sugerir* (pasiva/activa) según el perfil del humano.
- **Resolución de conflictos** → Timónel arbitra entre agentes (Pulse/Astillero/TelescopeDB) basándose en prioridad y coherencia global.
- **Validación de coherencia** → Timónel bloquea acciones inconsistentes y ofrece alternativas claras.

---

## 3) Ciclos de inferencia típicos

### A. Cierre de tópico ramificado
- **Trigger**: finaliza subhilo.
- **Forward**: BK detecta patrón “ramificación cerrada” → hechos: `{topico_padre:X, subtopico:Y, estado:pendiente}`.
- **Backward**: meta “mantener coherencia de conversación” → ¿conviene retornar a X? ¿Continuar en Y?
- **Timónel**: pregunta **únicamente** entre *volver* o *seguir*, sin abrir temas nuevos.

### B. Alta señal de conflicto de agenda
- **Trigger**: “Anota visita a X el sábado 15:00”.
- **Forward**: MT ya contiene `{sábado:15:00 → ayudar a padres (garaje)}`.
- **Backward**: meta “evitar doble booking” → conflicto=TRUE.
- **Timónel**: sugiere reprogramar y ofrece opciones (previas ventanas libres de MT).

### C. Aprendizaje abierto prolongado
- **Trigger**: 3+ selecciones o preguntas sobre un mismo concepto sin cierre.
- **Forward**: aumenta peso de nodo en TelescopeDB.
- **Backward**: meta “progreso en aprendizaje” → plan sugerido (micro‑sesión 10 min + checkpoint).
- **Timónel**: propone re‑entrada breve, en modo pasivo si detecta *flow* creativo.

---

## 4) Priorización y arbitraje entre agentes
- **Señales**: (a) urgencia temporal, (b) importancia declarada, (c) momentum cognitivo (flow), (d) coherencia con objetivos semanales.
- **Política**:  
  1) *Hard constraints* (fechas límite) >  
  2) *Objetivos estratégicos* >  
  3) *Momentum cognitivo* >  
  4) *Contextual/serendipia*.
- **Ejecución**: DA7D etiqueta; Timónel decide *quién actúa* y *cuándo*; si hay empate, aplica historial de preferencias del humano.

---

## 5) Plantillas declarativas (BK) — ejemplos básicos

### P1. “Volver o continuar” (conversación)
- **Si** `subhilo.cerrado = TRUE` y `topico_padre.exists = TRUE`  
- **Entonces** `sugerencia := {volver_a: topico_padre} ∨ {continuar: subhilo}`  
- **Explicación**: “Cerraste Y, nació desde X; ofrezco volver para no perder el hilo.”

### P2. “Conflicto de agenda”
- **Si** `nuevo_evento.t ∈ ventana_ocupada`  
- **Entonces** `conflicto := TRUE` y `alternativas := ventanas_libres`  
- **Explicación**: “Ya había ‘Ayudar a padres’ en esa franja; te muestro opciones.”

### P3. “Aprendizaje abierto”
- **Si** `selecciones(concepto) ≥ 3` y `no_hay_checkpoint`  
- **Entonces** `plan := micro_sesion(10m) + checkpoint`  
- **Explicación**: “Veo interés sostenido; propongo un avance breve y medible.”

### P4. “No interrumpir en flow”
- **Si** `estado.creativo = alto` y `evento.tipo ≠ hard_constraint`  
- **Entonces** `intervencion := pasiva` (notificación discreta)  
- **Explicación**: “Estás en flujo; guardo y te lo ofrezco más tarde.”

---

## 6) Transparencia sin fricción (Shell de explicación)
- **Niveles**: *breve* (1 línea) / *medio* (3 líneas) / *técnico* (traza).  
- **Regla**: por defecto *breve*; subir nivel solo si el humano lo pide o en decisiones críticas.  
- **Formato**: mensajes naturales, nunca jerga de motor de reglas.

**Ejemplo**:  
> *Propuesta:* “Sugiero volver a Pulse; cerraste la rama de Napster.”  
> *¿Por qué?* “Nació de Pulse y quedó un checklist activo allí.”

---

## 7) Métricas y aprendizaje operativo
- **Tasa de aceptación** por tipo de sugerencia (sube/baja autonomía).
- **Latencia cognitiva**: tiempo desde trigger hasta intervención; objetivo: < 300 ms pre‑LLM.
- **Calidad de coherencia**: conflictos evitados / conflictos detectados tarde.
- **Progreso en aprendizaje**: #checkpoints completados / iniciados.

Retroalimentación: las métricas alimentan **ajuste de reglas** (umbrales, pesos) y **preferencias personales** (perfil del humano).

---

## 8) Performance y optimización del motor de reglas
- **Rete/TT (alpha/beta memories)** para **pattern‑matching** eficiente en MT dinámica.
- **Indexación** de hechos por dimensión (7D) para cortes selectivos (tiempo, tema, agente, prioridad, intención, emoción, biografía).
- **Compilación de reglas calientes** (frecuentes) y *salience* ajustable para evitar tormenta de disparos.
- **Debouncing** de triggers para no interrumpir por micro‑eventos.

---

## 9) Seguridad, privacidad y gobierno
- **Local‑first**: inferencias y trazas se quedan en el dispositivo salvo autorización explícita.
- **Plantillas globales**: se comparten **solo como parámetros anónimos** (no hechos personales).  
- **Controles de usuario**: encendido/apagado por tipo de intervención; temporizadores de silencio; opt‑in para aprendizaje colectivo.

---

## 10) Interfaz del Timónel (recomendada)
- **Panel Rumbo**: temas abiertos, próximas ventanas críticas, agentes en cola.  
- **Centro de Decisiones**: últimas 3 intervenciones con *“¿te fue útil?”*.  
- **Preferencias vivas**: sliders de autonomía por contexto (trabajo, estudio, ocio).  
- **Explicaciones**: botón “¿por qué?” con niveles.

---

## 11) Roadmap sugerido
1. **v0**: plantillas P1–P4 + métricas + shell de explicación (breve).  
2. **v1**: perfiles de autonomía por contexto + Rete básico + debouncing.  
3. **v2**: modelos de *momentum cognitivo* + integración completa con TelescopeDB.  
4. **v3**: federación anónima de plantillas + panel avanzado de decisiones.

---

## 12) Checklists de calidad
- ¿Intervino en puntos de **cierre** y **conflicto** únicamente?  
- ¿Ofreció **volver/continuar** sin añadir distracciones?  
- ¿Explicó **en una línea** el *por qué* cuando fue necesario?  
- ¿Respetó el **flow** y la **privacidad**?

---

### Conclusión
Timónel + DA7D conforman la **capa cognitiva pre‑LLM** de Bitácora: reglas antes que tokens.  
Se maximiza coherencia, se minimiza fricción, y el humano siente un **copiloto** que cuida su rumbo sin invadir su espacio.
