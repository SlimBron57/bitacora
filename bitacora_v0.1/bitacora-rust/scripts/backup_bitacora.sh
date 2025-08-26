#!/bin/bash
# ============================================================================
# Script: backup_bitacora.sh
# Funcionalidad: Create local backup of Bitacora Rust project
# Flujo: Generates ZIP backup excluding build artifacts and temporary files
# Uso: ./backup_bitacora.sh
# Fecha de creacion: 2025-08-24
# ============================================================================

set -e  # Exit on any error

# Configuration
PROJECT_NAME="bitacora"
BACKUP_DIR="/home/edgi/Backups"
TIMESTAMP=$(date +"%Y%m%d-%H%M")
BACKUP_FILENAME="${TIMESTAMP}_${PROJECT_NAME}.zip"
BACKUP_PATH="${BACKUP_DIR}/${BACKUP_FILENAME}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_step() {
    echo -e "${CYAN}[STEP]${NC} $1"
}

# Function to create backup directory if it doesn't exist
create_backup_directory() {
    if [ ! -d "$BACKUP_DIR" ]; then
        log_info "Creating backup directory: $BACKUP_DIR"
        mkdir -p "$BACKUP_DIR"
    fi
}

# Function to get project root directory
get_project_root() {
    if [ -f "Cargo.toml" ]; then
        echo "$(pwd)"
    else
        log_error "Cargo.toml not found. Please run this script from the project root directory."
        exit 1
    fi
}

# Function to preserve directory structures that should remain empty
preserve_empty_structures() {
    local temp_dir="/tmp/backup_empty_structures_$$"
    log_step "Preserving empty directory structures..."
    
    # Directories that should be preserved even if empty
    local preserve_dirs=(
        "cache"
        "data"
        "logs"
        "tmp"
        "migrations/data"
        "migrations/scripts"
        "migrations/validation"
    )
    
    mkdir -p "$temp_dir"
    
    for dir in "${preserve_dirs[@]}"; do
        if [ -d "$dir" ]; then
            # Create directory structure
            mkdir -p "$temp_dir/$dir"
            # Create .gitkeep to preserve empty directories
            touch "$temp_dir/$dir/.gitkeep"
            log_info "Preserved empty structure: $dir"
        fi
    done
    
    echo "$temp_dir"
}

# Function to create the backup
create_backup() {
    local project_root="$1"
    local temp_empty_structures="$2"
    
    log_step "Creating backup: $BACKUP_FILENAME"
    
    # Create temporary directory for backup preparation
    local temp_backup_dir="/tmp/backup_prep_$$"
    mkdir -p "$temp_backup_dir"
    
    cd "$project_root"
    
    # Copy project files excluding build artifacts and temporary content
    log_info "Copying project files..."
    
    # Use rsync for efficient copying with exclusions
    rsync -av \
        --exclude='target/' \
        --exclude='cache/*' \
        --exclude='logs/*.log' \
        --exclude='tmp/*' \
        --exclude='data/*' \
        --exclude='migrations/data/*' \
        --exclude='migrations/scripts/*' \
        --exclude='migrations/validation/*' \
        --exclude='*.log' \
        --exclude='*.tmp' \
        --exclude='.git/' \
        --exclude='node_modules/' \
        --exclude='*.zip' \
        --exclude='*.tar.gz' \
        --exclude='*.bak' \
        --exclude='*~' \
        --exclude='.DS_Store' \
        --exclude='Thumbs.db' \
        --exclude='.env' \
        --exclude='.env.local' \
        ./ "$temp_backup_dir/"
    
    # Copy preserved empty structures
    if [ -d "$temp_empty_structures" ]; then
        log_info "Restoring empty directory structures..."
        cp -r "$temp_empty_structures"/* "$temp_backup_dir/" 2>/dev/null || true
    fi
    
    # Create the ZIP file
    log_info "Compressing backup..."
    cd "$temp_backup_dir"
    zip -r "$BACKUP_PATH" . > /dev/null 2>&1
    
    # Cleanup
    rm -rf "$temp_backup_dir"
    rm -rf "$temp_empty_structures"
    
    cd "$project_root"
}

# Function to calculate and display backup info
display_backup_info() {
    if [ -f "$BACKUP_PATH" ]; then
        local file_size=$(du -h "$BACKUP_PATH" | cut -f1)
        local file_count=$(unzip -l "$BACKUP_PATH" | tail -1 | awk '{print $2}')
        
        log_success "Backup created successfully!"
        echo ""
        echo "📁 Backup Details:"
        echo "   • Location: $BACKUP_PATH"
        echo "   • Size: $file_size"
        echo "   • Files: $file_count items"
        echo "   • Timestamp: $(date)"
        echo ""
        
        # List recent backups
        log_info "Recent backups in $BACKUP_DIR:"
        ls -lht "$BACKUP_DIR"/*.zip 2>/dev/null | head -5 | while read -r line; do
            echo "   $line"
        done
    else
        log_error "Backup file was not created successfully"
        exit 1
    fi
}

# Function to verify backup contents
verify_backup() {
    log_step "Verifying backup contents..."
    
    # Check if important files are included
    local important_files=(
        "Cargo.toml"
        "Cargo.lock"
        "src/"
        "crates/"
        "docs/"
        "scripts/"
        "config/"
        "_map/"
        "docker-compose.yml"
    )
    
    local verification_passed=true
    
    for file in "${important_files[@]}"; do
        if unzip -l "$BACKUP_PATH" | grep -q "$file"; then
            log_info "✅ $file included"
        else
            log_warning "⚠️  $file not found in backup"
            verification_passed=false
        fi
    done
    
    # Check that excluded files are not present
    local excluded_patterns=("target/" "cache/" "logs/")
    
    for pattern in "${excluded_patterns[@]}"; do
        if unzip -l "$BACKUP_PATH" | grep -q "$pattern.*[^/]$"; then
            log_warning "⚠️  Files from excluded pattern $pattern found in backup"
            verification_passed=false
        fi
    done
    
    if [ "$verification_passed" = true ]; then
        log_success "Backup verification passed"
    else
        log_warning "Backup verification completed with warnings"
    fi
}

# Function to show usage
show_usage() {
    echo ""
    echo "🚀 Bitácora - Project Backup Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --verify   Verify backup after creation"
    echo "  -q, --quiet    Quiet mode (less verbose output)"
    echo ""
    echo "Examples:"
    echo "  $0              # Create backup with default settings"
    echo "  $0 --verify     # Create backup and verify contents"
    echo "  $0 --quiet      # Create backup in quiet mode"
    echo ""
}

# Function to open backup directory in file manager
open_backup_directory() {
    log_info "Opening backup directory..."
    
    # Detect the desktop environment and open file manager accordingly
    if command -v xdg-open > /dev/null 2>&1; then
        # Linux with xdg-utils (most common)
        xdg-open "$BACKUP_DIR" 2>/dev/null &
        log_success "Backup directory opened in file manager"
    elif command -v nautilus > /dev/null 2>&1; then
        # GNOME file manager
        nautilus "$BACKUP_DIR" 2>/dev/null &
        log_success "Backup directory opened in Nautilus"
    elif command -v thunar > /dev/null 2>&1; then
        # XFCE file manager
        thunar "$BACKUP_DIR" 2>/dev/null &
        log_success "Backup directory opened in Thunar"
    elif command -v dolphin > /dev/null 2>&1; then
        # KDE file manager
        dolphin "$BACKUP_DIR" 2>/dev/null &
        log_success "Backup directory opened in Dolphin"
    elif command -v pcmanfm > /dev/null 2>&1; then
        # LXDE file manager
        pcmanfm "$BACKUP_DIR" 2>/dev/null &
        log_success "Backup directory opened in PCManFM"
    else
        log_warning "Could not detect file manager. Backup created at: $BACKUP_PATH"
        log_info "Manual path: $BACKUP_DIR"
    fi
}

# Main execution function
main() {
    local verify_backup_flag=false
    local quiet_mode=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--verify)
                verify_backup_flag=true
                shift
                ;;
            -q|--quiet)
                quiet_mode=true
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    # Redirect output if quiet mode
    if [ "$quiet_mode" = true ]; then
        exec 3>&1 4>&2 1>/dev/null 2>/dev/null
    fi
    
    echo ""
    echo "🚀 Starting Bitácora Project Backup"
    echo "===================================="
    echo ""
    
    # Get project root and validate
    local project_root
    project_root=$(get_project_root)
    log_info "Project root: $project_root"
    
    # Create backup directory
    create_backup_directory
    
    # Preserve empty directory structures
    local temp_empty_structures
    temp_empty_structures=$(preserve_empty_structures)
    
    # Create the backup
    create_backup "$project_root" "$temp_empty_structures"
    
    # Restore output if quiet mode
    if [ "$quiet_mode" = true ]; then
        exec 1>&3 2>&4 3>&- 4>&-
    fi
    
    # Display backup information
    display_backup_info
    
    # Verify backup if requested
    if [ "$verify_backup_flag" = true ]; then
        verify_backup
    fi
    
    # Open backup directory in file manager
    open_backup_directory
    
    echo ""
    echo "🎉 Backup process completed successfully!"
    echo ""
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
