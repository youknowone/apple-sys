# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

apple-sys is a Rust FFI bindings generator for Apple frameworks. It provides auto-generated bindgen bindings for 200+ Apple frameworks (CoreFoundation, UIKit, AVFoundation, AppKit, etc.) across macOS and iOS platforms.

The workspace contains two crates:
- **crates/apple-sys/** - The main `apple-sys` crate with pre-generated, feature-gated bindings
- **crates/apple-bindgen/** - The `apple-bindgen` CLI tool for generating framework bindings

## Build Commands

```bash
# Build with specific framework features
cargo build --manifest-path=crates/apple-sys/Cargo.toml --features CoreFoundation,IOKit

# Build for iOS
cargo build --target=aarch64-apple-ios --manifest-path=crates/apple-sys/Cargo.toml --features UIKit
```

## apple-bindgen CLI

```bash
# Generate bindings for a framework
cargo run --features=bin --manifest-path=crates/apple-bindgen/Cargo.toml -- generate CoreFoundation --sdk macosx

# Analyze framework dependencies
cargo run --features=bin --manifest-path=crates/apple-bindgen/Cargo.toml -- analyze-deps AppKit --sdk macosx
```

## Testing

```bash
# Run bindgen crate tests
cargo test --manifest-path=crates/apple-bindgen/Cargo.toml

# Run a single test
cargo test --manifest-path=crates/apple-bindgen/Cargo.toml generate_core_foundation

# Verify a framework builds (macOS)
cargo build --manifest-path=crates/apple-sys/Cargo.toml --features AGL

# Verify a framework builds (iOS)
cargo build --target=aarch64-apple-ios --manifest-path=crates/apple-sys/Cargo.toml --features ARKit
```

## Formatting

```bash
cargo fmt
```

## Architecture

### Generation Pipeline

1. User enables frameworks via Cargo features (feature names match framework names)
2. `crates/apple-sys/build.rs` reads enabled features and calls `apple_bindgen::Builder` for each
3. Builder loads configuration from `crates/apple-bindgen/Bindgen.toml` (opaque types, blocklists, replacements)
4. Generates Rust FFI bindings via bindgen from Apple SDK headers
5. Post-processes bindings with regex replacements to fix name collisions and Objective-C quirks
6. Writes `{framework}.rs` files to `OUT_DIR`
7. `crates/apple-sys/src/lib.rs` includes generated files conditionally per target OS and features

### Key Files

- `crates/apple-bindgen/Bindgen.toml` - Framework-specific bindgen rules (replacements for name collisions, opaque types, deps)
- `crates/apple-bindgen/src/deps/` - Dependency analysis module (type extraction, ownership, symbol isolation)
- `crates/apple-sys/build.rs` - Build script that orchestrates binding generation with ownership-based deduplication
- `crates/apple-sys/configure.py` - Python script to regenerate lib.rs, Cargo.toml, and platform includes from Xcode SDKs

### Bindgen.toml Configuration

The config file handles:
- **opaque_types**: Types to make opaque (workarounds for bindgen bugs)
- **replacements**: Regex-like fixes for Foundation/AppKit/UIKit name collisions (e.g., `timezone` → `timezone_`)
- **deps**: Framework dependencies (e.g., Quartz depends on PDFKit)
- **impl_debugs**: Types needing manual Debug impl

### Dependency Isolation

The build.rs pipeline automatically deduplicates symbols across frameworks using an ownership-based approach:
1. Scans SDK framework headers to build a global ownership map (symbol → owning framework)
2. Computes unique symbols per framework via topological ordering and dependency closure
3. Filters generated bindings to only include each framework's unique symbols
4. Earlier frameworks (in topological order) are imported via `use crate::FrameworkName::*`

## Contributing

This project manages bindgen rules, not generated code. Changes go in `crates/apple-bindgen/Bindgen.toml` for framework-specific fixes.
