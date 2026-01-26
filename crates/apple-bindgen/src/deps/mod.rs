//! Framework dependency analysis and type extraction module.
//!
//! This module provides tools to:
//! - Extract type names from generated `.rs` binding files
//! - Analyze framework dependencies from umbrella headers
//! - Isolate framework symbols to avoid duplicates

mod analyzer;
pub mod depgraph;
mod extractor;
pub mod isolation;
pub mod ownership;

pub use analyzer::DependencyAnalyzer;
pub use depgraph::{
    DependencyGraphs, build_dependency_graph, build_dependency_graphs, c_integer_primitive,
    compute_reachable, compute_reachable_symbols, impl_block_deps, is_builtin,
};
pub use extractor::{extract_types_for_framework, extract_types_from_rs};
pub use isolation::{
    CacheKey, collect_all_deps, extract_symbols, filter_symbols, filter_to_reachable,
    get_filterable_dep_symbols, load_cached_framework, load_cached_symbols, load_deps,
    save_cached_symbols, topological_sort,
};
pub use ownership::{
    FrameworkSymbols, scan_framework_headers, scan_objc_headers, scan_sub_frameworks,
    scan_system_types,
};
