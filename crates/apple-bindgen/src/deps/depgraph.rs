//! Symbol dependency graph construction and reachability analysis.
//!
//! Parses generated Rust bindings with `syn` and builds a directed graph
//! of type references between symbols. Then computes the set of reachable
//! symbols from a given set of root (owned) symbols via BFS.

use std::collections::{HashMap, HashSet, VecDeque};
use syn::visit::Visit;
use syn::{Item, Visibility};

/// Built-in types and paths that should not be tracked as dependencies.
pub fn is_builtin(name: &str) -> bool {
    // C/BSD integer type aliases: u_int32_t, int64_t, uint16_t, __int32_t, __uint16_t, etc.
    if is_c_integer_alias(name) {
        return true;
    }

    matches!(
        name,
        // Rust primitives
        "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "f32"
            | "f64"
            | "str"
            | "String"
            | "char"
            // std/core types
            | "Option"
            | "Result"
            | "Vec"
            | "Box"
            | "Sized"
            | "Send"
            | "Sync"
            | "Copy"
            | "Clone"
            | "Debug"
            | "Display"
            | "Default"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "Drop"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Iterator"
            | "IntoIterator"
            | "Fn"
            | "FnMut"
            | "FnOnce"
            | "Deref"
            | "DerefMut"
            // C types from std::os::raw
            | "c_void"
            | "c_char"
            | "c_schar"
            | "c_uchar"
            | "c_short"
            | "c_ushort"
            | "c_int"
            | "c_uint"
            | "c_long"
            | "c_ulong"
            | "c_longlong"
            | "c_ulonglong"
            | "c_float"
            | "c_double"
            // Special
            | "Self"
            | "self"
            | "Target"
            | "Error"
            | "Output"
            | "Formatter"
            | "Arguments"
    )
}

/// C/BSD integer type aliases that are just primitive mappings.
/// Patterns: int32_t, uint16_t, u_int64_t, __int32_t, __uint16_t, etc.
fn is_c_integer_alias(name: &str) -> bool {
    c_integer_primitive(name).is_some()
}

/// Map a C integer type alias to its Rust primitive equivalent.
/// Returns None if not an integer alias.
pub fn c_integer_primitive(name: &str) -> Option<&'static str> {
    let s = name.strip_prefix("__").unwrap_or(name);
    let (prefix, rest) = if let Some(r) = s.strip_prefix("u_int") {
        ("u", r)
    } else if let Some(r) = s.strip_prefix("uint") {
        ("u", r)
    } else if let Some(r) = s.strip_prefix("int") {
        ("i", r)
    } else {
        return None;
    };
    match (prefix, rest) {
        ("u", "8_t") => Some("u8"),
        ("u", "16_t") => Some("u16"),
        ("u", "32_t") => Some("u32"),
        ("u", "64_t") => Some("u64"),
        ("i", "8_t") => Some("i8"),
        ("i", "16_t") => Some("i16"),
        ("i", "32_t") => Some("i32"),
        ("i", "64_t") => Some("i64"),
        _ => None,
    }
}

/// Check if a path starts with a well-known module prefix (std, core, objc, etc.)
fn is_external_path(path: &syn::Path) -> bool {
    if let Some(first) = path.segments.first() {
        let name = first.ident.to_string();
        matches!(
            name.as_str(),
            "std" | "core" | "alloc" | "objc" | "libc" | "crate" | "super" | "self"
        )
    } else {
        false
    }
}

/// Collects type references from a syn AST item.
pub(crate) struct TypeRefCollector {
    pub(crate) types: HashSet<String>,
}

impl TypeRefCollector {
    pub(crate) fn new() -> Self {
        Self {
            types: HashSet::new(),
        }
    }
}

impl<'ast> Visit<'ast> for TypeRefCollector {
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        // Skip paths starting with std::, core::, objc::, etc.
        if !is_external_path(&node.path) {
            if let Some(seg) = node.path.segments.last() {
                let name = seg.ident.to_string();
                if !is_builtin(&name) {
                    self.types.insert(name);
                }
            }
        }
        // Continue visiting generic arguments within the path
        syn::visit::visit_type_path(self, node);
    }
}

/// Extract the name of a public item (same logic as isolation.rs extract_item_name)
fn item_name(item: &Item) -> Option<String> {
    match item {
        Item::Struct(s) if matches!(s.vis, Visibility::Public(_)) => Some(s.ident.to_string()),
        Item::Enum(e) if matches!(e.vis, Visibility::Public(_)) => Some(e.ident.to_string()),
        Item::Type(t) if matches!(t.vis, Visibility::Public(_)) => Some(t.ident.to_string()),
        Item::Fn(f) if matches!(f.vis, Visibility::Public(_)) => Some(f.sig.ident.to_string()),
        Item::Const(c) if matches!(c.vis, Visibility::Public(_)) => Some(c.ident.to_string()),
        Item::Static(s) if matches!(s.vis, Visibility::Public(_)) => Some(s.ident.to_string()),
        Item::Trait(t) if matches!(t.vis, Visibility::Public(_)) => Some(t.ident.to_string()),
        Item::Union(u) if matches!(u.vis, Visibility::Public(_)) => Some(u.ident.to_string()),
        _ => None,
    }
}

/// Extract `pub use self::X as Y;` renames and register them in the dep graphs.
///
/// Handles patterns like:
/// - `pub use self::ppd_ui_e as ppd_ui_t;` → Y="ppd_ui_t" depends on X="ppd_ui_e"
/// - `pub use self::{A as B, C as D};` → group renames
fn extract_use_renames(
    tree: &syn::UseTree,
    def_graph: &mut HashMap<String, HashSet<String>>,
    all_graph: &mut HashMap<String, HashSet<String>>,
) {
    match tree {
        syn::UseTree::Path(path) if path.ident == "self" => {
            extract_use_renames(&path.tree, def_graph, all_graph);
        }
        syn::UseTree::Rename(rename) => {
            let source = rename.ident.to_string();
            let alias = rename.rename.to_string();
            let deps: HashSet<String> = [source].into_iter().collect();
            def_graph.insert(alias.clone(), deps.clone());
            all_graph.insert(alias, deps);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                extract_use_renames(item, def_graph, all_graph);
            }
        }
        _ => {}
    }
}

/// Split dependency graphs: definition-level deps vs all deps (including impl blocks).
///
/// `definition_deps` contains only dependencies from struct/type/fn/extern definitions.
/// `all_deps` additionally includes dependencies from impl blocks merged into the self type.
///
/// The split is needed because impl block deps (e.g., ObjC category extensions referencing
/// types from other frameworks) should not cause the base type to be removed during
/// dependency closure in the ownership phase.
pub struct DependencyGraphs {
    /// Dependencies from definitions only (struct, type, fn, extern).
    /// Used for dependency closure (removing symbols whose deps are unavailable).
    pub definition_deps: HashMap<String, HashSet<String>>,
    /// All dependencies including impl block references.
    /// Used for BFS reachability computation.
    pub all_deps: HashMap<String, HashSet<String>>,
}

/// Build split dependency graphs from generated Rust code.
///
/// Returns `DependencyGraphs` with both definition-only and full (incl. impl) dep maps.
pub fn build_dependency_graphs(code: &str) -> DependencyGraphs {
    let file = match syn::parse_file(code) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "Warning: Failed to parse generated code for dep graph: {}",
                e
            );
            return DependencyGraphs {
                definition_deps: HashMap::new(),
                all_deps: HashMap::new(),
            };
        }
    };

    let mut def_graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_graph: HashMap<String, HashSet<String>> = HashMap::new();

    for item in &file.items {
        match item {
            Item::ForeignMod(fm) => {
                for foreign_item in &fm.items {
                    let name = match foreign_item {
                        syn::ForeignItem::Fn(f) => Some(f.sig.ident.to_string()),
                        syn::ForeignItem::Static(s) => Some(s.ident.to_string()),
                        syn::ForeignItem::Type(t) => Some(t.ident.to_string()),
                        _ => None,
                    };
                    if let Some(name) = name {
                        let mut collector = TypeRefCollector::new();
                        collector.visit_foreign_item(foreign_item);
                        def_graph.insert(name.clone(), collector.types.clone());
                        all_graph.insert(name, collector.types);
                    }
                }
            }
            Item::Impl(impl_item) => {
                let type_name = match impl_item.self_ty.as_ref() {
                    syn::Type::Path(tp) => tp.path.segments.last().map(|s| s.ident.to_string()),
                    _ => None,
                };

                if let Some(type_name) = type_name {
                    let mut collector = TypeRefCollector::new();
                    collector.visit_item_impl(impl_item);

                    // Impl block deps go ONLY into all_deps, not definition_deps.
                    // This prevents ObjC category extensions from pulling in
                    // cross-framework types that would cause the base type to be
                    // removed during dependency closure.
                    def_graph.entry(type_name.clone()).or_default();
                    let all_entry = all_graph.entry(type_name).or_default();
                    all_entry.extend(collector.types);

                    if let Some((_, path, _)) = &impl_item.trait_ {
                        if let Some(seg) = path.segments.last() {
                            let trait_name = seg.ident.to_string();
                            if !is_builtin(&trait_name) {
                                all_entry.insert(trait_name);
                            }
                        }
                    }
                }
            }
            Item::Use(use_item) => {
                // Track `pub use self::X as Y;` re-exports.
                // Registers Y as a symbol with dependency on X, so that:
                // - BFS discovers X transitively through Y
                // - Dependency closure verifies X is available before keeping Y
                extract_use_renames(&use_item.tree, &mut def_graph, &mut all_graph);
            }
            _ => {
                if let Some(name) = item_name(item) {
                    let mut collector = TypeRefCollector::new();
                    collector.visit_item(item);

                    let mut refs = collector.types;
                    refs.remove(&name);
                    def_graph.insert(name.clone(), refs.clone());
                    all_graph.insert(name, refs);
                }
            }
        }
    }

    DependencyGraphs {
        definition_deps: def_graph,
        all_deps: all_graph,
    }
}

/// Build a symbol dependency graph from generated Rust code.
///
/// Returns a map from each symbol name to the set of symbol names it references.
/// This is the legacy API that merges impl block deps into the type entry.
pub fn build_dependency_graph(code: &str) -> HashMap<String, HashSet<String>> {
    build_dependency_graphs(code).all_deps
}

/// Extract type references from a single impl block.
///
/// Returns all non-builtin type names referenced within the impl block,
/// including the trait name (if it's a trait impl). The trait path goes
/// through `visit_path` rather than `visit_type_path`, so it must be
/// handled explicitly.
pub fn impl_block_deps(impl_item: &syn::ItemImpl) -> HashSet<String> {
    let mut collector = TypeRefCollector::new();
    collector.visit_item_impl(impl_item);

    // Trait name is visited via visit_path, not visit_type_path,
    // so TypeRefCollector doesn't capture it. Add it explicitly.
    if let Some((_, path, _)) = &impl_item.trait_ {
        if !is_external_path(path) {
            if let Some(seg) = path.segments.last() {
                let trait_name = seg.ident.to_string();
                if !is_builtin(&trait_name) {
                    collector.types.insert(trait_name);
                }
            }
        }
    }

    collector.types
}

/// Compute the set of reachable symbols from roots via BFS.
pub fn compute_reachable(
    graph: &HashMap<String, HashSet<String>>,
    roots: &HashSet<String>,
) -> HashSet<String> {
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();

    // Seed BFS with roots that exist in the graph
    for root in roots {
        if graph.contains_key(root) && reachable.insert(root.clone()) {
            queue.push_back(root.clone());
        }
    }

    while let Some(current) = queue.pop_front() {
        if let Some(deps) = graph.get(&current) {
            for dep in deps {
                if reachable.insert(dep.clone()) {
                    // Only continue BFS if this symbol has its own edges
                    if graph.contains_key(dep) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
    }

    reachable
}

/// High-level API: compute reachable symbols from generated code and owned symbol set.
///
/// 1. Builds the dependency graph from the generated Rust code
/// 2. Intersects owned_symbols with the graph (to find roots that actually exist)
/// 3. BFS from roots to find all transitively reachable symbols
pub fn compute_reachable_symbols(code: &str, owned_symbols: &HashSet<String>) -> HashSet<String> {
    let graph = build_dependency_graph(code);
    compute_reachable(&graph, owned_symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_reachability() {
        let mut graph = HashMap::new();
        graph.insert("A".into(), HashSet::from(["B".into(), "C".into()]));
        graph.insert("B".into(), HashSet::from(["D".into()]));
        graph.insert("C".into(), HashSet::new());
        graph.insert("D".into(), HashSet::new());
        graph.insert("E".into(), HashSet::new()); // unreachable

        let roots = HashSet::from(["A".into()]);
        let reachable = compute_reachable(&graph, &roots);

        assert!(reachable.contains("A"));
        assert!(reachable.contains("B"));
        assert!(reachable.contains("C"));
        assert!(reachable.contains("D"));
        assert!(!reachable.contains("E"));
    }

    #[test]
    fn test_cyclic_reachability() {
        let mut graph = HashMap::new();
        graph.insert("A".into(), HashSet::from(["B".into()]));
        graph.insert("B".into(), HashSet::from(["A".into()]));
        graph.insert("C".into(), HashSet::new());

        let roots = HashSet::from(["A".into()]);
        let reachable = compute_reachable(&graph, &roots);

        assert!(reachable.contains("A"));
        assert!(reachable.contains("B"));
        assert!(!reachable.contains("C"));
    }

    #[test]
    fn test_build_graph_from_code() {
        let code = r#"
pub type CFIndex = ::std::os::raw::c_long;
pub type CFStringRef = *const __CFString;
pub struct __CFString {
    _data: [u8; 0],
}
pub struct MyStruct {
    pub field: CFIndex,
    pub name: CFStringRef,
}
"#;
        let graph = build_dependency_graph(code);

        // CFStringRef references __CFString
        assert!(
            graph
                .get("CFStringRef")
                .map_or(false, |deps| deps.contains("__CFString"))
        );

        // MyStruct references CFIndex and CFStringRef
        let my_deps = graph.get("MyStruct").unwrap();
        assert!(my_deps.contains("CFIndex"));
        assert!(my_deps.contains("CFStringRef"));

        // CFIndex should not reference anything (c_long is builtin)
        assert!(graph.get("CFIndex").map_or(true, |deps| deps.is_empty()));
    }

    #[test]
    fn test_extern_functions() {
        let code = r#"
pub type CFAllocatorRef = *const CFAllocator;
pub struct CFAllocator { _data: [u8; 0] }
pub type CFStringRef = *const __CFString;
pub struct __CFString { _data: [u8; 0] }
unsafe extern "C" {
    pub fn CFStringCreateCopy(alloc: CFAllocatorRef, theString: CFStringRef) -> CFStringRef;
}
"#;
        let graph = build_dependency_graph(code);

        let func_deps = graph.get("CFStringCreateCopy").unwrap();
        assert!(func_deps.contains("CFAllocatorRef"));
        assert!(func_deps.contains("CFStringRef"));
    }

    #[test]
    fn test_reachable_from_code() {
        let code = r#"
pub type CFIndex = ::std::os::raw::c_long;
pub type CFStringRef = *const __CFString;
pub struct __CFString { _data: [u8; 0] }
pub struct Unrelated { pub x: CFIndex }
unsafe extern "C" {
    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
}
"#;
        let owned = HashSet::from(["CFStringGetLength".to_string()]);
        let reachable = compute_reachable_symbols(code, &owned);

        assert!(reachable.contains("CFStringGetLength"));
        assert!(reachable.contains("CFStringRef"));
        assert!(reachable.contains("CFIndex"));
        assert!(reachable.contains("__CFString"));
        assert!(!reachable.contains("Unrelated"));
    }
}
