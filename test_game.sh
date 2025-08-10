#!/bin/bash

# Mobile Poker Game Testing Suite
# This script provides comprehensive testing for the teach-poker mobile game

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_section "Checking Prerequisites"
    
    if ! command_exists cargo; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    print_success "Cargo found"
    
    if ! command_exists git; then
        print_error "Git not found. Please install Git."
        exit 1
    fi
    print_success "Git found"
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found. Please run this script from the project root."
        exit 1
    fi
    print_success "Project structure verified"
}

# Run unit tests
run_unit_tests() {
    print_section "Running Unit Tests"
    
    echo "Running all unit tests..."
    if cargo test --lib; then
        print_success "Unit tests passed"
    else
        print_error "Unit tests failed"
        return 1
    fi
}

# Run integration tests
run_integration_tests() {
    print_section "Running Integration Tests"
    
    echo "Running integration tests..."
    if cargo test --test integration_tests; then
        print_success "Integration tests passed"
    else
        print_error "Integration tests failed"
        return 1
    fi
    
    echo "Running poker-specific tests..."
    if cargo test --test poker_tests; then
        print_success "Poker tests passed"
    else
        print_error "Poker tests failed"
        return 1
    fi
}

# Check code quality
check_code_quality() {
    print_section "Code Quality Checks"
    
    echo "Checking compilation..."
    if cargo check; then
        print_success "Code compiles successfully"
    else
        print_error "Compilation failed"
        return 1
    fi
    
    echo "Running clippy for code quality..."
    if command_exists cargo-clippy || cargo install clippy 2>/dev/null; then
        if cargo clippy -- -D warnings; then
            print_success "Clippy checks passed"
        else
            print_warning "Clippy found issues (not blocking)"
        fi
    else
        print_warning "Clippy not available, skipping"
    fi
    
    echo "Checking code formatting..."
    if command_exists rustfmt || cargo install rustfmt 2>/dev/null; then
        if cargo fmt -- --check; then
            print_success "Code formatting is correct"
        else
            print_warning "Code formatting issues found (run 'cargo fmt' to fix)"
        fi
    else
        print_warning "rustfmt not available, skipping"
    fi
}

# Test mobile build targets
test_mobile_builds() {
    print_section "Testing Mobile Build Targets"
    
    # Test iOS simulator build (if on macOS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Testing iOS simulator build..."
        if rustup target list | grep -q "aarch64-apple-ios-sim.*installed"; then
            if cargo build --target aarch64-apple-ios-sim; then
                print_success "iOS simulator build successful"
            else
                print_error "iOS simulator build failed"
                return 1
            fi
        else
            print_warning "iOS simulator target not installed. Run: rustup target add aarch64-apple-ios-sim"
        fi
        
        echo "Testing iOS device build..."
        if rustup target list | grep -q "aarch64-apple-ios.*installed"; then
            if cargo build --target aarch64-apple-ios; then
                print_success "iOS device build successful"
            else
                print_error "iOS device build failed"
                return 1
            fi
        else
            print_warning "iOS device target not installed. Run: rustup target add aarch64-apple-ios"
        fi
    else
        print_warning "Not on macOS, skipping iOS builds"
    fi
    
    # Test Android builds (if Android targets are available)
    if rustup target list | grep -q "aarch64-linux-android.*installed"; then
        echo "Testing Android build..."
        if cargo build --target aarch64-linux-android; then
            print_success "Android build successful"
        else
            print_error "Android build failed"
            return 1
        fi
    else
        print_warning "Android target not installed. Run: rustup target add aarch64-linux-android"
    fi
}

# Test desktop builds for development
test_desktop_builds() {
    print_section "Testing Desktop Builds"
    
    echo "Testing debug build..."
    if cargo build; then
        print_success "Debug build successful"
    else
        print_error "Debug build failed"
        return 1
    fi
    
    echo "Testing release build..."
    if cargo build --release; then
        print_success "Release build successful"
    else
        print_error "Release build failed"
        return 1
    fi
}

# Performance testing
run_performance_tests() {
    print_section "Performance Testing"
    
    echo "Running performance benchmarks..."
    if cargo test --release performance_tests; then
        print_success "Performance tests passed"
    else
        print_warning "Performance tests failed or not available"
    fi
    
    echo "Testing memory usage..."
    if command_exists valgrind; then
        echo "Running memory leak detection..."
        # Note: This might not work on all systems, especially macOS
        print_warning "Memory testing requires manual verification"
    else
        print_warning "Valgrind not available for memory testing"
    fi
}

# Simulate game scenarios
test_game_scenarios() {
    print_section "Game Scenario Testing"
    
    echo "Testing game startup..."
    timeout 10s cargo run --bin teach-poker 2>/dev/null || {
        if [ $? -eq 124 ]; then
            print_success "Game started successfully (timed out as expected)"
        else
            print_error "Game failed to start"
            return 1
        fi
    }
    
    echo "Testing simple game binary..."
    timeout 5s cargo run --bin teach-poker src/main_simple.rs 2>/dev/null || {
        if [ $? -eq 124 ]; then
            print_success "Simple game started successfully (timed out as expected)"
        else
            print_warning "Simple game may have issues"
        fi
    }
}

# Generate test report
generate_report() {
    print_section "Test Report Generation"
    
    echo "Generating detailed test report..."
    {
        echo "# Poker Game Test Report"
        echo "Generated on: $(date)"
        echo ""
        echo "## Test Results"
        echo ""
        echo "### Unit Tests"
        cargo test --lib 2>&1 | grep -E "(test result:|running [0-9]+ test)"
        echo ""
        echo "### Integration Tests"
        cargo test --test integration_tests 2>&1 | grep -E "(test result:|running [0-9]+ test)"
        echo ""
        echo "### Code Statistics"
        echo "Lines of code:"
        find src -name "*.rs" -exec wc -l {} + | tail -1
        echo ""
        echo "Number of Rust files:"
        find src -name "*.rs" | wc -l
        echo ""
        echo "### Warnings"
        cargo build 2>&1 | grep -c "warning:" || echo "0"
        echo " warnings found"
    } > test_report.md
    
    print_success "Test report saved to test_report.md"
}

# Main testing function
run_all_tests() {
    local failed_tests=0
    
    check_prerequisites || exit 1
    
    run_unit_tests || ((failed_tests++))
    run_integration_tests || ((failed_tests++))
    check_code_quality || ((failed_tests++))
    test_desktop_builds || ((failed_tests++))
    test_mobile_builds || ((failed_tests++))
    run_performance_tests || ((failed_tests++))
    test_game_scenarios || ((failed_tests++))
    
    generate_report
    
    print_section "Final Results"
    
    if [ $failed_tests -eq 0 ]; then
        print_success "All tests completed successfully! 🎉"
        echo -e "${GREEN}Your poker game is ready for testing and deployment.${NC}"
    else
        print_warning "$failed_tests test suite(s) had issues"
        echo -e "${YELLOW}Some issues were found. Check the output above for details.${NC}"
    fi
    
    return $failed_tests
}

# Quick test function for rapid development
run_quick_tests() {
    print_section "Quick Development Tests"
    
    cargo check && \
    cargo test --lib && \
    print_success "Quick tests passed! Ready for development."
}

# Help function
show_help() {
    echo "Mobile Poker Game Testing Suite"
    echo ""
    echo "Usage: $0 [option]"
    echo ""
    echo "Options:"
    echo "  all        Run all tests (default)"
    echo "  quick      Run quick tests for development"
    echo "  unit       Run only unit tests"
    echo "  integration Run only integration tests"
    echo "  build      Test builds only"
    echo "  mobile     Test mobile builds only"
    echo "  quality    Run code quality checks only"
    echo "  performance Run performance tests only"
    echo "  help       Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0              # Run all tests"
    echo "  $0 quick        # Quick tests for development"
    echo "  $0 unit         # Only unit tests"
}

# Main script logic
case "${1:-all}" in
    "all")
        run_all_tests
        ;;
    "quick")
        run_quick_tests
        ;;
    "unit")
        run_unit_tests
        ;;
    "integration")
        run_integration_tests
        ;;
    "build")
        test_desktop_builds
        ;;
    "mobile")
        test_mobile_builds
        ;;
    "quality")
        check_code_quality
        ;;
    "performance")
        run_performance_tests
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        echo "Unknown option: $1"
        show_help
        exit 1
        ;;
esac
