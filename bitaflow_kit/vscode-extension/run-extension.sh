#!/bin/bash
# Script para ejecutar la extensión BitaFlow en modo desarrollo

echo "🚀 Ejecutando extensión BitaFlow..."
cd /home/edgi/Documents/Development/own/bitacora/bitaflow_kit/vscode-extension
code --extensionDevelopmentPath=$(pwd) --new-window
echo "✅ Extensión ejecutándose en nueva ventana de VS Code"
