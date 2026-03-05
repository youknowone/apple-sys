//! Framework dependency isolation module.
//!
//! Provides symbol filtering to avoid duplicate definitions when
//! frameworks depend on each other.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::Command;
use syn::{Item, Visibility};

/// Get the type prefixes for a given framework.
/// Only types with these prefixes are considered "owned" by the framework.
fn framework_prefixes(framework: &str) -> Option<&'static [&'static str]> {
    match framework {
        // Root framework - owns everything (system constants, etc.)
        "CoreFoundation" => None,
        "Foundation" => Some(&["NS", "__NS"]),
        // AppKit/CoreData use NS* but don't own them (Foundation does)
        "AppKit" => Some(&[]),
        "CoreData" => Some(&[]),
        "CoreGraphics" => Some(&["CG", "__CG"]),
        "CoreText" => Some(&["CT", "__CT"]),
        "QuartzCore" => Some(&["CA", "__CA"]),
        "CoreServices" => Some(&["LS", "UT", "MDItem", "FSEvent", "AE"]),
        "CoreImage" => Some(&["CI", "__CI"]),
        "CoreMedia" => Some(&["CM", "__CM"]),
        "CoreVideo" => Some(&["CV", "__CV"]),
        "CoreAudio" => Some(&["Audio", "kAudio"]),
        "AVFoundation" => Some(&["AV", "__AV"]),
        "Metal" => Some(&["MTL", "__MTL"]),
        "IOKit" => Some(&["IO", "io_", "kIO"]),
        "Security" => Some(&["Sec", "CSSM", "kSec"]),
        "SystemConfiguration" => Some(&["SC", "kSC"]),
        "ImageIO" => Some(&["CGImage", "kCGImage"]),
        "ColorSync" => Some(&["ColorSync"]),
        "Cocoa" => Some(&[]), // Umbrella framework, no unique types
        _ => None,
    }
}

/// Common bindgen symbols that should only be defined once
const BINDGEN_COMMON_SYMBOLS: &[&str] = &[
    "__BindgenBitfieldUnit",
    "__BindgenComplex",
    "__BindgenFloat16",
    "__IncompleteArrayField",
    "id",
];

/// Check if a symbol is a bindgen common symbol
fn is_bindgen_common_symbol(symbol: &str) -> bool {
    BINDGEN_COMMON_SYMBOLS.contains(&symbol)
}

/// Check if a symbol belongs to a framework (has framework-specific prefix)
fn is_framework_owned_symbol(symbol: &str, framework: &str) -> bool {
    // Bindgen common symbols should be defined in every framework
    // because generated code uses fully qualified paths
    if is_bindgen_common_symbol(symbol) {
        return false;
    }
    match framework_prefixes(framework) {
        Some(prefixes) if prefixes.is_empty() => false,
        Some(prefixes) => prefixes.iter().any(|p| symbol.starts_with(p)),
        None => true, // No prefix info = root framework, owns everything
    }
}

/// Filter dep_symbols to only include framework-owned symbols
pub fn get_filterable_dep_symbols(
    dep_symbols: &HashSet<String>,
    dep_framework: &str,
) -> HashSet<String> {
    dep_symbols
        .iter()
        .filter(|s| is_framework_owned_symbol(s, dep_framework))
        .cloned()
        .collect()
}

/// Get SDK version from xcrun
pub fn get_sdk_version() -> String {
    Command::new("xcrun")
        .args(["--show-sdk-version"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Cache key for symbol cache validation
#[derive(Debug, PartialEq)]
pub struct CacheKey {
    pub sdk_version: String,
    pub bindgen_version: String,
    pub apple_bindgen_version: String,
}

impl CacheKey {
    pub fn current() -> Self {
        Self {
            sdk_version: get_sdk_version(),
            bindgen_version: "0.72".to_string(), // bindgen major.minor
            apple_bindgen_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Get the cache subdirectory name for this key
    pub fn cache_subdir(&self) -> String {
        format!(
            "MacOSX{}-bindgen{}-apple_bindgen{}",
            self.sdk_version, self.bindgen_version, self.apple_bindgen_version
        )
    }
}

/// Load cached symbols for a framework
pub fn load_cached_symbols(
    cache_dir: &PathBuf,
    framework: &str,
    current_key: &CacheKey,
) -> Option<HashSet<String>> {
    load_cached_framework(cache_dir, framework, current_key).map(|(syms, _)| syms)
}

/// Load cached symbols and dependencies for a framework
pub fn load_cached_framework(
    cache_dir: &PathBuf,
    framework: &str,
    current_key: &CacheKey,
) -> Option<(HashSet<String>, Vec<String>)> {
    let versioned_dir = cache_dir.join(current_key.cache_subdir());
    let cache_file = versioned_dir.join(format!("{}.toml", framework));
    let content = std::fs::read_to_string(&cache_file).ok()?;

    let mut symbols = Vec::new();
    let mut dependencies = Vec::new();
    let mut section = "";

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("symbols") {
            section = "symbols";
        } else if line.starts_with("dependencies") {
            section = "dependencies";
        } else if line == "]" {
            section = "";
        } else if !section.is_empty() {
            let val = line.trim_matches(|c| c == '"' || c == ',' || c == ' ');
            if !val.is_empty() {
                match section {
                    "symbols" => symbols.push(val.to_string()),
                    "dependencies" => dependencies.push(val.to_string()),
                    _ => {}
                }
            }
        }
    }

    Some((symbols.into_iter().collect(), dependencies))
}

/// Save unique symbols to cache file (symbols not found in dependencies)
pub fn save_cached_symbols(
    cache_dir: &PathBuf,
    framework: &str,
    key: &CacheKey,
    unique_symbols: &HashSet<String>,
    dependencies: &[String],
) {
    let versioned_dir = cache_dir.join(key.cache_subdir());
    let _ = std::fs::create_dir_all(&versioned_dir);
    let cache_file = versioned_dir.join(format!("{}.toml", framework));

    let mut sorted_symbols: Vec<_> = unique_symbols.iter().collect();
    sorted_symbols.sort();

    let mut sorted_deps: Vec<_> = dependencies.to_vec();
    sorted_deps.sort();

    let mut content = String::new();
    content.push_str("dependencies = [\n");
    for dep in &sorted_deps {
        content.push_str(&format!("  \"{}\",\n", dep));
    }
    content.push_str("]\n\n");
    content.push_str("symbols = [\n");
    for sym in sorted_symbols {
        content.push_str(&format!("  \"{}\",\n", sym));
    }
    content.push_str("]\n");

    if let Err(e) = std::fs::write(&cache_file, content) {
        eprintln!(
            "Warning: Failed to write cache file {}: {}",
            cache_file.display(),
            e
        );
    }
}

/// Load framework dependencies from deps.toml
pub fn load_deps() -> HashMap<String, Vec<String>> {
    let deps_content = include_str!("../../deps.toml");
    let mut deps = HashMap::new();

    for line in deps_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((name, rest)) = line.split_once(" = ") {
            let deps_str = rest.trim_matches(|c| c == '[' || c == ']');
            let dep_list: Vec<String> = deps_str
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !dep_list.is_empty() {
                deps.insert(name.to_string(), dep_list);
            }
        }
    }
    deps
}

/// Collect all transitive dependencies for a framework
pub fn collect_all_deps(
    framework: &str,
    deps: &HashMap<String, Vec<String>>,
    result: &mut HashSet<String>,
) {
    if let Some(direct_deps) = deps.get(framework) {
        for dep in direct_deps {
            if result.insert(dep.clone()) {
                collect_all_deps(dep, deps, result);
            }
        }
    }
}

/// Topological sort of frameworks based on dependencies
pub fn topological_sort(frameworks: &[&str], deps: &HashMap<String, Vec<String>>) -> Vec<String> {
    let framework_set: HashSet<&str> = frameworks.iter().copied().collect();
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut temp_mark = HashSet::new();

    fn visit(
        node: &str,
        deps: &HashMap<String, Vec<String>>,
        framework_set: &HashSet<&str>,
        visited: &mut HashSet<String>,
        temp_mark: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) {
        if visited.contains(node) {
            return;
        }
        if temp_mark.contains(node) {
            return;
        }
        temp_mark.insert(node.to_string());

        if let Some(node_deps) = deps.get(node) {
            for dep in node_deps {
                if framework_set.contains(dep.as_str()) {
                    visit(dep, deps, framework_set, visited, temp_mark, result);
                }
            }
        }

        temp_mark.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());
    }

    for &framework in frameworks {
        visit(
            framework,
            deps,
            &framework_set,
            &mut visited,
            &mut temp_mark,
            &mut result,
        );
    }

    result
}

/// Extract public symbols from generated Rust code using syn
pub fn extract_symbols(code: &str) -> HashSet<String> {
    let mut symbols = HashSet::new();

    let file = match syn::parse_file(code) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Warning: Failed to parse generated code: {}", e);
            return symbols;
        }
    };

    for item in file.items {
        if let Some(name) = extract_item_name(&item) {
            // Skip bindgen anonymous types: each independent bindgen run assigns
            // sequential numbers, so _bindgen_ty_42 in framework A is a different
            // type than _bindgen_ty_42 in framework B.
            if !name.starts_with("_bindgen_ty_") {
                symbols.insert(name);
            }
        }
    }

    symbols
}

/// Extract the name of a public item
fn extract_item_name(item: &Item) -> Option<String> {
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

/// Get the type name from an impl block's self_ty
fn get_impl_type_name(impl_item: &syn::ItemImpl) -> Option<String> {
    match impl_item.self_ty.as_ref() {
        syn::Type::Path(tp) => tp.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    }
}

/// Get the trait name from an impl block if it's a trait impl
fn get_impl_trait_name(impl_item: &syn::ItemImpl) -> Option<String> {
    impl_item
        .trait_
        .as_ref()
        .and_then(|(_, path, _)| path.segments.last().map(|s| s.ident.to_string()))
}

/// Check if a use tree references any symbol in dep_symbols
fn use_references_dep_symbol(tree: &syn::UseTree, dep_symbols: &HashSet<String>) -> bool {
    match tree {
        syn::UseTree::Path(path) => use_references_dep_symbol(&path.tree, dep_symbols),
        syn::UseTree::Name(name) => dep_symbols.contains(&name.ident.to_string()),
        syn::UseTree::Rename(rename) => dep_symbols.contains(&rename.ident.to_string()),
        syn::UseTree::Glob(_) => false,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|t| use_references_dep_symbol(t, dep_symbols)),
    }
}

/// Check if a use tree references a symbol NOT in the reachable set.
/// For `use self::EnumName::*;`, checks if EnumName is reachable.
fn use_references_unreachable(tree: &syn::UseTree, reachable: &HashSet<String>) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            let segment = path.ident.to_string();
            // `use self::X::*` → check if X is reachable
            if segment == "self" {
                return use_references_unreachable(&path.tree, reachable);
            }
            // External paths (crate::, std::, etc.) are always kept
            if matches!(
                segment.as_str(),
                "crate" | "super" | "std" | "core" | "alloc" | "objc" | "objc2"
            ) {
                return false;
            }
            // The path segment itself is a symbol name — check if it's reachable
            if !reachable.contains(&segment) {
                return true;
            }
            use_references_unreachable(&path.tree, reachable)
        }
        syn::UseTree::Name(name) => {
            let n = name.ident.to_string();
            !reachable.contains(&n)
        }
        syn::UseTree::Rename(rename) => {
            let n = rename.ident.to_string();
            !reachable.contains(&n)
        }
        syn::UseTree::Glob(_) => false,
        syn::UseTree::Group(group) => {
            // If ALL items in the group are unreachable, remove the entire use
            group
                .items
                .iter()
                .all(|t| use_references_unreachable(t, reachable))
        }
    }
}

/// Extract source and alias from a `use self::Source as Alias;` tree.
fn extract_use_rename(tree: &syn::UseTree) -> Option<(String, String)> {
    match tree {
        syn::UseTree::Path(path) if path.ident == "self" => extract_use_rename(&path.tree),
        syn::UseTree::Rename(rename) => {
            Some((rename.ident.to_string(), rename.rename.to_string()))
        }
        _ => None,
    }
}

/// Filter generated bindgen code to only include symbols owned by this framework.
///
/// This function performs three passes:
///
/// ## Pass 1: Primary filtering (strict)
///
/// Iterates over all items in the parsed code and keeps only:
/// - **Named items** (struct, enum, trait, type, const, static, fn, union):
///   kept if the name is in `reachable` (the framework's unique symbol set).
/// - **`use` statements**: kept if all referenced names are in `reachable`.
///   Removes imports that reference types owned by other frameworks.
/// - **`extern "C"` blocks**: individual foreign items are kept/dropped by name.
/// - **Impl blocks**: kept if the implementing type is in `reachable` AND all
///   type references within the block resolve to `reachable ∪ available ∪ builtin`.
///   This prevents emitting impl blocks that reference types from frameworks
///   not in the dependency chain (e.g., ObjC category extensions on NSString
///   that reference AppKit types would be dropped when filtering Foundation).
///
/// ## Pass 2: Restore dropped trait definitions
///
/// The dependency closure in `compute_ownership` may remove trait definitions
/// from `reachable` if their method signatures reference types that aren't
/// resolved (e.g., `IEKCalendar` uses `CGColorRef` which may not be available
/// during closure computation). This cascades: if the trait is removed, all
/// `impl Trait for Struct` blocks also fail the deps check in Pass 1.
///
/// This pass scans the original source for impl blocks whose implementing
/// struct survived Pass 1 but whose trait did not. Those traits are "needed"
/// and their definitions are restored from the original source.
///
/// ## Pass 3: Restore dropped impl blocks
///
/// With the needed traits now in the output, re-scans the original source for
/// impl blocks where the implementing type is in the surviving set and the
/// trait is either surviving, needed, or available from a dependency framework.
/// These impl blocks are added back.
///
/// ## Arguments
///
/// - `code`: Raw generated Rust source code (from bindgen).
/// - `reachable`: Unique symbols owned by this framework.
/// - `dep_frameworks`: List of dependency framework names (for `use crate::X::*`).
/// - `available`: Combined symbol set of this framework + all dependencies.
///   Used to validate impl block type references.
/// Build a dedup key for an impl block that distinguishes different impl blocks
/// for the same type. Trait impls use the trait name; plain impls use the first
/// method/item name (e.g., `__BindgenBitfieldUnit` has two plain impls: one
/// with `new`, another with `get`/`set`).
fn impl_dedup_key(impl_item: &syn::ItemImpl) -> String {
    let type_name = get_impl_type_name(impl_item);
    let trait_name = get_impl_trait_name(impl_item);
    let discriminant = trait_name
        .as_deref()
        .map(|t| t.to_string())
        .unwrap_or_else(|| {
            impl_item
                .items
                .first()
                .and_then(|it| match it {
                    syn::ImplItem::Fn(f) => Some(f.sig.ident.to_string()),
                    syn::ImplItem::Const(c) => Some(c.ident.to_string()),
                    syn::ImplItem::Type(t) => Some(t.ident.to_string()),
                    _ => None,
                })
                .unwrap_or_default()
        });
    format!(
        "impl_{}_{}",
        type_name.as_deref().unwrap_or(""),
        discriminant
    )
}

pub fn filter_to_reachable(
    code: &str,
    reachable: &HashSet<String>,
    dep_frameworks: &[String],
    available: Option<&HashSet<String>>,
) -> String {
    let file = match syn::parse_file(code) {
        Ok(f) => f,
        Err(_) => return code.to_string(),
    };

    let mut filtered_items = Vec::new();
    let mut extern_blocks = Vec::new();
    let mut emitted_symbols: HashSet<String> = HashSet::new();

    for item in file.items {
        match &item {
            Item::ForeignMod(fm) => {
                let mut filtered_foreign_items = Vec::new();
                for foreign_item in &fm.items {
                    let name = match foreign_item {
                        syn::ForeignItem::Fn(f) => Some(f.sig.ident.to_string()),
                        syn::ForeignItem::Static(s) => Some(s.ident.to_string()),
                        syn::ForeignItem::Type(t) => Some(t.ident.to_string()),
                        _ => None,
                    };
                    if let Some(n) = name {
                        if reachable.contains(&n) && emitted_symbols.insert(n) {
                            filtered_foreign_items.push(foreign_item.clone());
                        }
                    } else {
                        filtered_foreign_items.push(foreign_item.clone());
                    }
                }
                if !filtered_foreign_items.is_empty() {
                    let mut new_fm = fm.clone();
                    new_fm.items = filtered_foreign_items;
                    extern_blocks.push(Item::ForeignMod(new_fm));
                }
            }
            Item::Impl(impl_item) => {
                let type_name = get_impl_type_name(impl_item);
                let type_reachable = type_name.as_ref().map_or(false, |n| reachable.contains(n));

                if type_reachable {
                    // If available set is provided, check that all type refs
                    // in this impl block resolve to available symbols.
                    let deps_satisfied = if let Some(avail) = available {
                        use super::depgraph::{impl_block_deps, is_builtin};
                        let deps = impl_block_deps(impl_item);
                        deps.iter().all(|dep| {
                            is_builtin(dep) || reachable.contains(dep) || avail.contains(dep)
                        })
                    } else {
                        true
                    };

                    if deps_satisfied {
                        if emitted_symbols.insert(impl_dedup_key(impl_item)) {
                            filtered_items.push(item);
                        }
                    }
                }
            }
            Item::Use(use_item) => {
                if !use_references_unreachable(&use_item.tree, reachable) {
                    filtered_items.push(item);
                } else if let Some(avail) = available {
                    // `pub use self::X as Y;` where Y is reachable but X is owned by
                    // another framework. Convert to `pub type Y = X;` so it compiles
                    // when X is imported via `use crate::OtherFramework::*;`.
                    if let Some((source, alias)) = extract_use_rename(&use_item.tree) {
                        if reachable.contains(&alias) && avail.contains(&source) {
                            let type_alias: Item = syn::parse_str(&format!(
                                "pub type {alias} = {source};"
                            ))
                            .expect("failed to parse type alias");
                            filtered_items.push(type_alias);
                        }
                    }
                }
            }
            _ => {
                if let Some(name) = extract_item_name(&item) {
                    if reachable.contains(&name) && emitted_symbols.insert(name) {
                        filtered_items.push(item);
                    }
                } else {
                    filtered_items.push(item);
                }
            }
        }
    }

    // Pass 2 & 3: Restore dropped traits and impl blocks.
    // When a trait is removed by dependency closure but its implementing
    // struct survives, both the trait and impl block should be restored.
    if let Ok(file2) = syn::parse_file(code) {
        let surviving_names: HashSet<String> = filtered_items
            .iter()
            .filter_map(|item| extract_item_name(item))
            .collect();

        let available_set: HashSet<&str> = available
            .iter()
            .flat_map(|a| a.iter().map(|s| s.as_str()))
            .collect();

        // Find candidate traits for restoration: traits referenced by impl blocks
        // whose implementing struct survived but the trait itself was not emitted.
        let mut candidate_traits: HashSet<String> = HashSet::new();
        for item in &file2.items {
            if let Item::Impl(impl_item) = item {
                let type_name = get_impl_type_name(impl_item);
                let trait_name = get_impl_trait_name(impl_item);
                if let (Some(tn), Some(trn)) = (&type_name, &trait_name) {
                    if surviving_names.contains(tn)
                        && !surviving_names.contains(trn)
                        && !available_set.contains(trn.as_str())
                    {
                        candidate_traits.insert(trn.clone());
                    }
                }
            }
        }

        // Validate candidates: only restore traits whose type dependencies
        // are all satisfied (in surviving_names or available). This prevents
        // restoring traits whose methods reference types from unavailable
        // frameworks (e.g., INSObject in the objc module references instancetype
        // from CoreFoundation).
        use super::depgraph::is_builtin;
        let mut needed_traits: HashSet<String> = HashSet::new();
        for item in &file2.items {
            if let Item::Trait(trait_item) = item {
                let name = trait_item.ident.to_string();
                if candidate_traits.contains(&name) {
                    let mut collector = super::depgraph::TypeRefCollector::new();
                    syn::visit::Visit::visit_item_trait(&mut collector, trait_item);
                    let deps_ok = collector.types.iter().all(|dep| {
                        dep == &name
                            || is_builtin(dep)
                            || surviving_names.contains(dep)
                            || available_set.contains(dep.as_str())
                    });
                    if deps_ok {
                        needed_traits.insert(name);
                    }
                }
            }
        }

        // Restore validated trait definitions
        for item in &file2.items {
            if let Some(name) = extract_item_name(item) {
                if needed_traits.contains(&name) && emitted_symbols.insert(name) {
                    filtered_items.push(item.clone());
                }
            }
        }

        // Restore impl blocks whose type survived and trait is now known
        for item in &file2.items {
            if let Item::Impl(impl_item) = item {
                let type_name = get_impl_type_name(impl_item);
                let trait_name = get_impl_trait_name(impl_item);

                let type_ok = type_name
                    .as_ref()
                    .map_or(false, |n| surviving_names.contains(n));
                let trait_ok = trait_name.as_ref().map_or(true, |n| {
                    surviving_names.contains(n)
                        || needed_traits.contains(n)
                        || available_set.contains(n.as_str())
                });

                if type_ok && trait_ok {
                    if emitted_symbols.insert(impl_dedup_key(impl_item)) {
                        filtered_items.push(item.clone());
                    }
                }
            }
        }
    }

    let mut output = String::new();

    for dep in dep_frameworks {
        output.push_str(&format!(
            "#[allow(unused_imports)]\nuse crate::{}::*;\n",
            dep
        ));
    }
    if !dep_frameworks.is_empty() {
        output.push('\n');
    }

    use quote::ToTokens;
    for item in filtered_items {
        output.push_str(&item.to_token_stream().to_string());
        output.push('\n');
    }
    for item in extern_blocks {
        output.push_str(&item.to_token_stream().to_string());
        output.push('\n');
    }

    output
}

/// Filter out symbols that exist in dependencies and add use statements
pub fn filter_symbols(
    code: &str,
    dep_symbols: &HashSet<String>,
    dep_frameworks: &[String],
) -> String {
    let file = match syn::parse_file(code) {
        Ok(f) => f,
        Err(_) => return code.to_string(),
    };

    let mut filtered_items = Vec::new();
    let mut extern_blocks = Vec::new();
    // Track already emitted symbols to avoid duplicates from bindgen
    let mut emitted_symbols: HashSet<String> = HashSet::new();

    for item in file.items {
        match &item {
            Item::ForeignMod(fm) => {
                let mut filtered_foreign_items = Vec::new();
                for foreign_item in &fm.items {
                    let name = match foreign_item {
                        syn::ForeignItem::Fn(f) => Some(f.sig.ident.to_string()),
                        syn::ForeignItem::Static(s) => Some(s.ident.to_string()),
                        syn::ForeignItem::Type(t) => Some(t.ident.to_string()),
                        _ => None,
                    };
                    if let Some(n) = name {
                        if !dep_symbols.contains(&n) && emitted_symbols.insert(n) {
                            filtered_foreign_items.push(foreign_item.clone());
                        }
                    } else {
                        filtered_foreign_items.push(foreign_item.clone());
                    }
                }
                if !filtered_foreign_items.is_empty() {
                    let mut new_fm = fm.clone();
                    new_fm.items = filtered_foreign_items;
                    extern_blocks.push(Item::ForeignMod(new_fm));
                }
            }
            Item::Impl(impl_item) => {
                let type_name = get_impl_type_name(impl_item);
                let trait_name = get_impl_trait_name(impl_item);

                let type_in_deps = type_name
                    .as_ref()
                    .map_or(false, |n| dep_symbols.contains(n));
                let trait_in_deps = trait_name
                    .as_ref()
                    .map_or(false, |n| dep_symbols.contains(n));

                // For impl blocks, check if both type and trait have been emitted
                let impl_key = format!(
                    "impl_{}_{}",
                    type_name.as_deref().unwrap_or(""),
                    trait_name.as_deref().unwrap_or("")
                );

                if !type_in_deps && !trait_in_deps && emitted_symbols.insert(impl_key) {
                    filtered_items.push(item);
                }
            }
            Item::Use(use_item) => {
                if !use_references_dep_symbol(&use_item.tree, dep_symbols) {
                    filtered_items.push(item);
                }
            }
            _ => {
                if let Some(name) = extract_item_name(&item) {
                    if !dep_symbols.contains(&name) && emitted_symbols.insert(name) {
                        filtered_items.push(item);
                    }
                } else {
                    filtered_items.push(item);
                }
            }
        }
    }

    let mut output = String::new();

    // Add private use statements for dependencies (not pub, to avoid diamond problem)
    for dep in dep_frameworks {
        output.push_str(&format!(
            "#[allow(unused_imports)]\nuse crate::{}::*;\n",
            dep
        ));
    }
    if !dep_frameworks.is_empty() {
        output.push('\n');
    }

    use quote::ToTokens;
    for item in filtered_items {
        output.push_str(&item.to_token_stream().to_string());
        output.push('\n');
    }
    for item in extern_blocks {
        output.push_str(&item.to_token_stream().to_string());
        output.push('\n');
    }

    output
}
