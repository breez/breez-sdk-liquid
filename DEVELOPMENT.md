# Development guide

## Repository structure

### Crates

The project is organized into several Rust crates located in the `lib` directory:

- **bindings**: Contains UniFFI bindings for multiple languages (Kotlin, Swift, etc.)
- **core**: Core functionality of the SDK
- **wasm**: WebAssembly bindings for JavaScript/TypeScript environments

### Packages

The packages directory contains platform-specific implementations:

- **flutter_breez_liquid**: Flutter package built using [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) and [cargokit](https://github.com/irondash/cargokit)
  - `cargokit`: Cargokit package used to build/verify binaries
  - `lib`: Generated dart bindings
  - `rust`: Rust crate used for generating `flutter_rust_bridge` bindings
- **react-native**: React Native package for mobile development
- **wasm**: JavaScript/TypeScript packages derived from the Rust WASM bindings
  - `examples`: Example usage of the WASM bindings

## Development workflow

### Setting up your development environment

1. Ensure you have Rust installed with `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install required tools for cross-platform development:
   ```bash
   # For Flutter development
   flutter pub get
   
   # For WASM development
   cargo install wasm-pack
   
   # For bindings generation
   cargo install uniffi-bindgen
   ```

3. Clone the repository and navigate to your target package:
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```

### Building the project

Please refer to the specific development guides for detailed build instructions:

- [SDK Bindings](lib/bindings/DEVELOPMENT.md) - For Rust bindings and cross-platform generation
- [SDK Core](lib/core/DEVELOPMENT.md) - For core Lightning Network functionality  
- [SDK Wasm](lib/wasm/DEVELOPMENT.md) - For WebAssembly builds and JavaScript bindings
- [Flutter Package](packages/flutter_breez_liquid/DEVELOPMENT.md) - For Flutter/Dart development
- [React Native Package](packages/react-native/DEVELOPMENT.md) - For React Native development
- [Wasm Package](packages/wasm/DEVELOPMENT.md) - For browser and Node.js JavaScript packages

### Code formatting and linting

The project enforces code style using rustfmt and clippy. To enable automatic formatting and syntax checking:

```bash
git config --local core.hooksPath .githooks/
```

This will configure Git hooks to automatically check code formatting and run linting before commits.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on the contribution workflow, pull request process, and code standards.