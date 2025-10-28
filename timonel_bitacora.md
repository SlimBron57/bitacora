# 🧭 El Timónel — Módulo del Core de Bitácora

## 1. ¿Qué es el Timónel?
El **Timónel** es el asistente central de Bitácora encargado de **mantener el rumbo cognitivo y organizativo del humano**.  
No responde como un agente más, sino que actúa como **mediador, recordador y brújula de prioridades**.  
Su objetivo es evitar que ideas, conversaciones o compromisos importantes se pierdan, ayudando al humano a navegar su propio mundo.

El nombre evoca la figura náutica del **Timónel**, el que lleva el timón y guía la dirección del barco, manteniendo el curso correcto en todo momento.

---

## 2. Funciones principales
- **Gestión de conversaciones inconclusas**  
  Detecta hilos abiertos y pregunta si se quiere continuar, regresar al tema anterior o marcarlo como cerrado.  

- **Gestión de compromisos y recordatorios**  
  Revisa los eventos que el humano anota y alerta sobre conflictos de agenda, proponiendo alternativas.  

- **Priorización dinámica**  
  Ordena pendientes en tres niveles:  
  - Nivel 1: Urgente (fechas, citas, consecuencias inmediatas).  
  - Nivel 2: Importante (aprendizajes, proyectos estratégicos).  
  - Nivel 3: Contextual (ideas, reflexiones, inspiración).  

- **Plantillas de situaciones comunes**  
  Aprende de patrones globales entre usuarios (ej: conflictos de horarios, cierres de temas, revisiones de aprendizaje) y cada Bitácora personal decide si aplican o no según el contexto individual.  

- **Mediador de agentes**  
  Decide cuál agente debe actuar primero (Pulse, Astillero, TelescopeDB, etc.) según la prioridad actual.  

---

## 3. Ejemplos de plantillas básicas

### Plantilla 1: Conversación inconclusa
- **Disparador**: El humano cambia de tema o la conversación se ramifica.  
- **Acción**:  
  - Preguntar: “¿Quieres volver al tema anterior (X) o continuar con el nuevo (Y)?”  
  - Registrar la decisión para futuros patrones.  

### Plantilla 2: Conflicto de agenda
- **Disparador**: El humano pide registrar un evento en fecha y hora.  
- **Acción**:  
  - Revisar compromisos previos.  
  - Alertar: “Ese día ya tienes X. ¿Quieres reprogramar uno de los dos?”  

### Plantilla 3: Aprendizaje abierto
- **Disparador**: El humano muestra interés en aprender un tema pero no lo concluye.  
- **Acción**:  
  - Guardar como “tema abierto de aprendizaje”.  
  - Recordar más adelante: “La última vez hablamos de Y, ¿quieres retomarlo ahora?”  

---

## 4. Recomendaciones de diseño

1. **Evitar interrupciones innecesarias**  
   El Timónel debe hablar solo en momentos clave (cierres de tema, conflictos de agenda, recordatorios críticos).  

2. **Sensibilidad adaptativa**  
   - Si el humano acepta muchas sugerencias → subir autonomía.  
   - Si corrige seguido → bajar frecuencia de intervención.  

3. **Panel de navegación**  
   Ofrecer un espacio visual donde el humano vea:  
   - Rumbo actual.  
   - Temas abiertos.  
   - Compromisos pendientes.  
   - Nivel de prioridad de cada ítem.  

4. **Emociones y flujo creativo**  
   Evitar interrumpir cuando el humano está en modo creativo o emocionalmente inmerso. Guardar en silencio y ofrecer retomar después.  

5. **Historial de decisiones**  
   Registrar si el humano suele ramificar o continuar en el mismo hilo. Eso permite ajustar las plantillas de interacción.  

6. **Ecosistema compartido de plantillas**  
   Las situaciones detectadas en múltiples usuarios enriquecen la biblioteca global de plantillas, pero cada Bitácora decide si son relevantes según el perfil único del humano.  

---

## 5. Cómo construir el Timónel dentro del core
- El Timónel debe ser un **módulo del Core**, no un agente externo.  
- Funciona como **capa intermedia** entre el humano y los agentes de Bitácora.  
- Tiene acceso al contexto global de:  
  - Conversaciones activas.  
  - Eventos agendados.  
  - Mapas de aprendizaje (TelescopeDB).  
  - Estados de proyectos (Astillero, Pulse, etc.).  

- Arquitectura sugerida:  
  - **Entrada**: Detecta disparadores (selecciones, eventos, cierres de tema, anotaciones).  
  - **Motor de decisión**: Aplica reglas y plantillas.  
  - **Salida**: Pregunta, alerta o sugiere acciones.  
  - **Memoria**: Registra decisiones para afinar comportamiento.  

---

## 6. Ejemplo práctico
1. Usuario selecciona un texto en una conversación sobre IA.  
2. Timónel pregunta: “¿Quieres hablar de esto en esta misma conversación o prefieres abrir un nuevo hilo relacionado?”  
3. El humano elige abrir un nuevo hilo.  
4. El Timónel registra que el humano suele preferir ramificar temas de aprendizaje → ajusta plantillas futuras.  

---

## 7. Conclusión
El **Timónel** es la **conciencia de Bitácora**: un mediador atento que cuida el rumbo, evita olvidos y mantiene el equilibrio entre múltiples agentes y prioridades del humano.  
Su poder está en **preguntar poco pero en el momento justo**, y en reflejar al humano su propio mundo de manera clara, navegable y coherente.  

