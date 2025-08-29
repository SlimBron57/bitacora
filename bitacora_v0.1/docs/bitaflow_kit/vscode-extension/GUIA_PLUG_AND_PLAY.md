# 🚀 BitaFlow - Guía Plug and Play

## ⚡ Inicio Rápido en 30 Segundos

### 1. **Ejecutar la Extensión** (Una sola vez)
```bash
cd /home/edgi/Documents/Development/own/bitacora/bitaflow_kit/vscode-extension
./run-extension.sh
```
> ✅ Se abre una nueva ventana de VS Code con BitaFlow activado

### 2. **Crear tu Primer Archivo**
- Crea un archivo con extensión `.bfl` (ejemplo: `mi-proceso.bfl`)
- Escribe `bfl-front` y presiona `Tab`
- ¡Listo! Ya tienes un documento BitaFlow básico

## 📝 Todo lo que Necesitas Saber

### **Extensiones de Archivo Válidas:**
- `.bfl` - BitaFlow completo
- `.bt` - BitaFlow básico  
- `.bita` - BitaFlow con alias

### **4 Comandos Mágicos (Auto-completado):**

| Escribe esto | Presiona Tab | Obtienes |
|--------------|--------------|-----------|
| `bfl-front` | Tab | Plantilla completa con metadatos |
| `bfl-include` | Tab | `{{> TEMPLATE }}` |
| `bfl-ph` | Tab | `{{variable}}` |
| `bfl-ops` | Tab | Operadores de flujo `↦ ↘ ↗` |

## 🎨 Sintaxis Simple

### **Metadatos (Al inicio del archivo):**
```yaml
---
alias: MI-PROCESO-v1
name: Mi Proceso Genial
---
```

### **Títulos:**
```markdown
# Título Principal
## Subtítulo
```

### **Comentarios:**
```bfl
;; Esto es un comentario
```

### **Variables:**
```bfl
El usuario {{nombre}} hizo {{accion}}
```

### **Incluir otros archivos:**
```bfl
{{> OTRO-PROCESO-v1 }}
```

### **Flujos de Proceso:**
```bfl
Inicio ↦ Proceso ↦ Final
      ↘        ↗
        Validación
```

## 🛠️ Atajos de Teclado

| Atajo | Función |
|-------|---------|
| `Ctrl + /` | Comentar/descomentar línea |
| `Tab` | Navegar entre campos en snippets |
| `Ctrl + Space` | Ver sugerencias de auto-completado |

## 🎯 Ejemplo Completo (Copia y Pega)

```bfl
---
alias: BITA-EJEMPLO-v1
name: Mi Primer Proceso
slug: ejemplo
kind: TPL
version: 1
---

# 🎉 Mi Primer Proceso BitaFlow

**Objetivo:** Aprender BitaFlow rápidamente

;; Este es mi primer comentario

## Flujo del Proceso

Inicio ↦ (Validar + Procesar) ↦ Finalizar
      ↘                     ↗
        Error Handling

## Variables Dinámicas

- Usuario: {{usuario_actual}}
- Fecha: {{fecha_proceso}}
- Estado: {{estado_final}}

## Incluir Otros Procesos

{{> BITA-TPL-COMMON-v1 }}
```

## 🔄 Para Próximas Veces

1. **Navegar al directorio:**
   ```bash
   cd /home/edgi/Documents/Development/own/bitacora/bitaflow_kit/vscode-extension
   ```

2. **Ejecutar:**
   ```bash
   ./run-extension.sh
   ```

3. **Crear archivo `.bfl` y usar snippets**

## 📂 Archivos Importantes

- **`test.bfl`** - Ejemplo completo para copiar
- **`DOCUMENTACION_COMPLETA.md`** - Manual detallado
- **`run-extension.sh`** - Script para ejecutar fácilmente

---

## 💡 Tips Rápidos

✅ **Siempre empezar con** `bfl-front`  
✅ **Usar** `;;` **para comentarios**  
✅ **Los flujos se hacen con** `↦ ↘ ↗ +`  
✅ **Las variables van entre** `{{}}` 
✅ **Los includes van con** `{{> NOMBRE }}`

¡Es así de simple! 🎉
