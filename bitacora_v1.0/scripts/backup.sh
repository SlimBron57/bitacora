#!/bin/bash
#
# 🔐 BITÁCORA - SCRIPT DE BACKUP OPTIMIZADO
# 
# Propósito: Backup eficiente solo de archivos esenciales (sin basura)
# Mejoras: Validación pre-backup, limpieza, verificación post-backup
#
# Autor: Eduardo Gil (Vangijroc)
# Fecha creación: 2025-10-29
# Última actualización: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

set -e  # Exit on error

# ============================================================================
# CONFIGURACIÓN
# ============================================================================

PROJECT_ROOT="/home/edgi/Documents/Development/own/bitacora/bitacora_v1.0"
BACKUP_BASE_DIR="/home/edgi/Documents/Development/own/bitacora/00_BACKUPS"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="${BACKUP_BASE_DIR}/BACKUP_COMPLETO_${TIMESTAMP}"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# ============================================================================
# FUNCIONES
# ============================================================================

print_header() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║       🔐 BITÁCORA - BACKUP OPTIMIZADO (SIN BASURA) 🔐        ║"
    echo "║                                                                ║"
    echo "║  ✅ Solo código fuente + docs + git history                   ║"
    echo "║  ❌ NO binarios, NO logs, NO temporales                       ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_step() {
    echo -e "\n${BLUE}[PASO $1/11]${NC} ${GREEN}$2${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✅ ${NC}$1"
}

print_warning() {
    echo -e "${YELLOW}⚠️  ${NC}$1"
}

print_error() {
    echo -e "${RED}❌ ${NC}$1"
}

human_readable_size() {
    du -sh "$1" 2>/dev/null | cut -f1
}

# ============================================================================
# AUTO-COMMIT DE CAMBIOS PENDIENTES
# ============================================================================

auto_commit_changes() {
    print_info "🔄 Verificando cambios pendientes en git..."
    
    cd "$PROJECT_ROOT" || {
        print_error "No se puede acceder al directorio del proyecto"
        exit 1
    }
    
    # Verificar si hay cambios
    if [[ -n $(git status --porcelain) ]]; then
        print_info "📝 Cambios detectados, creando commit automático..."
        
        # Agregar todos los cambios
        git add .
        
        # Crear commit con timestamp
        COMMIT_MSG="auto-backup: snapshot before backup $(date +'%Y-%m-%d %H:%M:%S')"
        git commit -m "$COMMIT_MSG"
        
        print_success "Commit automático creado: $COMMIT_MSG"
        
        # Intentar push (no fallar si hay problemas de red)
        if git push origin $(git branch --show-current) 2>/dev/null; then
            print_success "Push a GitHub exitoso"
        else
            print_warning "No se pudo hacer push a GitHub (continuando con backup local)"
        fi
    else
        print_info "✨ No hay cambios pendientes, repositorio limpio"
    fi
}

# ============================================================================
# VALIDACIONES PRE-BACKUP
# ============================================================================

validate_environment() {
    print_step 1 "Validando entorno..."
    
    # Verificar que estamos en el directorio correcto
    if [ ! -d "$PROJECT_ROOT" ]; then
        print_error "Directorio del proyecto no encontrado: $PROJECT_ROOT"
        exit 1
    fi
    
    # Verificar que existe Cargo.toml
    if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
        print_error "Cargo.toml no encontrado. ¿Estás en el proyecto correcto?"
        exit 1
    fi
    
    # Verificar espacio en disco
    AVAILABLE_SPACE=$(df "$BACKUP_BASE_DIR" | awk 'NR==2 {print $4}')
    if [ "$AVAILABLE_SPACE" -lt 1048576 ]; then  # < 1GB
        print_warning "Espacio disponible bajo: $(df -h "$BACKUP_BASE_DIR" | awk 'NR==2 {print $4}')"
        read -p "¿Continuar? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    
    print_success "Validación completa"
}

# ============================================================================
# ANÁLISIS PRE-BACKUP
# ============================================================================

analyze_project() {
    print_step 2 "Analizando proyecto..."
    
    cd "$PROJECT_ROOT"
    
    # Tamaños
    TOTAL_SIZE=$(du -sh . 2>/dev/null | cut -f1)
    TARGET_SIZE=$(du -sh target 2>/dev/null | cut -f1 || echo "0")
    
    # Contadores
    RS_FILES=$(find . -name "*.rs" -not -path "./target/*" -not -path "./B20250915-data-compressor/target/*" | wc -l)
    MD_FILES=$(find . -name "*.md" -not -path "./target/*" | wc -l)
    TOTAL_RS_LINES=$(find . -name "*.rs" -not -path "./target/*" -not -path "./B20250915-data-compressor/target/*" -exec wc -l {} + 2>/dev/null | tail -n 1 | awk '{print $1}' || echo "0")
    
    print_info "Tamaño total: $TOTAL_SIZE"
    print_info "Directorio target/: $TARGET_SIZE (será EXCLUIDO)"
    print_info "Archivos Rust (.rs): $RS_FILES"
    print_info "Archivos Markdown (.md): $MD_FILES"
    print_info "Líneas de código Rust: $TOTAL_RS_LINES"
    
    print_success "Análisis completo"
}

# ============================================================================
# LIMPIEZA PRE-BACKUP (OPCIONAL)
# ============================================================================

cleanup_before_backup() {
    print_step 3 "Limpiando archivos temporales..."
    
    cd "$PROJECT_ROOT"
    
    # Buscar y reportar archivos temporales
    TEMP_FILES=$(find . -type f \( -name "*.log" -o -name "*.tmp" -o -name "*~" \) -not -path "./target/*" 2>/dev/null | wc -l)
    
    if [ "$TEMP_FILES" -gt 0 ]; then
        print_warning "Se encontraron $TEMP_FILES archivos temporales"
        find . -type f \( -name "*.log" -o -name "*.tmp" -o -name "*~" \) -not -path "./target/*" 2>/dev/null | head -10
        
        read -p "¿Eliminar antes del backup? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            find . -type f \( -name "*.log" -o -name "*.tmp" -o -name "*~" \) -not -path "./target/*" -delete 2>/dev/null
            print_success "Archivos temporales eliminados"
        else
            print_info "Archivos temporales serán excluidos del backup"
        fi
    else
        print_success "No hay archivos temporales"
    fi
}

# ============================================================================
# CREAR BACKUP
# ============================================================================

create_backup_structure() {
    print_step 4 "Creando estructura de backup..."
    
    mkdir -p "$BACKUP_DIR"
    mkdir -p "$BACKUP_DIR/codigo_fuente"
    mkdir -p "$BACKUP_DIR/git_history"
    mkdir -p "$BACKUP_DIR/documentacion"
    mkdir -p "$BACKUP_DIR/evidencia_legal"
    mkdir -p "$BACKUP_DIR/metadata"
    
    print_success "Estructura creada en: $BACKUP_DIR"
}

backup_source_code() {
    print_step 5 "Respaldando código fuente..."
    
    cd "$PROJECT_ROOT"
    
    # Rsync con exclusiones optimizadas
    rsync -av \
        --exclude 'target/' \
        --exclude '**/target/' \
        --exclude 'node_modules/' \
        --exclude '.git/' \
        --exclude '**/.git/' \
        --exclude '00_BACKUPS/' \
        --exclude 'tmp/' \
        --exclude 'temp/' \
        --exclude '*.log' \
        --exclude '*.tmp' \
        --exclude '*~' \
        --exclude '.DS_Store' \
        --exclude 'Thumbs.db' \
        --exclude '*.tar.gz' \
        --exclude '*.zip' \
        --exclude '.vscode/' \
        --exclude '.idea/' \
        --progress \
        "$PROJECT_ROOT/" "$BACKUP_DIR/codigo_fuente/"
    
    SOURCE_SIZE=$(human_readable_size "$BACKUP_DIR/codigo_fuente")
    print_success "Código fuente respaldado: $SOURCE_SIZE"
}

backup_git_history() {
    print_step 6 "Exportando historial de Git..."
    
    cd "$PROJECT_ROOT"
    
    # Git log completo
    git log --all --pretty=fuller --date=iso --reverse > "$BACKUP_DIR/git_history/git_log_completo.txt" 2>/dev/null || true
    
    # Git log con diffs
    git log --all --patch --reverse > "$BACKUP_DIR/git_history/git_log_con_diffs.txt" 2>/dev/null || true
    
    # Estadísticas
    git shortlog -sn --all > "$BACKUP_DIR/git_history/git_stats_contributors.txt" 2>/dev/null || true
    git log --all --pretty=format:"%h|%an|%ae|%ad|%s" --date=iso > "$BACKUP_DIR/git_history/git_log_csv.txt" 2>/dev/null || true
    
    # Bundle completo del repositorio
    git bundle create "$BACKUP_DIR/git_history/bitacora_repo.bundle" --all 2>/dev/null || true
    
    # 🆕 COPIA COMPLETA DEL DIRECTORIO .git (NUEVA FUNCIONALIDAD v1.5)
    print_info "Copiando directorio .git completo..."
    if [ -d "/home/edgi/Documents/Development/own/bitacora/.git" ]; then
        cp -r /home/edgi/Documents/Development/own/bitacora/.git "$BACKUP_DIR/git_history/dot_git_backup/" 2>/dev/null || true
        print_success "Directorio .git respaldado (permite restauración completa del repo)"
    else
        print_warning "Directorio .git no encontrado"
    fi
    
    GIT_SIZE=$(human_readable_size "$BACKUP_DIR/git_history")
    print_success "Git history exportado: $GIT_SIZE"
}

backup_documentation() {
    print_step 7 "Respaldando documentación..."
    
    cd "$PROJECT_ROOT"
    
    # Copiar directorios de documentación
    for dir in ROADMAP_V2 FUSION_BAYESIANA PUBLIC RECREO_CON_MI_COMPANERO; do
        if [ -d "$dir" ]; then
            cp -r "$dir" "$BACKUP_DIR/documentacion/" 2>/dev/null || true
        fi
    done
    
    # Archivos markdown en raíz
    find . -maxdepth 1 -name "*.md" -exec cp {} "$BACKUP_DIR/documentacion/" \; 2>/dev/null || true
    
    # Cargo.toml y archivos de configuración importantes
    cp Cargo.toml "$BACKUP_DIR/documentacion/" 2>/dev/null || true
    cp Cargo.lock "$BACKUP_DIR/documentacion/" 2>/dev/null || true
    
    DOCS_SIZE=$(human_readable_size "$BACKUP_DIR/documentacion")
    print_success "Documentación respaldada: $DOCS_SIZE"
}

generate_metadata() {
    print_step 8 "Generando metadata legal..."
    
    cd "$PROJECT_ROOT"
    
    cat > "$BACKUP_DIR/evidencia_legal/METADATA_BACKUP.txt" << EOF
═══════════════════════════════════════════════════════════════
  🜛BITÁCORA - METADATA DE BACKUP OPTIMIZADO
═══════════════════════════════════════════════════════════════

Fecha de Backup (UTC): $(date -u +"%Y-%m-%d %H:%M:%S %Z")
Fecha de Backup (Local): $(date +"%Y-%m-%d %H:%M:%S %Z")
Timestamp Unix: $(date +%s)

Autor: Eduardo Gil (Vangijroc)
Proyecto: Bitácora v1.0 - Sistema de Inteligencia Contextual

═══════════════════════════════════════════════════════════════
  INFORMACIÓN DEL REPOSITORIO
═══════════════════════════════════════════════════════════════

Commit actual: $(git rev-parse HEAD 2>/dev/null || echo "N/A")
Branch actual: $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "N/A")
Total commits: $(git rev-list --all --count 2>/dev/null || echo "N/A")
Primer commit: $(git log --reverse --format="%H %ai %s" 2>/dev/null | head -n 1 || echo "N/A")
Último commit: $(git log -1 --format="%H %ai %s" 2>/dev/null || echo "N/A")

Autores del proyecto:
$(git shortlog -sn --all 2>/dev/null || echo "N/A")

═══════════════════════════════════════════════════════════════
  ESTADÍSTICAS DEL PROYECTO
═══════════════════════════════════════════════════════════════

Archivos Rust (.rs): $(find "$BACKUP_DIR/codigo_fuente" -name "*.rs" | wc -l)
Archivos Markdown (.md): $(find "$BACKUP_DIR/codigo_fuente" -name "*.md" | wc -l)
Total líneas de código Rust: $(find "$BACKUP_DIR/codigo_fuente" -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -n 1 | awk '{print $1}' || echo "N/A")
Total líneas de documentación: $(find "$BACKUP_DIR/codigo_fuente" -name "*.md" -exec wc -l {} + 2>/dev/null | tail -n 1 | awk '{print $1}' || echo "N/A")

═══════════════════════════════════════════════════════════════
  TAMAÑO DEL BACKUP (SIN BASURA)
═══════════════════════════════════════════════════════════════

Código fuente: $(human_readable_size "$BACKUP_DIR/codigo_fuente")
Git history: $(human_readable_size "$BACKUP_DIR/git_history")
Documentación: $(human_readable_size "$BACKUP_DIR/documentacion")
Total backup: $(human_readable_size "$BACKUP_DIR")

Exclusiones aplicadas:
  ❌ target/ (binarios compilados)
  ❌ node_modules/ (dependencias JS)
  ❌ *.log, *.tmp, *~ (temporales)
  ❌ .git/ (preservado en bundle)
  ❌ 00_BACKUPS/ (evitar recursión)

═══════════════════════════════════════════════════════════════
  PROPÓSITO LEGAL
═══════════════════════════════════════════════════════════════

Este backup optimizado sirve como evidencia de:
1. Fecha de invención y desarrollo del sistema Bitácora
2. Autoría de Eduardo Gil (Vangijroc)
3. Prior art para protección de propiedad intelectual
4. Historial completo de desarrollo y evolución del proyecto

IMPORTANTE: Este backup contiene SOLO archivos esenciales.
NO incluye basura ni binarios compilados.

═══════════════════════════════════════════════════════════════

EOF

    print_success "Metadata generada"
}

generate_hashes() {
    print_step 9 "Generando hashes SHA-256..."
    
    cd "$BACKUP_DIR"
    find . -type f -exec sha256sum {} \; | sort > "./evidencia_legal/SHA256SUMS.txt"
    
    HASH_COUNT=$(wc -l < "./evidencia_legal/SHA256SUMS.txt")
    print_success "Hashes generados: $HASH_COUNT archivos"
}

compress_backup() {
    print_step 10 "Comprimiendo backup..."
    
    cd "$BACKUP_BASE_DIR"
    ARCHIVE_NAME="BITACORA_BACKUP_${TIMESTAMP}.tar.gz"
    
    tar -czf "$ARCHIVE_NAME" "BACKUP_COMPLETO_${TIMESTAMP}/" --checkpoint=.1000
    
    ARCHIVE_SIZE=$(human_readable_size "$ARCHIVE_NAME")
    print_success "Archivo comprimido: $ARCHIVE_NAME ($ARCHIVE_SIZE)"
    
    # Generar hash del archivo (se guardará en el reporte)
    ARCHIVE_HASH=$(sha256sum "$ARCHIVE_NAME" | cut -d' ' -f1)
    print_success "Hash SHA-256: ${ARCHIVE_HASH:0:16}..."
    
    # Limpiar directorio temporal (todo está en el .tar.gz)
    print_info "Limpiando directorio temporal..."
    rm -rf "BACKUP_COMPLETO_${TIMESTAMP}/"
    print_success "Directorio temporal eliminado (todo en .tar.gz)"
}

verify_backup() {
    print_step 11 "Verificando integridad del backup..."
    
    cd "$BACKUP_BASE_DIR"
    ARCHIVE_NAME="BITACORA_BACKUP_${TIMESTAMP}.tar.gz"
    
    # Test de integridad del archivo tar.gz (sin extraer)
    if tar -tzf "$ARCHIVE_NAME" > /dev/null 2>&1; then
        print_success "Verificación de archivo tar: OK"
    else
        print_error "Archivo tar corrupto"
        exit 1
    fi
    
    # Calcular tamaño comprimido
    COMPRESSED_SIZE=$(du -sh "$ARCHIVE_NAME" | cut -f1)
    
    print_info "Tamaño del backup: $COMPRESSED_SIZE"
    print_success "Backup verificado correctamente"
}

# ============================================================================
# REPORTE FINAL
# ============================================================================

generate_final_report() {
    cd "$BACKUP_BASE_DIR"
    ARCHIVE_NAME="BITACORA_BACKUP_${TIMESTAMP}.tar.gz"
    
    # Generar hash final del archivo
    ARCHIVE_HASH=$(sha256sum "$ARCHIVE_NAME" | cut -d' ' -f1)
    
    cat > "$BACKUP_BASE_DIR/REPORTE_BACKUP_${TIMESTAMP}.txt" << EOF
═══════════════════════════════════════════════════════════════
  🔐 BITÁCORA - REPORTE DE BACKUP OPTIMIZADO
═══════════════════════════════════════════════════════════════

Fecha: $(date +"%Y-%m-%d %H:%M:%S %Z")
Autor: Eduardo Gil (Vangijroc)

═══════════════════════════════════════════════════════════════
  ARCHIVOS GENERADOS
═══════════════════════════════════════════════════════════════

📦 Archivo comprimido (TODO incluido):
   $BACKUP_BASE_DIR/$ARCHIVE_NAME
   Tamaño: $(human_readable_size "$ARCHIVE_NAME")

🔐 Hash SHA-256 del archivo:
   $ARCHIVE_HASH

═══════════════════════════════════════════════════════════════
  CONTENIDO DEL BACKUP (SIN BASURA)
═══════════════════════════════════════════════════════════════

✅ Código fuente completo (sin target/)
✅ Historial completo de Git (bundle + logs)
✅ Documentación (ROADMAP_V2, FUSION_BAYESIANA, etc.)
✅ Evidencia legal (metadata, hashes)
✅ Cargo.toml + Cargo.lock

❌ Binarios compilados (target/)
❌ Logs y temporales
❌ Archivos de IDE
❌ Node modules (si existiera)

═══════════════════════════════════════════════════════════════
  ESTADÍSTICAS
═══════════════════════════════════════════════════════════════

Tamaño del backup: $(human_readable_size "$ARCHIVE_NAME")

NOTA: El directorio temporal fue eliminado.
      TODO el contenido está dentro del archivo .tar.gz

═══════════════════════════════════════════════════════════════
  PRÓXIMOS PASOS RECOMENDADOS
═══════════════════════════════════════════════════════════════

1. 💾 Copiar $ARCHIVE_NAME a USB #1 (local)
2. 💾 Copiar a USB #2 (casa padres en Colombia)
3. ☁️  Subir directamente a Google Drive/Dropbox
4. 📝 Generar OpenTimestamp (opcional):
   ./scripts/opentimestamp.sh (cuando esté listo)
5. 📋 Documentar fecha en registro de patentes

═══════════════════════════════════════════════════════════════
  COMANDOS ÚTILES
═══════════════════════════════════════════════════════════════

# Verificar integridad:
sha256sum $ARCHIVE_NAME
# Debe coincidir con: $ARCHIVE_HASH

# Listar contenido sin extraer:
tar -tzf $ARCHIVE_NAME | less

# Extraer backup completo:
tar -xzf $ARCHIVE_NAME

# Restaurar repositorio Git:
tar -xzf $ARCHIVE_NAME
cd BACKUP_COMPLETO_${TIMESTAMP}/git_history/
git clone bitacora_repo.bundle bitacora_restored

═══════════════════════════════════════════════════════════════

EOF

    print_success "Reporte generado: REPORTE_BACKUP_${TIMESTAMP}.txt"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

print_header

# NUEVO: Auto-commit ANTES de todo
auto_commit_changes

validate_environment
analyze_project
cleanup_before_backup
create_backup_structure
backup_source_code
backup_git_history
backup_documentation
generate_metadata
generate_hashes
compress_backup
verify_backup
generate_final_report

# ============================================================================
# RESUMEN FINAL
# ============================================================================

echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║              ✅ BACKUP OPTIMIZADO COMPLETADO                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Calcular hash final
ARCHIVE_HASH=$(sha256sum "$BACKUP_BASE_DIR/BITACORA_BACKUP_${TIMESTAMP}.tar.gz" | cut -d' ' -f1)

echo -e "${GREEN}📦 Archivo:${NC} BITACORA_BACKUP_${TIMESTAMP}.tar.gz"
echo -e "${GREEN}📍 Ubicación:${NC} $BACKUP_BASE_DIR"
echo -e "${GREEN}💾 Tamaño:${NC} $(human_readable_size "$BACKUP_BASE_DIR/BITACORA_BACKUP_${TIMESTAMP}.tar.gz")"
echo -e "${GREEN}🔐 Hash:${NC} ${ARCHIVE_HASH:0:32}..."
echo ""
echo -e "${MAGENTA}🎯 CARACTERÍSTICAS:${NC}"
echo -e "   ✅ TODO incluido en un solo archivo .tar.gz"
echo -e "   ✅ NO incluye binarios (target/ excluido)"
echo -e "   ✅ NO incluye logs ni temporales"
echo -e "   ✅ Verificación automática de integridad"
echo -e "   ✅ Sin archivos sueltos (directorio temporal eliminado)"
echo ""
echo -e "${YELLOW}📋 PRÓXIMOS PASOS:${NC}"
echo -e "   1. Copiar a USB #1 (local)"
echo -e "   2. Copiar a USB #2 (Colombia)"
echo -e "   3. Subir a nube (Google Drive/Dropbox)"
echo ""
echo -e "${BLUE}ℹ️  Reporte: REPORTE_BACKUP_${TIMESTAMP}.txt${NC}"
echo -e "${BLUE}ℹ️  Hash completo en el reporte${NC}"
echo ""