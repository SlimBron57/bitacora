#!/bin/bash

# ===============================================
# BACKUP PROJECT SCRIPT
# Creates optimized backup of bitacora_v1.0 and ROADMAP
# Date: $(date +%Y%m%d_%H%M%S)
# ===============================================

set -e  # Exit on any error

# Configuration
BACKUP_DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="bitacora_essential_backup_${BACKUP_DATE}"
BASE_DIR="/home/edgi/Documents/Development/own/bitacora"
TEMP_DIR="/tmp/${BACKUP_NAME}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create temporary directory
log "Creating temporary directory: ${TEMP_DIR}"
mkdir -p "${TEMP_DIR}"
cd "${TEMP_DIR}"

# ===============================================
# BACKUP BITACORA_V1.0 (ESSENTIAL CODE ONLY)
# ===============================================

log "Backing up bitacora_v1.0 essential code..."

mkdir -p "bitacora_v1.0"
cd "bitacora_v1.0"

# Copy essential Rust project files
log "Copying Rust project structure..."
cp "${BASE_DIR}/bitacora_v1.0/Cargo.toml" . 2>/dev/null || warning "Cargo.toml not found"
cp "${BASE_DIR}/bitacora_v1.0/Cargo.lock" . 2>/dev/null || warning "Cargo.lock not found"

# Copy essential documentation
log "Copying essential documentation..."
for doc in "README.md" "CHECKLIST.md" "CHECKLIST_TREE.md" "CODE_VALIDATION.md" "TGUIDE.md"; do
    if [ -f "${BASE_DIR}/bitacora_v1.0/${doc}" ]; then
        cp "${BASE_DIR}/bitacora_v1.0/${doc}" .
        log "✓ Copied ${doc}"
    fi
done

# Copy src directory (excluding large generated files)
if [ -d "${BASE_DIR}/bitacora_v1.0/src" ]; then
    log "Copying src directory..."
    mkdir -p "src"
    
    # Copy main source files
    find "${BASE_DIR}/bitacora_v1.0/src" -name "*.rs" -type f | while read -r file; do
        relative_path=${file#"${BASE_DIR}/bitacora_v1.0/src/"}
        target_dir=$(dirname "src/${relative_path}")
        mkdir -p "${target_dir}"
        cp "${file}" "src/${relative_path}"
        log "✓ Copied src/${relative_path}"
    done
    
    # Copy essential markdown files in src
    find "${BASE_DIR}/bitacora_v1.0/src" -name "*.md" -type f | while read -r file; do
        relative_path=${file#"${BASE_DIR}/bitacora_v1.0/src/"}
        target_dir=$(dirname "src/${relative_path}")
        mkdir -p "${target_dir}"
        cp "${file}" "src/${relative_path}"
        log "✓ Copied src/${relative_path}"
    done
    
    success "Source code copied successfully"
else
    warning "src directory not found in bitacora_v1.0"
fi

# Copy examples directory (only .rs files)
if [ -d "${BASE_DIR}/bitacora_v1.0/examples" ]; then
    log "Copying examples..."
    mkdir -p "examples"
    find "${BASE_DIR}/bitacora_v1.0/examples" -name "*.rs" -type f -exec cp {} "examples/" \;
    success "Examples copied"
fi

# Copy tests directory (only .rs files)
if [ -d "${BASE_DIR}/bitacora_v1.0/tests" ]; then
    log "Copying tests..."
    mkdir -p "tests"
    find "${BASE_DIR}/bitacora_v1.0/tests" -name "*.rs" -type f -exec cp {} "tests/" \;
    success "Tests copied"
fi

# Back to temp directory
cd "${TEMP_DIR}"

# ===============================================
# BACKUP ROADMAP (DOCUMENTATION ONLY)
# ===============================================

log "Backing up ROADMAP documentation..."

if [ -d "${BASE_DIR}/ROADMAP" ]; then
    # Copy entire ROADMAP structure but only essential files
    rsync -av --include="*/" \
             --include="*.md" \
             --include="*.txt" \
             --include="*.json" \
             --exclude="*" \
             "${BASE_DIR}/ROADMAP/" "ROADMAP/"
    
    success "ROADMAP documentation copied"
else
    warning "ROADMAP directory not found"
fi

# ===============================================
# CREATE BACKUP ARCHIVE
# ===============================================

log "Creating backup archive..."

# Create zip file in the original bitacora directory
cd "${TEMP_DIR}"
ZIP_FILE="${BASE_DIR}/${BACKUP_NAME}.zip"

# Create zip with compression
zip -r -9 "${ZIP_FILE}" . \
    -x "*.git*" "*/target/*" "*/node_modules/*" "*.tmp" "*.log" \
    > /dev/null 2>&1

# ===============================================
# CLEANUP AND SUMMARY
# ===============================================

# Get file sizes
if [ -f "${ZIP_FILE}" ]; then
    ZIP_SIZE=$(du -h "${ZIP_FILE}" | cut -f1)
    success "Backup created successfully: ${BACKUP_NAME}.zip (${ZIP_SIZE})"
    
    # Show contents summary
    echo ""
    echo "=== BACKUP SUMMARY ==="
    echo "Backup file: ${ZIP_FILE}"
    echo "Size: ${ZIP_SIZE}"
    echo "Date: $(date)"
    echo ""
    echo "Contents:"
    echo "├── bitacora_v1.0/"
    echo "│   ├── Cargo.toml & Cargo.lock"
    echo "│   ├── Essential documentation (*.md)"
    echo "│   ├── src/ (Rust source files only)"
    echo "│   ├── examples/ (Rust examples)"
    echo "│   └── tests/ (Rust tests)"
    echo "└── ROADMAP/"
    echo "    └── Complete documentation structure"
    echo ""
    
    # Show excluded items
    echo "=== EXCLUDED FROM BACKUP ==="
    echo "• target/ directory (Rust build artifacts)"
    echo "• Large binary files"
    echo "• Git history (.git/)"
    echo "• Log files (*.log)"
    echo "• Temporary files (*.tmp)"
    echo ""
    
else
    error "Failed to create backup file"
    exit 1
fi

# Cleanup temporary directory
log "Cleaning up temporary files..."
rm -rf "${TEMP_DIR}"

success "Backup completed successfully!"
echo "Backup location: ${ZIP_FILE}"
echo ""
echo "To restore, extract the zip file:"
echo "  unzip ${BACKUP_NAME}.zip"
echo ""