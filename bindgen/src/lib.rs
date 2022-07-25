//! Generate Rust bindings for Apple frameworks.
//!
//! This projects only generate bindings to string.
//! See [apple-sys](https://crates.io/crates/apple-sys) to import the result as a dependency.

mod builder;
mod config;
mod sdk;
#[cfg(test)]
mod test;

pub use builder::Builder;
pub use config::{Config, ConfigMap, FileConfig};
pub use sdk::{SdkPath, SdkPathError};
