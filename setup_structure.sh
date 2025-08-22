#!/bin/bash
# Script para crear la estructura completa de directorios del workspace Bitacora V1.0
set -e

echo "🏗️  Creando estructura de directorios para Bitacora V1.0..."

# Crear directorios restantes para API
echo "📁 Creando bitacora-api..."
mkdir -p crates/bitacora-api/src/{server,handlers,middleware,dto,documentation}

# Crear directorios para admin
echo "📁 Creando bitacora-admin..."
mkdir -p crates/bitacora-admin/src/{commands,config,database,health,users,audit}

# Crear directorios para configuración
echo "📁 Creando bitacora-config..."
mkdir -p crates/bitacora-config/src

# Crear directorios de scripts de desarrollo
echo "📁 Creando scripts de desarrollo..."
mkdir -p scripts

# Crear directorios para Docker y CI/CD
echo "📁 Creando directorios de deployment..."
mkdir -p .github/workflows
mkdir -p docker
mkdir -p config/{dev,staging,prod}

# Crear directorios para documentación
echo "📁 Creando directorios de documentación..."
mkdir -p docs/{api,development,deployment,migration}

# Crear directorios para tests
echo "📁 Creando directorios de testing..."
mkdir -p tests/{integration,e2e,fixtures}

# Crear directorios para migraciones
echo "📁 Creando directorios de migración..."
mkdir -p migrations/{scripts,data,validation}

# Crear directorios para logs y cache
echo "📁 Creando directorios de runtime..."
mkdir -p logs
mkdir -p cache
mkdir -p tmp

echo "✅ Estructura de directorios creada exitosamente!"
echo ""
echo "📊 Resumen de la estructura creada:"
tree crates/ -d -L 3 2>/dev/null || find crates/ -type d | head -20
