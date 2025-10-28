#!/bin/bash

# 🧪 PRUEBA DE COMPILACIÓN ACTUAL
echo "🧪 PRUEBA DE ESTADO ACTUAL DE BITÁCORA"
echo "======================================"

cd /home/edgi/Documents/Development/own/bitacora/bitacora_v1.0

echo "📊 ESTADO ANTES DE CUALQUIER MIGRACIÓN:"
echo "======================================="

echo "📁 Directorio actual:"
pwd

echo ""
echo "📋 Archivos en src/:"
ls -la src/

echo ""
echo "🔧 Verificando Cargo.toml:"
if [ -f "Cargo.toml" ]; then
    echo "✅ Cargo.toml encontrado"
    cat Cargo.toml
else
    echo "❌ Cargo.toml NO encontrado"
fi

echo ""
echo "🦀 INTENTANDO COMPILACIÓN ACTUAL:"
echo "================================"

# Verificar si compila actualmente
if cargo check 2>&1; then
    echo "✅ COMPILACIÓN EXITOSA - ESTADO BUENO"
else
    echo "❌ ERRORES DE COMPILACIÓN DETECTADOS"
fi

echo ""
echo "🧪 INTENTANDO EJECUCIÓN:"
echo "======================="

if cargo run --help 2>&1; then
    echo "✅ BINARIO EJECUTABLE"
else
    echo "❌ NO SE PUEDE EJECUTAR"
fi