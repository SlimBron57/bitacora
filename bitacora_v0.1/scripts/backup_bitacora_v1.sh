#!/bin/bash
# ============================================================================
# Script: backup_bitacora_v1.sh
# Funcionalidad: Create local backup of Bitacora V1.0 project
# Flujo: Generates ZIP backup excluding build artifacts and including all architecture docs
# Uso: ./backup_bitacora_v1.sh
# Fecha de creacion: 2025-08-27
# ============================================================================

set -e  # Exit on any error

# Configuration
PROJECT_NAME="bitacora_v1.0"
BACKUP_DIR="/home/edgi/Backups"
TIMESTAMP=$(date +"%Y%m%d-%H%M")
BACKUP_FILENAME="${TIMESTAMP}_${PROJECT_NAME}.zip"
BACKUP_PATH="${BACKUP_DIR}/${BACKUP_FILENAME}"
PROJECT_ROOT="/home/edgi/Documents/Development/own/bitacora/bitacora_v0.1"

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

# Function to validate project structure
validate_project_structure() {
    cd "$PROJECT_ROOT"
    
    local required_dirs=(
        "_map"
        "bitacora-rust"
        "docs"
    )
    
    local missing_dirs=false
    
    for dir in "${required_dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            log_error "Required directory '$dir' not found"
            missing_dirs=true
        fi
    done
    
    if [ "$missing_dirs" = true ]; then
        log_error "Project structure validation failed"
        exit 1
    fi
    
    log_success "Project structure validated"
}

# Function to create the backup
create_backup() {
    log_step "Creating backup: $BACKUP_FILENAME"
    
    # Create temporary directory for backup preparation
    local temp_backup_dir="/tmp/bitacora_backup_$$"
    mkdir -p "$temp_backup_dir"
    
    cd "$PROJECT_ROOT"
    
    # Copy project files excluding build artifacts
    log_info "Copying project files..."
    
    # Use rsync for efficient copying with exclusions
    rsync -av \
        --exclude='bitacora-rust/target/' \
        --exclude='bitacora-rust/test-api/target/' \
        --exclude='bitacora-rust/cache/' \
        --exclude='bitacora-rust/logs/' \
        --exclude='bitacora-rust/data/' \
        --exclude='bitacora-rust/tmp/' \
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
        --exclude='__pycache__/' \
        --exclude='*.pyc' \
        --exclude='.pytest_cache/' \
        ./ "$temp_backup_dir/"
    
    # Create the ZIP file
    log_info "Compressing backup..."
    cd "$temp_backup_dir"
    zip -r "$BACKUP_PATH" . > /dev/null 2>&1
    
    # Cleanup
    rm -rf "$temp_backup_dir"
    
    cd "$PROJECT_ROOT"
}

# Function to calculate and display backup info
display_backup_info() {
    if [ -f "$BACKUP_PATH" ]; then
        local file_size=$(du -h "$BACKUP_PATH" | cut -f1)
        local file_count=$(unzip -l "$BACKUP_PATH" | tail -1 | awk '{print $2}')
        
        log_success "Backup created successfully!"
        echo ""
        echo "🧭 Bitacora V1.0 Backup Details:"
        echo "   • Location: $BACKUP_PATH"
        echo "   • Size: $file_size"
        echo "   • Files: $file_count items"
        echo "   • Timestamp: $(date)"
        echo ""
        echo "🚀 Includes:"
        echo "   • Complete Hybrid Navigator Architecture (6 docs)"
        echo "   • All Rust crates and source code"
        echo "   • Documentation and progress tracking"
        echo "   • Configuration files"
        echo "   • Project maps and checklists"
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
        "bitacora-rust/Cargo.toml"
        "_map/CHECKLIST.md"
        "_map/NAVIGATION.md"
        "_map/PROGRESS_UPDATE_20250827.md"
        "docs/architecture/01_HYBRID_NAVIGATOR_SYSTEM.md"
        "docs/CLI_TESTING_PROPOSAL.md"
        "bitacora-rust/src/"
        "bitacora-rust/crates/"
    )
    
    local verification_passed=true
    
    echo ""
    echo "🔍 Verification Results:"
    for file in "${important_files[@]}"; do
        if unzip -l "$BACKUP_PATH" | grep -q "$file"; then
            echo "   ✅ $file included"
        else
            echo "   ⚠️  $file not found in backup"
            verification_passed=false
        fi
    done
    
    # Check that excluded files are not present
    local excluded_patterns=("target/" "cache/" "logs/" "data/" "tmp/" ".git/")
    
    echo ""
    echo "🚫 Exclusion Check:"
    for pattern in "${excluded_patterns[@]}"; do
        if unzip -l "$BACKUP_PATH" | grep -q "$pattern"; then
            echo "   ⚠️  Excluded pattern $pattern found in backup"
            verification_passed=false
        else
            echo "   ✅ $pattern properly excluded"
        fi
    done
    
    echo ""
    if [ "$verification_passed" = true ]; then
        log_success "Backup verification passed - All checks OK!"
    else
        log_warning "Backup verification completed with warnings"
    fi
}

# Function to show project status
show_project_status() {
    echo ""
    echo "📊 Project Status at Backup Time:"
    echo "   • Base System: 99% Complete"
    echo "   • Hybrid Navigator: Architecture 100% Documented"
    echo "   • Current Branch: cli-ready-production-20250827"
    echo "   • Total Documentation: $(find docs/ -name "*.md" | wc -l) markdown files"
    echo "   • Rust Crates: $(find bitacora-rust/crates -name "Cargo.toml" | wc -l) crates"
    echo "   • Architecture Docs: 6 comprehensive technical documents"
    echo ""
}

# Function to show usage
show_usage() {
    echo ""
    echo "🧭 Bitacora V1.0 - Project Backup Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --verify   Verify backup after creation"
    echo "  -s, --status   Show project status"
    echo "  -q, --quiet    Quiet mode (less verbose output)"
    echo ""
    echo "Examples:"
    echo "  $0              # Create backup with default settings"
    echo "  $0 --verify     # Create backup and verify contents"
    echo "  $0 --status     # Show project status before backup"
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
    else
        log_info "Manual path: $BACKUP_DIR"
    fi
}

# Main execution function
main() {
    local verify_backup_flag=false
    local show_status_flag=false
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
            -s|--status)
                show_status_flag=true
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
    echo "🧭 Starting Bitacora V1.0 Backup"
    echo "================================="
    echo ""
    
    # Show project status if requested
    if [ "$show_status_flag" = true ]; then
        show_project_status
    fi
    
    # Validate project structure
    validate_project_structure
    
    # Create backup directory
    create_backup_directory
    
    # Create the backup
    create_backup
    
    # Restore output if quiet mode
    if [ "$quiet_mode" = true ]; then
        exec 1>&3 2>&4 3>&- 4>&-
    fi
    
    # Display backup information
    display_backup_info
    
    # Show project status
    show_project_status
    
    # Verify backup if requested
    if [ "$verify_backup_flag" = true ]; then
        verify_backup
    fi
    
    # Open backup directory in file manager
    open_backup_directory
    
    echo ""
    echo "🎉 Bitacora V1.0 Backup completed successfully!"
    echo "   Ready for Hybrid Navigator implementation tomorrow! 🚀"
    echo ""
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
