#!/bin/bash
# Build script for decker v0.1.0
# Xbox Controller → Okular Control daemon
# Supports building for different CPU architectures

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
BUILD_DIR="${PROJECT_DIR}/target"
OUTPUT_DIR="${PROJECT_DIR}/builds"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${GREEN}[decker v0.1.0 Build System]${NC}"
    echo "$1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

show_help() {
    cat << EOF
Usage: ./build.sh [OPTIONS]

OPTIONS:
    znver3      Build optimized for Zen3/Zen4 (Ryzen 5000+, EPYC 7003+)
    znver2      Build optimized for Zen2 (Ryzen 3000+)
    avx2        Build optimized for AVX2 (Intel Skylake+, AMD Ryzen 1000+)
    v3          Build optimized for x86-64-v3 (Enhanced SIMD)
    generic     Build generic x86-64 (maximum compatibility)
    all         Build all variants
    clean       Clean build artifacts
    help        Show this help message

EXAMPLES:
    ./build.sh znver3           # Build for Zen3 (optimized)
    ./build.sh avx2             # Build for AVX2
    ./build.sh all              # Build all variants
    ./build.sh clean            # Clean up

ENVIRONMENT VARIABLES:
    RUSTFLAGS   Additional Rust compiler flags
    CARGO_BUILD_JOBS  Number of parallel build jobs

EOF
}

build_variant() {
    local variant=$1
    local target_cpu=$2
    local flags=$3
    
    print_info "Building for $variant..."
    print_info "Target CPU: $target_cpu"
    
    export RUSTFLAGS="-C target-cpu=$target_cpu -C opt-level=3 -C lto=thin -C codegen-units=1 $flags"
    
    cargo build --release 2>&1 | grep -E "(Compiling|Finished|error|warning:)" || true
    
    if [ -f "$BUILD_DIR/release/decker" ]; then
        mkdir -p "$OUTPUT_DIR"
        local output_file="$OUTPUT_DIR/decker-$variant"
        cp "$BUILD_DIR/release/decker" "$output_file"
        chmod +x "$output_file"
        
        # Get file size
        local size=$(du -h "$output_file" | cut -f1)
        print_info "✓ Built: $output_file ($size)"
        
        return 0
    else
        print_error "Build failed for $variant"
        return 1
    fi
}

main() {
    local target=${1:-help}
    
    cd "$PROJECT_DIR"
    
    # Verify Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Install from https://rustup.rs/"
        exit 1
    fi

    # Verify dependencies
    if ! command -v xdotool &> /dev/null; then
        print_error "xdotool not found. Install with: sudo apt install xdotool"
        exit 1
    fi
    
    case "$target" in
        znver3)
            build_variant "znver3" "znver3" ""
            ;;
        znver2)
            build_variant "znver2" "znver2" ""
            ;;
        avx2)
            build_variant "avx2" "x86-64-v2" ""
            ;;
        v3)
            build_variant "v3" "x86-64-v3" ""
            ;;
        generic)
            build_variant "generic" "x86-64" ""
            ;;
        all)
            print_header "Building all variants..."
            echo ""
            
            local failed=0
            
            build_variant "znver3" "znver3" "" || ((failed++))
            echo ""
            build_variant "znver2" "znver2" "" || ((failed++))
            echo ""
            build_variant "avx2" "x86-64-v2" "" || ((failed++))
            echo ""
            build_variant "v3" "x86-64-v3" "" || ((failed++))
            echo ""
            build_variant "generic" "x86-64" "" || ((failed++))
            
            echo ""
            if [ $failed -eq 0 ]; then
                print_header "All builds successful!"
                echo "Binaries available in: $OUTPUT_DIR"
                ls -lh "$OUTPUT_DIR/"
            else
                print_error "$failed build(s) failed"
                exit 1
            fi
            ;;
        clean)
            print_info "Cleaning build artifacts..."
            cargo clean
            rm -rf "$OUTPUT_DIR"
            print_info "✓ Cleaned"
            ;;
        help)
            show_help
            ;;
        *)
            print_error "Unknown target: $target"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"
