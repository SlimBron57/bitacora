#!/bin/bash

# Script de análisis completo del proyecto Bitácora
echo "🔍 ANÁLISIS COMPLETO DEL PROYECTO BITÁCORA"
echo "========================================"

# Función para crear separador visual
separator() {
    echo ""
    echo "----------------------------------------"
    echo "$1"
    echo "----------------------------------------"
}

# 1. ANÁLISIS DEL DIRECTORIO TEMPORAL
separator "📁 CONTENIDO DEL DIRECTORIO TEMPORAL"
if [ -d "/home/edgi/Documents/Development/own/bitacora/temporal" ]; then
    echo "✅ Directorio temporal encontrado"
    ls -la /home/edgi/Documents/Development/own/bitacora/temporal/
    echo ""
    echo "📋 Archivos Markdown en temporal:"
    find /home/edgi/Documents/Development/own/bitacora/temporal -name "*.md" -type f 2>/dev/null
    echo ""
    echo "📊 Todos los archivos en temporal:"
    find /home/edgi/Documents/Development/own/bitacora/temporal -type f 2>/dev/null | head -20
else
    echo "❌ Directorio temporal no encontrado"
fi

# 2. ANÁLISIS DEL CÓDIGO BITÁCORA V1.0
separator "🚀 CONTENIDO DE BITÁCORA V1.0"
if [ -d "/home/edgi/Documents/Development/own/bitacora/bitacora_v1.0" ]; then
    echo "✅ Directorio bitacora_v1.0 encontrado"
    ls -la /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/
    echo ""
    echo "📊 Estructura de src:"
    if [ -d "/home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/src" ]; then
        ls -la /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/src/
    fi
    echo ""
    echo "💻 Archivos de código principales:"
    find /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0 -name "*.rs" -o -name "*.py" -o -name "*.js" -o -name "*.ts" -o -name "*.toml" 2>/dev/null
else
    echo "❌ Directorio bitacora_v1.0 no encontrado"
fi

# 3. BÚSQUEDA DE ARCHIVOS DE LOGROS
separator "🏆 ARCHIVOS DE LOGROS HISTÓRICOS"
echo "🔍 Buscando archivos de logros..."
find /home/edgi/Documents/Development/own/bitacora -name "*LOGRO*" -o -name "*BREAKTHROUGH*" 2>/dev/null
echo ""
echo "📋 Contenido de archivos de logros encontrados:"
for file in $(find /home/edgi/Documents/Development/own/bitacora -name "*LOGRO*" -o -name "*BREAKTHROUGH*" 2>/dev/null); do
    echo "📄 === $file ==="
    head -20 "$file" 2>/dev/null || echo "No se pudo leer el archivo"
    echo ""
done

# 4. ANÁLISIS GENERAL DEL PROYECTO
separator "📊 ESTRUCTURA GENERAL DEL PROYECTO"
echo "📁 Directorios principales:"
ls -la /home/edgi/Documents/Development/own/bitacora/

# 5. ROADMAP STATUS
separator "🗺️ ESTADO DEL ROADMAP"
if [ -d "/home/edgi/Documents/Development/own/bitacora/ROADMAP" ]; then
    echo "✅ ROADMAP encontrado"
    echo "📄 Estructura del ROADMAP:"
    find /home/edgi/Documents/Development/own/bitacora/ROADMAP -type d | head -10
    echo ""
    echo "📋 Documentos principales:"
    find /home/edgi/Documents/Development/own/bitacora/ROADMAP -name "*.md" -type f | head -10
else
    echo "❌ Directorio ROADMAP no encontrado"
fi

# 6. ANÁLISIS DE ARCHIVOS UI
separator "🎨 BITÁCORA UI V1.0"
if [ -d "/home/edgi/Documents/Development/own/bitacora/bitacora_UIv1.0" ]; then
    echo "✅ Directorio bitacora_UIv1.0 encontrado"
    ls -la /home/edgi/Documents/Development/own/bitacora/bitacora_UIv1.0/
else
    echo "❌ Directorio bitacora_UIv1.0 no encontrado"
fi

echo ""
echo "✅ Análisis completado."