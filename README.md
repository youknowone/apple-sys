# apple-sys

Apple platforms have a rather monotonous programming environment compared to other platforms. On several development machines, we will dependably obtain the same [bindgen](https://github.com/rust-lang/rust-bindgen) result. Then why not simply have bindgen configurations for the frameworks?

## Workspace Structure

The workspace contains four crates:

- **`apple-sys`** - The main crate with feature-gated bindings for 200+ Apple frameworks
- **`apple-bindgen`** - CLI tool and library for generating framework bindings
- **`apple-sys-prebuilt-macosx`** - Pre-generated macOS bindings for faster builds without local SDK. Do not manually use this crate. This is exposed over `prebuilt` feature of apple-sys.
- **`apple-sys-prebuilt-iphoneos`** - Pre-generated iOS bindings used by `prebuilt` builds targeting iOS.

## How to use?

To add CoreFoundation and IOKit, try to run:
```sh
$ cargo add apple-sys --features=CoreFoundation,IOKit
```

Or add to Cargo.toml:
```toml
apple-sys = { version = "0.3", features = ["CoreFoundation", "IOKit"] }
```

The names of the features and the frameworks are same.
To see the full list, check the project's features in the Cargo.toml file.

The feature names are the module names. Everything is single-depth. There are no nested modules.

### Symbol Ownership Policy

`apple-sys` does not target framework-level self-contained re-exports.
Symbols are exposed by their owning framework module, based on ownership analysis.
Cross-framework references are imported internally for compilation, but not re-exported from dependent frameworks.

For example, if an API in module `A` references a type owned by module `B`, use the type from `B` directly.

```rust
use apple_sys::{CoreFoundation, IOKit};

// CoreFoundation::<any name>
// IOKit::<any name>
```

### Prebuilt vs Bindgen

By default, `apple-sys` uses prebuilt bindings, so no local SDK is required. If you need to generate bindings from your local SDK instead, disable default features and enable `bindgen`:

```toml
apple-sys = { version = "0.3", default-features = false, features = ["bindgen", "CoreFoundation"] }
```

## apple-bindgen

The bindgen tool is installable and generates the same result as apple-sys crates.
To create a new `-sys` project, starting with `apple-bindgen` result will be a convenient way.

Install:
```
$ cargo install apple-bindgen
```

To generate CoreFoundation bindings,
```
$ apple-bindgen generate CoreFoundation --sdk macosx
```

To generate UIKit bindings for iOS,
```
$ apple-bindgen generate UIKit --sdk iphoneos
```

To analyze framework dependencies,
```
$ apple-bindgen analyze-deps AppKit --sdk macosx
```

## Examples

The `apple-sys` crate includes numerous examples demonstrating usage of various Apple frameworks. Each example targets a specific framework.

```sh
# Run an example (requires the corresponding framework features)
$ cargo run --manifest-path=crates/apple-sys/Cargo.toml \
    --example metal_devices --features Metal,prebuilt

$ cargo run --manifest-path=crates/apple-sys/Cargo.toml \
    --example foundation_sysinfo --features Foundation,prebuilt
```

## Why apple-sys?

`apple-sys` contains auto-generated bindgen modules for Apple platforms. As long as we use the same versions of SDKs and bindgen, the result will be reproducible.

## Why not apple-sys?

Continually using the same SDKs doesn't sound realistic. I agree. Don't trust apple-sys. Use the managed versions as best you can. For `CoreFoundation`, for instance, use [core-foundation-sys](https://github.com/servo/core-foundation-rs).

The [objc2](https://github.com/madsmtm/objc2) project provides human-curated bindings with safe Rust APIs for most Apple frameworks.

Then why do I use apple-sys? I created apple-sys for minor and unmanaged frameworks. apple-sys will be the last fallback.

## Contributing

This project manages bindgen rules, not generated code. Changes go in `crates/apple-bindgen/Bindgen.toml` for framework-specific fixes.
