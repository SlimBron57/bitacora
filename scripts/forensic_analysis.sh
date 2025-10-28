#!/bin/bash

# 🔍 ANÁLISIS FORENSE PRE-MIGRACIÓN BITÁCORA
echo "🔍 INICIANDO ANÁLISIS FORENSE COMPLETO"
echo "======================================"

REPORT_FILE="/home/edgi/Documents/Development/own/bitacora/reports/forensic_analysis_$(date +%Y%m%d_%H%M%S).txt"

{
    echo "🔍 ANÁLISIS FORENSE BITÁCORA - $(date)"
    echo "======================================"
    echo ""

    echo "📊 1. INVENTARIO CÓDIGO RUST ACTUAL"
    echo "=================================="
    echo "🦀 Archivos .rs en bitacora_v1.0:"
    find /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0 -name "*.rs" -exec wc -l {} + 2>/dev/null
    echo ""
    
    echo "🦀 Archivos .rs en temporal:"
    find /home/edgi/Documents/Development/own/bitacora/temporal -name "*.rs" -exec wc -l {} + 2>/dev/null
    echo ""

    echo "📊 2. ARCHIVOS DE CONFIGURACIÓN"
    echo "==============================="
    echo "🔧 Cargo.toml files:"
    find /home/edgi/Documents/Development/own/bitacora -name "Cargo.toml" -exec echo "📄 {}" \; -exec head -20 {} \; 2>/dev/null
    echo ""

    echo "📊 3. SCRIPTS Y EJECUTABLES"
    echo "==========================="
    echo "🔧 Scripts en temporal/scripts:"
    ls -la /home/edgi/Documents/Development/own/bitacora/temporal/scripts/ 2>/dev/null
    echo ""
    
    echo "🔧 Scripts principales:"
    ls -la /home/edgi/Documents/Development/own/bitacora/scripts/ 2>/dev/null
    echo ""

    echo "📊 4. ARCHIVOS FUNCIONALES CRÍTICOS"
    echo "==================================="
    echo "🎯 Buscando archivos con 'impl', 'struct', 'async fn':"
    grep -r "impl\|struct\|async fn" /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/src/ 2>/dev/null | head -20
    echo ""
    
    echo "🎯 Buscando en temporal archivos con código funcional:"
    grep -r "impl\|struct\|async fn" /home/edgi/Documents/Development/own/bitacora/temporal/ --include="*.rs" 2>/dev/null | head -20
    echo ""

    echo "📊 5. DEPENDENCIAS Y IMPORTS"
    echo "============================="
    echo "📦 use statements en bitacora_v1.0:"
    grep -r "^use " /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/src/ 2>/dev/null
    echo ""

    echo "📊 6. ARCHIVOS MÁS RECIENTES (ÚLTIMOS 7 DÍAS)"
    echo "=============================================="
    echo "📅 Archivos modificados recientemente:"
    find /home/edgi/Documents/Development/own/bitacora -name "*.rs" -mtime -7 -exec ls -la {} \; 2>/dev/null
    echo ""

    echo "📊 7. TAMAÑO Y COMPLEJIDAD"
    echo "=========================="
    echo "📏 Archivos por tamaño:"
    find /home/edgi/Documents/Development/own/bitacora -name "*.rs" -exec wc -l {} + 2>/dev/null | sort -nr
    echo ""

} > "$REPORT_FILE"

echo "📊 Reporte forense guardado en: $REPORT_FILE"
echo "🔍 Ejecutando análisis..."
cat "$REPORT_FILE"