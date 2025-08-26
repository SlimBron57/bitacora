# Documentación Completa de la Extensión BitaFlow para VS Code

## 🎯 **Propósito de la Extensión**

La extensión BitaFlow es una herramienta de soporte de lenguaje para VS Code que proporciona resaltado de sintaxis, snippets y funcionalidades de edición para archivos BitaFlow. BitaFlow parece ser un lenguaje de dominio específico (DSL) diseñado para describir procesos, flujos de trabajo o plantillas con una sintaxis híbrida que combina elementos de:

- **YAML** (para metadatos/front-matter)
- **Markdown** (para documentación)
- **Sintaxis propia** (para operadores de flujo y placeholders)

## 📁 **Extensiones de Archivo Soportadas**

La extensión reconoce y procesa los siguientes tipos de archivo:
- `.bt` - Archivos BitaFlow básicos
- `.bfl` - Archivos BitaFlow completos
- `.bita` - Archivos BitaFlow con alias/plantillas

## 🔧 **Funcionalidades Implementadas**

### 1. **Resaltado de Sintaxis (Syntax Highlighting)**

#### **Front-Matter YAML**
```yaml
---
alias: BITA-TPL-FOO-v1
name: Nombre legible
slug: slug
kind: TPL
version: 1
requires: []
---
```
- **Procesamiento**: Detecta bloques delimitados por `---` al inicio y final
- **Scope**: `meta.front-matter.bfl`
- **Herencia**: Incluye resaltado YAML completo para los metadatos

#### **Encabezados Markdown**
```markdown
# Título Principal
## Subtítulo
### Sub-subtítulo
```
- **Procesamiento**: Detecta líneas que empiezan con `#` (1-6 niveles)
- **Scope**: `entity.name.section.bfl`
- **Función**: Estructura jerárquica de documentos

#### **Comentarios de Línea**
```bfl
;; Este es un comentario
;; Los comentarios son ignorados en el procesamiento
```
- **Procesamiento**: Líneas que empiezan con `;;`
- **Scope**: `comment.line.semicolon.bfl`
- **Función**: Documentación inline y notas

#### **Includes/Inclusiones**
```bfl
{{> BITA-TPL-COMMON-v1 }}
{{> OTRO-TEMPLATE-v2 }}
```
- **Procesamiento**: Patrón `{{> ALIAS }}` con alias en mayúsculas, números, guiones
- **Scope**: `support.function.include.bfl`
- **Función**: Inclusión de plantillas o módulos externos

#### **Placeholders/Variables**
```bfl
{{nombre}}
{{descripcion_tarea}}
{{valor_calculado}}
```
- **Procesamiento**: Cualquier contenido entre `{{` y `}}` que no sea include
- **Scope**: `variable.other.placeholder.bfl`
- **Función**: Variables que serán reemplazadas en tiempo de ejecución

#### **Operadores de Flujo**
```bfl
A ↦ B        # Flujo secuencial
A + B        # Operación paralela/suma
A ↘ B        # Flujo hacia abajo-derecha  
A ↗ B        # Flujo hacia arriba-derecha
```
- **Procesamiento**: Caracteres especiales `↦`, `↘`, `↗`, `+`
- **Scope**: `keyword.operator.flow.bfl`
- **Función**: Definición de flujos y relaciones entre procesos

### 2. **Configuración de Lenguaje**

#### **Comentarios**
- **Tipo**: Comentario de línea
- **Símbolo**: `;;` (con espacio)
- **Uso**: `Ctrl+/` para comentar/descomentar líneas

#### **Pares de Cierre Automático**
- `{` se cierra con `}`
- `[` se cierra con `]`
- `(` se cierra con `)`
- `` ` `` se cierra con `` ` ``

#### **Pares Envolventes**
- Selección + `` ` `` envuelve con backticks
- Selección + `*` envuelve con asteriscos (para énfasis)

#### **Brackets de Navegación**
- `{}` - Llaves para bloques
- `[]` - Corchetes para arrays/listas
- `()` - Paréntesis para agrupación

### 3. **Snippets de Código**

#### **Front-Matter Completo (`bfl-front`)**
```yaml
---
alias: ${1:BITA-TPL-FOO-v1}
name: ${2:Nombre legible}
slug: ${3:slug}
kind: ${4:TPL}
version: ${5:1}
requires: [${6}]
---

# ${7:Título}
**Objetivo:** ${8:...}
```
- **Trigger**: `bfl-front`
- **Función**: Genera estructura básica completa de documento BitaFlow

#### **Include Rápido (`bfl-include`)**
```bfl
{{> ${1:BITA-TPL-DOD-v1} }}
```
- **Trigger**: `bfl-include`
- **Función**: Inserción rápida de referencias a plantillas

#### **Placeholder (`bfl-ph`)**
```bfl
{{${1:nombre}}}
```
- **Trigger**: `bfl-ph`
- **Función**: Creación rápida de variables/placeholders

#### **Operadores de Flujo (`bfl-ops`)**
```bfl
A ↦ (B + C) ↦ D
   ↘       ↗
     S
```
- **Trigger**: `bfl-ops`
- **Función**: Ejemplo de sintaxis de flujo compleja

## 🔄 **Procesamientos que Realiza la Extensión**

### **1. Análisis Lexical**
- Tokeniza el contenido del archivo en elementos reconocibles
- Identifica patrones mediante expresiones regulares
- Separa el contenido en scopes semánticos diferentes

### **2. Análisis Sintáctico**
- Reconoce estructura jerárquica (front-matter → contenido → includes)
- Valida patrones de sintaxis específicos de BitaFlow
- Mantiene contexto entre diferentes tipos de contenido

### **3. Resaltado Semántico**
- Aplica colores y estilos según el tema de VS Code
- Diferencia entre tipos de tokens (keywords, variables, comments, etc.)
- Proporciona feedback visual inmediato sobre la estructura

### **4. Asistencia de Edición**
- Auto-completado de snippets mediante triggers
- Navegación por brackets y pares
- Comentado/descomentado inteligente

### **5. Integración con VS Code**
- Registra el lenguaje en el sistema de lenguajes de VS Code
- Habilita funcionalidades estándar (find/replace, folding, etc.)
- Proporciona iconos específicos para archivos BitaFlow

## 📝 **Casos de Uso Típicos**

### **1. Creación de Plantillas**
```bfl
---
alias: BITA-TPL-ONBOARDING-v1
kind: TPL
---

# Proceso de Onboarding
**Objetivo:** Integrar nuevo empleado

Bienvenida ↦ Documentación ↦ Capacitación
          ↘              ↗
            Setup Técnico
```

### **2. Definición de Procesos**
```bfl
---
alias: PROC-DEPLOY-v2
kind: PROC
---

# Proceso de Despliegue

;; Etapas del pipeline
Build ↦ Test ↦ Deploy
      ↘     ↗
        QA Review

Usuario: {{developer}}
Entorno: {{environment}}
```

### **3. Documentación Modular**
```bfl
---
alias: DOC-SECURITY-v1
kind: DOC
---

# Políticas de Seguridad

{{> BITA-TPL-HEADER-v1 }}

## Autenticación
Usuario: {{username}}
Método: {{auth_method}}
```

## 🔮 **Extensiones Futuras Posibles**

1. **Validación en Tiempo Real**: Verificar sintaxis y referencias
2. **Auto-completado Inteligente**: Sugerir aliases existentes
3. **Navegación**: Ir a definición de includes
4. **Refactoring**: Renombrar aliases en todo el proyecto
5. **Preview**: Vista previa del resultado procesado
6. **Integración**: Conexión con herramientas de BitaFlow

## 🎨 **Temas de Color Soportados**

La extensión utiliza scopes estándar de TextMate que son compatibles con todos los temas de VS Code:
- `entity.name.section` - Encabezados (típicamente azul/verde)
- `support.function` - Includes (típicamente azul claro)
- `variable.other` - Placeholders (típicamente blanco/amarillo)
- `keyword.operator` - Operadores (típicamente rosa/rojo)
- `comment.line` - Comentarios (típicamente gris/verde apagado)
- `meta.front-matter` - Front-matter (hereda colores YAML)

Esta extensión transforma VS Code en un entorno completo para trabajar con archivos BitaFlow, proporcionando todas las herramientas necesarias para crear, editar y mantener documentos de este lenguaje de dominio específico.
