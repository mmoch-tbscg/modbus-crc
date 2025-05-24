.PHONY: all build build-release build-macos build-windows clean run-gui run-console install-deps

# Default target
all: build-release

# Build debug version
build:
	cargo build

# Build optimized release version  
build-release:
	cargo build --release

# Build portable macOS version
build-macos: build-release
	chmod +x build.sh
	./build.sh

# Build Windows .exe version
build-windows:
	chmod +x build_windows.sh
	./build_windows.sh

# Run GUI application
run-gui:
	cargo run --bin gui --release

# Run console application  
run-console:
	cargo run --bin console --release

# Install dependencies for cross-compilation
install-deps:
	rustup target add x86_64-pc-windows-gnu
	brew install mingw-w64

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist/ dist_windows/
	rm -f *.tar.gz *.zip

# Create release archives
package-macos: build-macos
	tar -czf crc_calculator_macos.tar.gz dist/

package-windows: build-windows  
	zip -r crc_calculator_windows.zip dist_windows/

package-all: package-macos package-windows
	@echo "✅ All packages created:"
	@echo "  📁 crc_calculator_macos.tar.gz"
	@echo "  📁 crc_calculator_windows.zip"

# Show help
help:
	@echo "CRC Calculator Build System"
	@echo "=========================="
	@echo ""
	@echo "Targets:"
	@echo "  build           - Build debug version"
	@echo "  build-release   - Build optimized version"
	@echo "  build-macos     - Create portable macOS bundle"
	@echo "  build-windows   - Create Windows .exe files"
	@echo "  run-gui         - Run GUI application"
	@echo "  run-console     - Run console application"
	@echo "  install-deps    - Install cross-compilation dependencies"
	@echo "  package-macos   - Create macOS archive"
	@echo "  package-windows - Create Windows archive"
	@echo "  package-all     - Create all archives"
	@echo "  clean           - Clean build artifacts"
	@echo "  help            - Show this help" 