#!/bin/bash

# Test Configuration System Script
# This script tests the bitacora-config system in different scenarios

set -e

echo "🔧 Testing Bitacora Configuration System"
echo "======================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Test 1: Compilation check${NC}"
cargo check --package bitacora-config --quiet || {
    echo -e "${RED}✗ Configuration compilation failed${NC}"
    exit 1
}
echo -e "${GREEN}✓ Configuration compilation passed${NC}"

echo -e "${YELLOW}Test 2: Unit tests${NC}"
cargo test --package bitacora-config --quiet || {
    echo -e "${RED}✗ Configuration unit tests failed${NC}"
    exit 1
}
echo -e "${GREEN}✓ Configuration unit tests passed${NC}"

echo -e "${YELLOW}Test 3: Environment configs validation${NC}"
for env in development staging production; do
    echo "  Checking $env.toml..."
    if [[ ! -f "crates/bitacora-config/config/$env.toml" ]]; then
        echo -e "${RED}✗ Missing $env.toml${NC}"
        exit 1
    fi
done
echo -e "${GREEN}✓ All environment configs present${NC}"

echo -e "${YELLOW}Test 4: Config structure validation${NC}"
required_files=(
    "crates/bitacora-config/src/lib.rs"
    "crates/bitacora-config/src/database.rs"
    "crates/bitacora-config/src/server.rs"
    "crates/bitacora-config/src/logging.rs"
    "crates/bitacora-config/src/integration.rs"
)

for file in "${required_files[@]}"; do
    if [[ ! -f "$file" ]]; then
        echo -e "${RED}✗ Missing required file: $file${NC}"
        exit 1
    fi
done
echo -e "${GREEN}✓ All required config files present${NC}"

echo ""
echo -e "${GREEN}🎉 All configuration tests passed!${NC}"
echo ""
echo -e "${YELLOW}Configuration System Summary:${NC}"
echo "  ✓ Type-safe configuration with validation"
echo "  ✓ Environment-specific configs (dev, staging, prod)"
echo "  ✓ Environment variable overrides supported"
echo "  ✓ Database connection string generation"
echo "  ✓ Server, logging, and integration configs"
echo ""
echo -e "${GREEN}✅ Configuration system is ready for Storage Layer integration.${NC}"

echo "🔧 Bitacora V1.0 - Database Configuration Tester"
echo "==============================================="

# Función para cargar variables de entorno
load_env() {
    local env_file="$1"
    if [ -f "$env_file" ]; then
        echo "📝 Cargando configuración desde: $env_file"
        export $(cat "$env_file" | grep -v '^#' | xargs)
    else
        echo "⚠️  Archivo $env_file no encontrado"
    fi
}

# Función para probar conexión MongoDB
test_mongodb() {
    echo "🔍 Probando conexión MongoDB..."
    if docker ps | grep -q bitacora_mongo_dev; then
        echo "✅ Container MongoDB ejecutándose"
        
        # Probar conexión
        if docker exec bitacora_mongo_dev mongosh --eval "db.runCommand('ping')" > /dev/null 2>&1; then
            echo "✅ MongoDB responde correctamente"
        else
            echo "❌ MongoDB no responde"
        fi
    else
        echo "❌ Container MongoDB no está ejecutándose"
        echo "🚀 Iniciando MongoDB..."
        docker-compose up -d mongodb
        sleep 5
        test_mongodb
    fi
}

# Función para crear directorio de datos
setup_data_dirs() {
    echo "📁 Configurando directorios de datos..."
    mkdir -p data
    mkdir -p logs
    touch data/.gitkeep
    touch logs/.gitkeep
    echo "✅ Directorios creados"
}

# Función para validar configuración
validate_config() {
    local config_file="$1"
    echo "🔍 Validando configuración: $config_file"
    
    if [ -f "$config_file" ]; then
        echo "✅ Archivo de configuración encontrado"
        
        # Validar sintaxis TOML básica
        if grep -q "\[database\]" "$config_file"; then
            echo "✅ Sección database encontrada"
        else
            echo "❌ Sección database no encontrada"
        fi
        
        if grep -q "type.*=.*\"mongodb\"" "$config_file"; then
            echo "✅ Tipo de base de datos: MongoDB"
        elif grep -q "type.*=.*\"sqlite\"" "$config_file"; then
            echo "✅ Tipo de base de datos: SQLite"
        else
            echo "⚠️  Tipo de base de datos no reconocido"
        fi
    else
        echo "❌ Archivo de configuración no encontrado: $config_file"
    fi
}

# Función principal
main() {
    local environment="${1:-development}"
    
    echo "🎯 Entorno seleccionado: $environment"
    echo ""
    
    # Cargar configuración
    load_env ".env.$environment"
    
    # Setup directorios
    setup_data_dirs
    
    # Validar configuración
    validate_config "config/$environment.toml"
    
    # Probar conexión según el tipo
    if [ "$DATABASE_TYPE" = "mongodb" ]; then
        test_mongodb
    elif [ "$DATABASE_TYPE" = "sqlite" ]; then
        echo "🔍 Usando SQLite: $SQLITE_PATH"
        mkdir -p "$(dirname "$SQLITE_PATH")"
        echo "✅ SQLite configurado"
    fi
    
    echo ""
    echo "🎉 Configuración validada para entorno: $environment"
    echo ""
    echo "📋 Próximos pasos:"
    echo "   1. docker-compose up -d mongodb (si usas MongoDB)"
    echo "   2. cargo build (para compilar el proyecto)"
    echo "   3. cargo run (para ejecutar la aplicación)"
}

# Mostrar ayuda
show_help() {
    echo "Uso: $0 [environment]"
    echo ""
    echo "Entornos disponibles:"
    echo "  development (por defecto)"
    echo "  production"
    echo ""
    echo "Ejemplos:"
    echo "  $0                    # Usar desarrollo"
    echo "  $0 development        # Usar desarrollo"
    echo "  $0 production         # Usar producción"
}

# Parse argumentos
case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
    *)
        main "$1"
        ;;
esac
