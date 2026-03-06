//! Generate Rust bindings for Apple frameworks.
//!
//! This project only generates bindings as strings.
//! See [apple-sys](https://crates.io/crates/apple-sys) to import the result as a dependency.

mod builder;
mod config;
pub mod deps;
pub mod objc2;
mod sdk;
#[cfg(test)]
mod test;

pub use builder::Builder;
pub use config::{Config, ConfigMap, FileConfig};
pub use deps::{
    CacheKey, DependencyAnalyzer, DependencyGraphs, FrameworkSymbols, build_dependency_graph,
    build_dependency_graphs, c_integer_primitive, collect_all_deps, compute_reachable,
    compute_reachable_symbols, extract_symbols, extract_types_for_framework, extract_types_from_rs,
    filter_symbols, filter_to_reachable, get_filterable_dep_symbols, impl_block_deps, is_builtin,
    load_cached_framework, load_cached_symbols, load_deps, save_cached_symbols,
    scan_framework_headers, scan_objc_headers, scan_sub_frameworks, scan_system_types,
    topological_sort,
};
pub use sdk::{SdkPath, SdkPathError};
