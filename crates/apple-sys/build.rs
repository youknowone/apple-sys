fn main() {
    println!("cargo:rerun-if-changed=macos.inc.rs");
    println!("cargo:rerun-if-changed=ios.inc.rs");

    // prebuilt takes priority over bindgen
    #[cfg(feature = "prebuilt")]
    prebuilt_impl::main();

    #[cfg(all(feature = "bindgen", not(feature = "prebuilt")))]
    bindgen_impl::main();

    #[cfg(not(any(feature = "prebuilt", feature = "bindgen")))]
    panic!("Either 'prebuilt' or 'bindgen' feature must be enabled for apple-sys");
}

/// Remove `unsafe fn` methods containing msg_send! calls that are
/// incompatible with objc2:
/// - More than 12 colon-separated arguments (MessageArguments tuple limit)
/// - Parameters of type `*mut Sel` (Sel doesn't implement RefEncode)
/// Fix msg_send! arguments that collide with type names.
///
/// When an ObjC selector part like `setPDFView:` generates `msg_send!(..., setPDFView : PDFView)`,
/// `PDFView` resolves to the type constructor instead of the parameter `PDFView_`.
/// This function detects such collisions and appends `_` to use the parameter name.
/// Collect all `pub struct` and `pub const` names from a source string.
fn collect_shadow_names(source: &str) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    for line in source.lines() {
        let trimmed = line.trim();
        let rest = trimmed
            .strip_prefix("pub struct ")
            .or_else(|| trimmed.strip_prefix("pub const "));
        if let Some(rest) = rest {
            if let Some(name) = rest
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
            {
                if !name.is_empty() {
                    names.insert(name.to_string());
                }
            }
        }
    }
    names
}

fn fix_msg_send_type_collisions(
    source: &str,
    extra_shadow_names: &std::collections::HashSet<String>,
) -> String {
    use std::collections::HashSet;
    let local_names = collect_shadow_names(source);
    let shadow_names: HashSet<&str> = local_names
        .iter()
        .chain(extra_shadow_names.iter())
        .map(|s| s.as_str())
        .collect();
    if shadow_names.is_empty() {
        return source.to_string();
    }

    let mut in_trait = false;
    let mut trait_brace_depth: i32 = 0;
    let mut in_msg_send = false;
    let mut msg_send_depth: i32 = 0;

    let mut result = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed = line.trim();

        // Track trait blocks to restrict parameter renames
        if !in_trait && trimmed.starts_with("pub trait ") {
            in_trait = true;
            trait_brace_depth = 0;
        }
        if in_trait {
            for c in line.chars() {
                match c {
                    '{' => trait_brace_depth += 1,
                    '}' => trait_brace_depth -= 1,
                    _ => {}
                }
            }
            if trait_brace_depth <= 0 {
                in_trait = false;
            }
        }

        // Track multi-line msg_send! blocks (save state before updating)
        let is_in_msg_send = in_msg_send;
        if !in_msg_send && trimmed.contains("msg_send") {
            in_msg_send = true;
            msg_send_depth = 0;
        }
        if in_msg_send {
            for c in line.chars() {
                match c {
                    '(' => msg_send_depth += 1,
                    ')' => msg_send_depth -= 1,
                    _ => {}
                }
            }
            if msg_send_depth <= 0 {
                in_msg_send = false;
            }
        }

        // Match msg_send! lines and continuation lines of multi-line msg_send! blocks
        if line.contains("msg_send") || is_in_msg_send {
            let mut fixed = line.to_string();
            for &name in &shadow_names {
                let pattern = format!(" : {name})");
                let replacement = format!(" : {name}_)");
                if fixed.contains(&pattern) {
                    fixed = fixed.replace(&pattern, &replacement);
                }
                let pattern2 = format!(" : {name} ,");
                let replacement2 = format!(" : {name}_ ,");
                if fixed.contains(&pattern2) {
                    fixed = fixed.replace(&pattern2, &replacement2);
                }
                let pattern3 = format!(" : {name},");
                let replacement3 = format!(" : {name}_,");
                if fixed.contains(&pattern3) {
                    fixed = fixed.replace(&pattern3, &replacement3);
                }
                // End of line in multi-line msg_send (e.g. `fraction : fraction\n`)
                if fixed.trim_end().ends_with(&format!(" : {name}")) {
                    let trimmed_len = fixed.trim_end().len();
                    let suffix = &fixed[trimmed_len..];
                    fixed = format!(
                        "{} : {name}_{suffix}",
                        &fixed[..trimmed_len - format!(" : {name}").len()]
                    );
                }
            }
            result.push_str(&fixed);
        } else if in_trait {
            // Rename parameter names that collide with top-level item names.
            // Only inside trait blocks where msg_send! could reference them.
            let mut fixed = line.to_string();
            let mut did_fix = false;
            for &name in &shadow_names {
                let indent_pattern = format!("        {name}: ");
                let indent_replacement = format!("        {name}_: ");
                if fixed.contains(&indent_pattern) {
                    fixed = fixed.replace(&indent_pattern, &indent_replacement);
                    did_fix = true;
                }
                let param_pattern = format!(", {name}: ");
                let param_replacement = format!(", {name}_: ");
                if fixed.contains(&param_pattern) {
                    fixed = fixed.replace(&param_pattern, &param_replacement);
                    did_fix = true;
                }
                let param_pattern2 = format!("({name}: ");
                let param_replacement2 = format!("({name}_: ");
                if fixed.contains(&param_pattern2) {
                    fixed = fixed.replace(&param_pattern2, &param_replacement2);
                    did_fix = true;
                }
            }
            if did_fix {
                result.push_str(&fixed);
            } else {
                result.push_str(line);
            }
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Fix duplicate parameter names in function signatures.
///
/// When bindgen generates ObjC methods like `dividerImageForLeftSegmentState:rightSegmentState:`,
/// both parameters get named `state`, causing E0415. This appends a numeric suffix to duplicates.
fn fix_duplicate_params(source: &str) -> String {
    use std::collections::HashMap;
    let lines: Vec<&str> = source.lines().collect();
    let mut result = String::with_capacity(source.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("unsafe fn ") || trimmed.starts_with("pub unsafe fn ") {
            // Collect the entire function signature (up to the opening brace)
            let sig_start = i;
            let mut sig_end = i;
            let mut found_brace = false;
            for j in i..lines.len() {
                if lines[j].contains('{') {
                    sig_end = j;
                    found_brace = true;
                    break;
                }
            }
            if !found_brace {
                result.push_str(lines[i]);
                result.push('\n');
                i += 1;
                continue;
            }

            // Extract parameter names from the signature
            let mut param_counts: HashMap<String, usize> = HashMap::new();
            let mut has_dups = false;
            for j in sig_start..=sig_end {
                let line = lines[j].trim();
                // Match parameter patterns like `name: Type` (indented, or after comma/paren)
                if let Some(colon_pos) = line.find(": ") {
                    let before = &line[..colon_pos];
                    let param_name = before
                        .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                        .unwrap_or("");
                    if !param_name.is_empty()
                        && param_name != "&self"
                        && param_name != "self"
                        && param_name != "&mut"
                    {
                        let count = param_counts.entry(param_name.to_string()).or_insert(0);
                        *count += 1;
                        if *count > 1 {
                            has_dups = true;
                        }
                    }
                }
            }

            if !has_dups {
                // No duplicates, output as-is
                for j in sig_start..=sig_end {
                    result.push_str(lines[j]);
                    result.push('\n');
                }
                i = sig_end + 1;
                continue;
            }

            // Find the method body end (matching braces)
            let mut brace_depth: i32 = 0;
            let mut method_end = sig_end;
            for j in sig_end..lines.len() {
                for ch in lines[j].chars() {
                    if ch == '{' {
                        brace_depth += 1;
                    } else if ch == '}' {
                        brace_depth -= 1;
                    }
                }
                if brace_depth == 0 {
                    method_end = j;
                    break;
                }
            }

            // Build rename map for duplicate parameters
            let mut seen: HashMap<String, usize> = HashMap::new();
            let mut renames: Vec<(String, String)> = Vec::new();
            for j in sig_start..=sig_end {
                let line = lines[j].trim();
                if let Some(colon_pos) = line.find(": ") {
                    let before = &line[..colon_pos];
                    let param_name = before
                        .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                        .unwrap_or("");
                    if !param_name.is_empty()
                        && param_name != "&self"
                        && param_name != "self"
                        && param_name != "&mut"
                    {
                        let count = seen.entry(param_name.to_string()).or_insert(0);
                        *count += 1;
                        if *count > 1 {
                            renames.push((
                                param_name.to_string(),
                                format!("{}_{}", param_name, count),
                            ));
                        }
                    }
                }
            }

            // Apply renames to the entire method (signature + body)
            // Process from sig_start to method_end
            let mut rename_idx = 0;
            let mut seen2: HashMap<String, usize> = HashMap::new();
            for j in sig_start..=method_end {
                let mut line_out = lines[j].to_string();
                // For parameter declaration lines, rename the Nth occurrence
                if j <= sig_end {
                    let trimmed_line = lines[j].trim();
                    if let Some(colon_pos) = trimmed_line.find(": ") {
                        let before = &trimmed_line[..colon_pos];
                        let param_name = before
                            .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
                            .next()
                            .unwrap_or("");
                        if !param_name.is_empty() {
                            let count = seen2.entry(param_name.to_string()).or_insert(0);
                            *count += 1;
                            if *count > 1 {
                                if rename_idx < renames.len() {
                                    let (ref old, ref new) = renames[rename_idx];
                                    // Rename in parameter declaration
                                    let pattern = format!("{old}: ");
                                    let replacement = format!("{new}: ");
                                    line_out = line_out.replacen(&pattern, &replacement, 1);
                                    rename_idx += 1;
                                }
                            }
                        }
                    }
                }
                // For body lines with msg_send, rename the matching selector arguments
                if j > sig_end && lines[j].contains("msg_send") {
                    for (old, new) in &renames {
                        let pattern = format!(" : {old}");
                        let replacement = format!(" : {new}");
                        line_out = line_out.replace(&pattern, &replacement);
                    }
                }
                result.push_str(&line_out);
                result.push('\n');
            }
            i = method_end + 1;
        } else {
            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
        }
    }
    result
}

fn strip_incompatible_msg_send(source: &str) -> String {
    const MAX_MSG_SEND_ARGS: usize = 12;
    // Types that lack Encode/RefEncode in objc2 and cause msg_send! compilation errors
    const NON_ENCODE_TYPES: &[&str] = &[
        "simd_float4x4",
        "simd_float3x3",
        "simd_quatf",
        "sockaddr",
        "Sel",
        // TODO: Remove once ownership dedup ensures UIViewController's Encode impl
        // is visible to all frameworks that reference it (e.g. VideoSubscriberAccount
        // uses UIViewController but only ExternalAccessory owns its Encode impl).
        "UIViewController",
    ];
    let lines: Vec<&str> = source.lines().collect();
    let mut result = String::with_capacity(source.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("unsafe fn ") {
            let mut brace_depth: i32 = 0;
            let mut found_open_brace = false;
            let mut method_end = i;
            let mut has_msg_send = false;
            let mut body_colon_count = 0;
            let mut has_sel_param = false;
            let mut has_non_encode_type = false;
            for j in i..lines.len() {
                let line = lines[j];
                for ch in line.chars() {
                    if ch == '{' {
                        brace_depth += 1;
                        found_open_brace = true;
                    } else if ch == '}' {
                        brace_depth -= 1;
                    }
                }
                if line.contains("msg_send") {
                    has_msg_send = true;
                }
                // Detect *mut Sel parameters (Sel lacks RefEncode in objc2)
                if !found_open_brace
                    && (line.contains("*mut Sel") || line.contains("*mut objc2::runtime::Sel"))
                {
                    has_sel_param = true;
                }
                // Detect types lacking Encode/RefEncode in method signature
                if !found_open_brace {
                    for ty in NON_ENCODE_TYPES {
                        // Use word-boundary matching to avoid false positives
                        // (e.g. "Sel" must not match "Self")
                        if let Some(pos) = line.find(ty) {
                            let before = if pos > 0 {
                                line.as_bytes()[pos - 1]
                            } else {
                                b' '
                            };
                            let after_pos = pos + ty.len();
                            let after = if after_pos < line.len() {
                                line.as_bytes()[after_pos]
                            } else {
                                b' '
                            };
                            let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
                            if !is_word(before) && !is_word(after) {
                                has_non_encode_type = true;
                                break;
                            }
                        }
                    }
                }
                // Count ` : ` patterns in the body (after opening brace)
                if found_open_brace {
                    body_colon_count += line.matches(" : ").count();
                }
                if found_open_brace && brace_depth == 0 {
                    method_end = j;
                    break;
                }
            }
            let should_strip = has_msg_send
                && (body_colon_count > MAX_MSG_SEND_ARGS || has_sel_param || has_non_encode_type);
            if should_strip {
                i = method_end + 1;
                continue;
            }
        }
        result.push_str(lines[i]);
        result.push('\n');
        i += 1;
    }
    result
}

#[cfg(feature = "prebuilt")]
mod prebuilt_impl {
    use std::collections::{HashMap, HashSet};
    use std::io::Write;
    use std::path::PathBuf;

    pub fn main() {
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

        let (root_var, frameworks) = match target_os.as_str() {
            "ios" => (
                "DEP_APPLE_SYS_PREBUILT_IPHONEOS_ROOT",
                include!("ios.inc.rs") as Vec<&str>,
            ),
            _ => (
                "DEP_APPLE_SYS_PREBUILT_MACOSX_ROOT",
                include!("macos.inc.rs") as Vec<&str>,
            ),
        };

        let root = std::env::var(root_var)
            .unwrap_or_else(|_| panic!("{root_var} not set — is the prebuilt crate a dependency?"));

        let src_dir = PathBuf::from(&root).join("generated");
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

        #[cfg(not(feature = "__allow_empty"))]
        if frameworks.is_empty() {
            panic!(
                "apple-sys accepts module names as features. Empty feature maybe a mistake. If this is intentional, add to features: '__allow_empty'"
            );
        }

        let mut enabled: HashSet<&str> = frameworks.iter().copied().collect();
        // "objc" virtual module is always available (objc.rs generated alongside frameworks)
        enabled.insert("objc");

        // Pre-load all prebuilt .rs files
        let mut all_contents: HashMap<String, String> = HashMap::new();
        if let Ok(entries) = std::fs::read_dir(&src_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(fw) = name.strip_suffix(".rs") {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        all_contents.insert(fw.to_string(), content);
                    }
                }
            }
        }

        // Split into enabled contents and disabled dep_contents
        let mut dep_contents: HashMap<String, String> = HashMap::new();
        for (fw, content) in &all_contents {
            if !enabled.contains(fw.as_str()) {
                dep_contents.insert(fw.clone(), content.clone());
            }
        }

        // Build direct import graph from prebuilt source files
        let mut direct_imports: HashMap<String, HashSet<String>> = HashMap::new();
        for (fw, content) in &all_contents {
            if !enabled.contains(fw.as_str()) {
                continue;
            }
            let mut imports = HashSet::new();
            for line in content.lines() {
                if let Some(dep) = extract_crate_import(line) {
                    if enabled.contains(dep) {
                        imports.insert(dep.to_string());
                    }
                }
            }
            direct_imports.insert(fw.clone(), imports);
        }

        // Compute transitive closure of imports for each enabled framework
        let transitive_imports = compute_transitive_imports(&direct_imports);

        for framework in &frameworks {
            let content = match all_contents.get(*framework) {
                Some(c) => c.clone(),
                None => {
                    // Framework enabled via feature deps but no prebuilt file
                    // (e.g. macOS-only framework activated on iOS via union deps)
                    let dst = out_dir.join(format!("{framework}.rs"));
                    std::fs::write(&dst, "// not available on this platform\n")
                        .expect("could not write stub");
                    continue;
                }
            };

            let trans_deps = transitive_imports
                .get(*framework)
                .cloned()
                .unwrap_or_default();

            // Patch cross-framework imports:
            // - Keep imports for enabled frameworks (as non-pub use)
            // - Add transitive deps as non-pub use
            // - For disabled frameworks, inline private compatibility types
            let patched = patch_imports(framework, &content, &enabled, &dep_contents, &trans_deps);

            // Replace system types that may appear in inlined definitions
            let patched = replace_prebuilt_system_types(&patched);

            // Strip methods with incompatible msg_send! calls (>12 args or *mut Sel params)
            let patched = super::strip_incompatible_msg_send(&patched);

            // Fix msg_send! arguments that collide with type names
            let empty = std::collections::HashSet::new();
            let patched = super::fix_msg_send_type_collisions(&patched, &empty);

            // Fix duplicate parameter names (e.g. UIKit's state/state)
            let patched = super::fix_duplicate_params(&patched);

            let dst = out_dir.join(format!("{framework}.rs"));
            let mut file = std::fs::File::create(&dst).expect("could not create output file");
            file.write_all(patched.as_bytes())
                .expect("could not write output file");
            println!("cargo:rustc-link-lib=framework={framework}");
        }

        // Generate objc.rs (ObjC runtime types virtual module)
        let objc_src = src_dir.join("objc.rs");
        let objc_dst = out_dir.join("objc.rs");
        if objc_src.exists() {
            std::fs::copy(&objc_src, &objc_dst).expect("could not copy objc.rs");
        } else {
            std::fs::write(&objc_dst, "").expect("could not write empty objc.rs");
        }

        eprintln!(
            "Prebuilt: processed {} frameworks from {}",
            frameworks.len(),
            src_dir.display()
        );
    }

    /// Compute transitive closure of imports for each enabled framework.
    fn compute_transitive_imports(
        direct_imports: &HashMap<String, HashSet<String>>,
    ) -> HashMap<String, HashSet<String>> {
        let mut result: HashMap<String, HashSet<String>> = HashMap::new();

        for fw in direct_imports.keys() {
            let mut visited = HashSet::new();
            let mut stack: Vec<String> = direct_imports
                .get(fw)
                .map(|deps| deps.iter().cloned().collect())
                .unwrap_or_default();

            while let Some(current) = stack.pop() {
                if !visited.insert(current.clone()) {
                    continue;
                }
                if let Some(deps) = direct_imports.get(&current) {
                    for dep in deps {
                        if !visited.contains(dep) {
                            stack.push(dep.clone());
                        }
                    }
                }
            }
            visited.remove(fw); // Don't include self
            result.insert(fw.clone(), visited);
        }

        result
    }

    /// Patch cross-framework imports in a prebuilt .rs file.
    /// - Imports referencing enabled frameworks are kept as non-pub use.
    /// - Transitive deps are added as non-pub use.
    /// - Imports referencing disabled frameworks are removed and their
    ///   private compatibility type/struct definitions are inlined.
    fn patch_imports(
        current_framework: &str,
        content: &str,
        enabled: &std::collections::HashSet<&str>,
        dep_contents: &std::collections::HashMap<String, String>,
        transitive_deps: &std::collections::HashSet<String>,
    ) -> String {
        let mut result = String::with_capacity(content.len() + 4096);
        let mut disabled_imports: Vec<&str> = Vec::new();
        let mut seen_imports: HashSet<String> = HashSet::new();

        for line in content.lines() {
            if let Some(fw) = extract_crate_import(line) {
                if enabled.contains(fw) {
                    result.push_str(line);
                    result.push('\n');
                    seen_imports.insert(fw.to_string());
                } else {
                    if result.ends_with("#[allow(unused_imports)]\n") {
                        let trim_pos = result.len() - "#[allow(unused_imports)]\n".len();
                        result.truncate(trim_pos);
                    }
                    disabled_imports.push(fw);
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        // Collect enabled frameworks already imported by this file
        let mut already_imported: std::collections::HashSet<String> = seen_imports.clone();
        let mut extra_imports = String::new();

        // Add transitive deps not already imported (always, even when no disabled imports)
        for dep in transitive_deps {
            if dep != current_framework
                && !already_imported.contains(dep)
                && enabled.contains(dep.as_str())
            {
                already_imported.insert(dep.clone());
                extra_imports
                    .push_str(&format!("#[allow(unused_imports)]\nuse crate::{dep}::*;\n"));
            }
        }

        if disabled_imports.is_empty() {
            // No disabled imports to inline, but still add transitive deps
            if !extra_imports.is_empty() {
                result.insert_str(0, &extra_imports);
            }
            return result;
        }

        // Inline types from the specific disabled frameworks that were imported.
        // Also follow their imports transitively for other disabled frameworks.
        // For enabled frameworks they import, add those imports to this file.
        let mut to_process: Vec<&str> = disabled_imports.clone();
        let mut processed: std::collections::HashSet<&str> = std::collections::HashSet::new();
        let mut all_types = String::from(
            "\n// Private compatibility types inlined from disabled dependency frameworks\n",
        );
        let mut libc_types: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

        // Track names already defined in the current framework to avoid duplicates
        let mut defined_names: std::collections::HashSet<String> = std::collections::HashSet::new();
        for line in content.lines() {
            let trimmed = line.trim();
            for prefix in &["pub type ", "pub struct ", "pub union ", "pub trait "] {
                if let Some(name) = extract_defined_type_name(trimmed, prefix) {
                    defined_names.insert(name);
                }
            }
        }

        // Collect existing libc imports from the target framework
        for line in content.lines() {
            if let Some(rest) = line.trim().strip_prefix("use libc::{") {
                if let Some(types_str) = rest.strip_suffix("};") {
                    for ty in types_str.split(',') {
                        let ty = ty.trim();
                        if !ty.is_empty() {
                            libc_types.insert(ty.to_string());
                        }
                    }
                }
            }
        }

        while let Some(fw) = to_process.pop() {
            if !processed.insert(fw) {
                continue;
            }
            if let Some(fw_content) = dep_contents.get(fw) {
                let types = extract_type_definitions(fw_content);
                if !types.is_empty() {
                    // Deduplicate: skip items already defined in the target or previously inlined
                    let deduped = dedup_inlined_types(&types, &mut defined_names);
                    if !deduped.is_empty() {
                        let compat = privatize_inlined_types(&deduped);
                        if !compat.is_empty() {
                            all_types.push_str(&compat);
                        }
                    }
                }
                // Collect libc imports from this disabled framework
                for line in fw_content.lines() {
                    let trimmed = line.trim();
                    if let Some(rest) = trimmed.strip_prefix("use libc::{") {
                        if let Some(types_str) = rest.strip_suffix("};") {
                            for ty in types_str.split(',') {
                                let ty = ty.trim();
                                if !ty.is_empty() {
                                    libc_types.insert(ty.to_string());
                                }
                            }
                        }
                    } else if let Some(rest) = trimmed.strip_prefix("use libc::") {
                        // Single import: use libc::pid_t;
                        if let Some(ty) = rest.strip_suffix(';') {
                            let ty = ty.trim();
                            if !ty.is_empty() && !ty.contains('{') {
                                libc_types.insert(ty.to_string());
                            }
                        }
                    }
                }
                // Follow imports from this disabled framework
                for line in fw_content.lines() {
                    if let Some(dep_fw) = extract_crate_import(line) {
                        if !enabled.contains(dep_fw) {
                            // Disabled → process transitively
                            if !processed.contains(dep_fw) {
                                to_process.push(dep_fw);
                            }
                        } else if dep_fw != current_framework && !already_imported.contains(dep_fw)
                        {
                            // Enabled but not yet imported (and not self) → add import
                            already_imported.insert(dep_fw.to_string());
                            extra_imports.push_str(&format!(
                                "#[allow(unused_imports)]\nuse crate::{dep_fw}::*;\n"
                            ));
                        }
                    }
                }
            }
        }

        // Insert extra imports at the beginning (before existing imports)
        if !extra_imports.is_empty() {
            result.insert_str(0, &extra_imports);
        }

        // Replace existing libc import with merged set, or add one
        if !libc_types.is_empty() {
            let libc_import = format!(
                "#[allow(unused_imports)]\nuse libc::{{{}}};\n",
                libc_types
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            // Try to replace existing use libc:: line
            let mut replaced = false;
            if let Some(start) = result.find("use libc::") {
                if let Some(end) = result[start..].find(";\n") {
                    let end_pos = start + end + 2;
                    // Also remove preceding #[allow(unused_imports)] if present
                    let actual_start = if start >= "#[allow(unused_imports)]\n".len()
                        && result[start - "#[allow(unused_imports)]\n".len()..start]
                            == *"#[allow(unused_imports)]\n"
                    {
                        start - "#[allow(unused_imports)]\n".len()
                    } else {
                        start
                    };
                    result.replace_range(actual_start..end_pos, &libc_import);
                    replaced = true;
                }
            }
            if !replaced {
                result.insert_str(0, &libc_import);
            }
        }

        if let Some(pos) = find_first_definition(&result) {
            result.insert_str(pos, &all_types);
        } else {
            result.push_str(&all_types);
        }

        result
    }

    /// Extract type/struct/union definitions and their essential impl blocks
    /// from a framework's .rs file for inlining.
    ///
    /// Inlines all `pub type` aliases (single-line and multi-line), all
    /// `pub struct`/`pub union` definitions with their attributes, and
    /// impl blocks for the extracted types (Encode, RefEncode, Debug, Deref,
    /// Message, and plain impl blocks).
    fn extract_type_definitions(content: &str) -> String {
        let mut result = String::new();
        let mut extracted_type_names: HashSet<String> = HashSet::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        // Accumulate attribute lines (#[repr(...)], #[derive(...)]) to prepend
        // to the next struct/union definition.
        let mut pending_attrs = String::new();

        // Pass 1: Extract type/struct/union definitions and collect type names
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // Accumulate attribute lines for upcoming struct/union
            if trimmed.starts_with("#[repr(") || trimmed.starts_with("#[derive(") {
                pending_attrs.push_str(line);
                pending_attrs.push('\n');
                i += 1;
                continue;
            }

            // pub type X = RHS; (single-line)
            if trimmed.starts_with("pub type ") && trimmed.ends_with(';') {
                pending_attrs.clear();
                if let Some(name) = extract_defined_type_name(trimmed, "pub type ") {
                    extracted_type_names.insert(name);
                }
                result.push_str(line);
                result.push('\n');
                i += 1;
                continue;
            }

            // pub use self::X as Y; (re-export alias)
            if trimmed.starts_with("pub use self::") && trimmed.ends_with(';') {
                pending_attrs.clear();
                result.push_str(line);
                result.push('\n');
                i += 1;
                continue;
            }

            // pub type X = ... (multi-line, e.g. Option<extern "C" fn(...)>)
            if trimmed.starts_with("pub type ") && trimmed.contains('=') && !trimmed.ends_with(';')
            {
                pending_attrs.clear();
                if let Some(name) = extract_defined_type_name(trimmed, "pub type ") {
                    extracted_type_names.insert(name);
                }
                let mut block = String::new();
                while i < lines.len() {
                    block.push_str(lines[i]);
                    block.push('\n');
                    let ends = lines[i].trim().ends_with(';');
                    i += 1;
                    if ends {
                        break;
                    }
                }
                result.push_str(&block);
                continue;
            }

            // Struct/union definitions
            if trimmed.starts_with("pub struct ") || trimmed.starts_with("pub union ") {
                let prefix = if trimmed.starts_with("pub struct ") {
                    "pub struct "
                } else {
                    "pub union "
                };
                if let Some(name) = extract_defined_type_name(trimmed, prefix) {
                    extracted_type_names.insert(name);
                }

                let attrs = std::mem::take(&mut pending_attrs);

                // Single-line struct/union (e.g. `pub struct Foo(pub id);`)
                if trimmed.ends_with(';') {
                    result.push_str(&attrs);
                    result.push_str(line);
                    result.push('\n');
                    i += 1;
                    continue;
                }

                // Multi-line struct/union with braces
                let mut block = String::new();
                let mut depth = 0i32;
                let mut found_open_brace = false;

                while i < lines.len() {
                    let l = lines[i];
                    block.push_str(l);
                    block.push('\n');

                    depth += l.chars().filter(|&c| c == '{').count() as i32;
                    if depth > 0 {
                        found_open_brace = true;
                    }
                    depth -= l.chars().filter(|&c| c == '}').count() as i32;
                    i += 1;

                    if found_open_brace && depth <= 0 {
                        break;
                    }
                }

                result.push_str(&attrs);
                result.push_str(&block);
                continue;
            }

            // Trait definitions: pub trait IFoo: ... { ... }
            if trimmed.starts_with("pub trait ") {
                pending_attrs.clear();
                if let Some(name) = extract_defined_type_name(trimmed, "pub trait ") {
                    extracted_type_names.insert(name);
                }
                let mut block = String::new();
                let mut depth = 0i32;
                let mut found_open_brace = false;
                while i < lines.len() {
                    block.push_str(lines[i]);
                    block.push('\n');
                    depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                    if depth > 0 {
                        found_open_brace = true;
                    }
                    depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                    i += 1;
                    if found_open_brace && depth <= 0 {
                        break;
                    }
                }
                result.push_str(&block);
                continue;
            }

            // Trait impl: impl IFoo for Bar {}, impl<T> Debug for Foo<T> {}
            // Also captures: unsafe impl objc2::Message for Bar {}
            if (trimmed.starts_with("impl ")
                || trimmed.starts_with("impl<")
                || trimmed.starts_with("unsafe impl ")
                || trimmed.starts_with("unsafe impl<"))
                && trimmed.contains(" for ")
            {
                pending_attrs.clear();
                let mut block = String::new();
                let mut depth = 0i32;
                let mut found_open_brace = false;
                while i < lines.len() {
                    block.push_str(lines[i]);
                    block.push('\n');
                    depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                    if depth > 0 {
                        found_open_brace = true;
                    }
                    depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                    i += 1;
                    if found_open_brace && depth <= 0 {
                        break;
                    }
                }
                result.push_str(&block);
                continue;
            }

            // Not a type definition — discard pending attributes
            pending_attrs.clear();
            i += 1;
        }

        // Pass 2: Extract remaining plain impl blocks for the extracted types
        // (e.g. `impl NSString { pub fn alloc() ... }`)
        // Trait impls (with `for`) are already captured in Pass 1.
        if !extracted_type_names.is_empty() {
            i = 0;
            while i < lines.len() {
                let trimmed = lines[i].trim();

                if let Some(type_name) = extract_impl_target_type(trimmed) {
                    if extracted_type_names.contains(&type_name) && is_plain_impl(trimmed) {
                        let mut block = String::new();
                        let mut depth = 0i32;
                        let mut found_open_brace = false;
                        while i < lines.len() {
                            block.push_str(lines[i]);
                            block.push('\n');
                            depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                            if depth > 0 {
                                found_open_brace = true;
                            }
                            depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                            i += 1;
                            if found_open_brace && depth <= 0 {
                                break;
                            }
                        }
                        result.push_str(&block);
                        continue;
                    }
                }
                i += 1;
            }
        }

        result
    }

    /// Filter inlined type definitions to remove items already present in defined_names.
    /// Adds newly seen names to defined_names for subsequent calls.
    fn dedup_inlined_types(
        types: &str,
        defined_names: &mut std::collections::HashSet<String>,
    ) -> String {
        let mut result = String::new();
        let lines: Vec<&str> = types.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let trimmed = lines[i].trim();

            // Check for definitions: pub type, pub struct, pub union, pub trait
            let mut def_name = None;
            for prefix in &["pub type ", "pub struct ", "pub union ", "pub trait "] {
                if let Some(name) = extract_defined_type_name(trimmed, prefix) {
                    def_name = Some(name);
                    break;
                }
            }

            // Check for impl blocks
            if def_name.is_none()
                && (trimmed.starts_with("impl ") || trimmed.starts_with("unsafe impl "))
            {
                // impl blocks: always include (they reference types, not define new names)
                // Just copy the block as-is
                let mut block = String::new();
                let mut depth = 0i32;
                let mut found_open_brace = false;
                while i < lines.len() {
                    block.push_str(lines[i]);
                    block.push('\n');
                    depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                    if depth > 0 {
                        found_open_brace = true;
                    }
                    depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                    i += 1;
                    if found_open_brace && depth <= 0 {
                        break;
                    }
                }
                result.push_str(&block);
                continue;
            }

            if let Some(name) = def_name {
                if defined_names.contains(&name) {
                    // Skip this definition (it's already defined)
                    // Skip the full block if it's multi-line
                    if trimmed.ends_with(';')
                        || (trimmed.starts_with("pub type ") && trimmed.ends_with(';'))
                    {
                        i += 1;
                        continue;
                    }
                    // Multi-line block: skip until braces balance
                    let mut depth = 0i32;
                    let mut found_open_brace = false;
                    while i < lines.len() {
                        depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                        if depth > 0 {
                            found_open_brace = true;
                        }
                        depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                        i += 1;
                        if found_open_brace && depth <= 0 {
                            break;
                        }
                    }
                    continue;
                }
                defined_names.insert(name);
            }

            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
        }

        result
    }

    /// Inlined compatibility definitions must stay internal to avoid exposing
    /// symbols from non-owning frameworks in the public API surface.
    fn privatize_inlined_types(types: &str) -> String {
        let mut result = String::with_capacity(types.len());
        for line in types.lines() {
            let trimmed = line.trim_start();
            let indent_len = line.len() - trimmed.len();
            let indent = &line[..indent_len];

            let rewritten = if let Some(rest) = trimmed.strip_prefix("pub type ") {
                format!("{indent}type {rest}")
            } else if let Some(rest) = trimmed.strip_prefix("pub struct ") {
                format!("{indent}struct {rest}")
            } else if let Some(rest) = trimmed.strip_prefix("pub union ") {
                format!("{indent}union {rest}")
            } else if let Some(rest) = trimmed.strip_prefix("pub trait ") {
                format!("{indent}trait {rest}")
            } else if let Some(rest) = trimmed.strip_prefix("pub use self::") {
                format!("{indent}use self::{rest}")
            } else {
                line.to_string()
            };

            result.push_str(&rewritten);
            result.push('\n');
        }
        result
    }

    /// Extract the type name from a definition line like "pub struct Foo(" or "pub type Bar = ...".
    fn extract_defined_type_name(line: &str, prefix: &str) -> Option<String> {
        let after = line.strip_prefix(prefix)?;
        let name: String = after
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() { None } else { Some(name) }
    }

    /// Extract the target type name from an impl block header line.
    /// Handles: `impl TypeName {`, `impl Trait for TypeName {`,
    ///          `unsafe impl Trait for TypeName {`, `impl<T> Trait for TypeName<T> {`
    fn extract_impl_target_type(line: &str) -> Option<String> {
        let rest = line.strip_prefix("unsafe ").unwrap_or(line);
        let rest = rest.strip_prefix("impl")?;
        // Must be followed by whitespace or '<' (not "ement" in "implement")
        if !rest.starts_with(' ') && !rest.starts_with('<') {
            return None;
        }
        let rest = rest.trim_start();

        // Skip generic parameters: impl<T: Bound> ...
        let rest = if rest.starts_with('<') {
            let mut depth = 0;
            let mut end = 0;
            for (i, c) in rest.char_indices() {
                match c {
                    '<' => depth += 1,
                    '>' => {
                        depth -= 1;
                        if depth == 0 {
                            end = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if end == 0 {
                return None;
            }
            rest[end..].trim_start()
        } else {
            rest
        };

        // If there's " for ", type is after it
        let type_part = if let Some(for_pos) = rest.rfind(" for ") {
            &rest[for_pos + 5..]
        } else {
            rest
        };

        // Extract just the identifier (before <, {, space, etc.)
        let name: String = type_part
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() { None } else { Some(name) }
    }

    /// Check if a line starts a plain impl block (no trait, no `for` keyword).
    /// e.g. `impl NSString {` or `impl<T> __IncompleteArrayField<T> {`
    fn is_plain_impl(line: &str) -> bool {
        let rest = line.strip_prefix("unsafe ").unwrap_or(line);
        let rest = match rest.strip_prefix("impl") {
            Some(r) => r,
            None => return false,
        };
        if !rest.starts_with(' ') && !rest.starts_with('<') {
            return false;
        }
        let rest = rest.trim_start();
        // Skip generics
        let rest = if rest.starts_with('<') {
            let mut depth = 0;
            let mut end = 0;
            for (i, c) in rest.char_indices() {
                match c {
                    '<' => depth += 1,
                    '>' => {
                        depth -= 1;
                        if depth == 0 {
                            end = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            rest[end..].trim_start()
        } else {
            rest
        };
        !rest.contains(" for ")
    }

    fn replace_prebuilt_system_types(content: &str) -> String {
        const REPLACEMENTS: &[(&str, &str)] = &[
            ("__darwin_off_t", "i64"),
            ("__darwin_time_t", "i64"),
            ("__darwin_ct_rune_t", "i32"),
        ];
        let mut result = content.to_string();
        for &(from, to) in REPLACEMENTS {
            if result.contains(from) {
                // Word-boundary-aware replacement
                let mut out = String::with_capacity(result.len());
                let mut remaining = result.as_str();
                while let Some(pos) = remaining.find(from) {
                    let before = if pos > 0 {
                        remaining.as_bytes()[pos - 1]
                    } else {
                        b' '
                    };
                    let after_pos = pos + from.len();
                    let after = if after_pos < remaining.len() {
                        remaining.as_bytes()[after_pos]
                    } else {
                        b' '
                    };
                    let is_word_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
                    if !is_word_char(before) && !is_word_char(after) {
                        out.push_str(&remaining[..pos]);
                        out.push_str(to);
                    } else {
                        out.push_str(&remaining[..after_pos]);
                    }
                    remaining = &remaining[after_pos..];
                }
                out.push_str(remaining);
                result = out;
            }
        }
        result
    }

    /// Find the byte position of the first real definition (after imports).
    fn find_first_definition(content: &str) -> Option<usize> {
        let mut pos = 0;
        for line in content.lines() {
            let trimmed = line.trim();
            let next_pos = pos + line.len() + 1; // +1 for newline
            if trimmed.starts_with("pub type ")
                || trimmed.starts_with("pub struct ")
                || trimmed.starts_with("#[repr(")
                || trimmed.starts_with("pub trait ")
                || trimmed.starts_with("pub enum ")
                || (trimmed.starts_with("use objc") && !trimmed.starts_with("use crate::"))
            {
                return Some(pos);
            }
            pos = next_pos;
        }
        None
    }

    /// Extract framework name from `use crate::FrameworkName::*;`
    fn extract_crate_import(line: &str) -> Option<&str> {
        let trimmed = line.trim();
        let rest = trimmed.strip_prefix("use crate::")?;
        let end = rest.find("::")?;
        let name = &rest[..end];
        if rest[end..].starts_with("::*;") {
            Some(name)
        } else {
            None
        }
    }
}

#[cfg(all(feature = "bindgen", not(feature = "prebuilt")))]
mod bindgen_impl {
    use apple_bindgen::{
        Builder, CacheKey, DependencyGraphs, FrameworkSymbols, SdkPath, build_dependency_graphs,
        c_integer_primitive, compute_reachable, filter_to_reachable, is_builtin,
        load_cached_framework, load_deps, save_cached_symbols, scan_framework_headers,
        scan_objc_headers, scan_sub_frameworks, scan_system_types, topological_sort,
    };
    use rayon::prelude::*;
    use std::collections::{HashMap, HashSet};
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::Command;
    use std::time::Instant;

    /// System types that exist in SDK headers but are not exported by the libc crate.
    /// Each entry maps a type name to its Rust definition.
    const LIBC_MISSING_TYPES: &[(&str, &str)] = &[
        ("mount_t", "*mut ::std::os::raw::c_void"), // kernel-only opaque pointer
    ];

    fn is_libc_missing(ty: &str) -> bool {
        LIBC_MISSING_TYPES.iter().any(|(name, _)| *name == ty)
    }

    /// Map __darwin_* intermediate typedefs to Rust primitives.
    fn darwin_type_primitive(name: &str) -> Option<&'static str> {
        match name {
            "__darwin_off_t" => Some("i64"),
            "__darwin_time_t" => Some("i64"), // long on Apple LP64
            "__darwin_ct_rune_t" => Some("i32"),
            _ => None,
        }
    }

    /// Replace whole-word occurrences only (identifier boundary aware).
    fn replace_whole_word(text: &str, word: &str, replacement: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut remaining = text;
        while let Some(pos) = remaining.find(word) {
            let before = if pos > 0 {
                remaining.as_bytes()[pos - 1]
            } else {
                b' '
            };
            let after_pos = pos + word.len();
            let after = if after_pos < remaining.len() {
                remaining.as_bytes()[after_pos]
            } else {
                b' '
            };
            let is_word_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
            if !is_word_char(before) && !is_word_char(after) {
                result.push_str(&remaining[..pos]);
                result.push_str(replacement);
            } else {
                result.push_str(&remaining[..after_pos]);
            }
            remaining = &remaining[after_pos..];
        }
        result.push_str(remaining);
        result
    }

    pub fn main() {
        // Limit rayon threads to half the cores (min 2, max 4)
        // to avoid overwhelming the system during generation
        let num_threads = std::thread::available_parallelism()
            .map(|n| (n.get() / 2).clamp(2, 4))
            .unwrap_or(2);
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .ok();

        let target = std::env::var("TARGET").expect("env TARGET must be set");
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

        let frameworks: Vec<&str> = match target_os.as_str() {
            "macos" => include!("macos.inc.rs"),
            "ios" => include!("ios.inc.rs"),
            unknown => panic!("unexpected target_os: {}", unknown),
        };

        let platform = apple_sdk::Platform::from_target_triple(&target)
            .expect("Unknown apple platform. please report it.");

        #[cfg(not(feature = "__allow_empty"))]
        if frameworks.is_empty() {
            panic!(
                "apple-sys accepts module names as features. Empty feature maybe a mistake. If this is intentional, add to features: '__allow_empty'"
            );
        }

        let mut deps = load_deps();
        let out_dir =
            PathBuf::from(std::env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

        // Get SDK path for header scanning (needed before toposort for sub-fw detection)
        let sdk_path: SdkPath = (&platform).try_into().expect("sdk lookup failed");

        // Auto-detect umbrella→sub-framework relationships from the SDK filesystem
        // and merge them into the deps map so topological_sort orders them correctly.
        let framework_set: HashSet<&str> = frameworks.iter().copied().collect();
        for &fw in &frameworks {
            let subs = scan_sub_frameworks(sdk_path.path(), fw);
            for sub in &subs {
                if framework_set.contains(sub.as_str()) {
                    // Umbrella fw depends on sub-fw → sub must come first in toposort
                    deps.entry(fw.to_string()).or_default().push(sub.clone());
                }
            }
            // Deduplicate
            if let Some(dep_list) = deps.get_mut(fw) {
                dep_list.sort();
                dep_list.dedup();
            }
        }

        let sorted_frameworks = topological_sort(&frameworks, &deps);

        // Cache directories
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let symbol_cache_dir = manifest_dir.join("../apple-bindgen/symbol-cache");
        let cache_key = CacheKey::current();
        let bindgen_cache_dir = manifest_dir
            .join(".bindgen-cache")
            .join(cache_key.cache_subdir());

        // Link all frameworks
        for framework in &sorted_frameworks {
            println!("cargo:rustc-link-lib=framework={framework}");
        }

        // Try loading symbol cache (for Phase 2 skip)
        let symbol_cache: Option<HashMap<String, (HashSet<String>, Vec<String>)>> = {
            let mut all_cached = HashMap::new();
            let mut hit = true;
            for fw in &sorted_frameworks {
                if let Some(data) = load_cached_framework(&symbol_cache_dir, fw, &cache_key) {
                    all_cached.insert(fw.to_string(), data);
                } else {
                    hit = false;
                    break;
                }
            }
            // Also load "objc" virtual module from cache
            if hit {
                if let Some(data) = load_cached_framework(&symbol_cache_dir, "objc", &cache_key) {
                    all_cached.insert("objc".to_string(), data);
                } else {
                    hit = false;
                }
            }
            // Verify cached deps only reference frameworks in the current build.
            // A cache generated from a larger framework set may assign symbol
            // ownership differently (e.g. GLenum → OpenGLES vs OpenGL).
            if hit {
                let built: HashSet<&str> = sorted_frameworks.iter().map(|s| s.as_str()).collect();
                for (fw, (_, dep_list)) in &all_cached {
                    for dep in dep_list {
                        if dep != "objc" && !built.contains(dep.as_str()) {
                            eprintln!(
                                "Symbol cache stale: {fw} depends on {dep} \
                                 which is not in the current framework set"
                            );
                            hit = false;
                            break;
                        }
                    }
                    if !hit {
                        break;
                    }
                }
            }
            if hit {
                eprintln!("Symbol cache hit ({} frameworks)", all_cached.len());
                Some(all_cached)
            } else {
                None
            }
        };
        let symbol_cache_hit = symbol_cache.is_some();

        // Phase 1: Generate bindings (with bindgen cache)
        let t1 = Instant::now();
        let phase1: HashMap<String, (String, DependencyGraphs)> = sorted_frameworks
            .par_iter()
            .map(|framework| {
                let cache_file = bindgen_cache_dir.join(format!("{framework}.rs"));

                // Try bindgen cache
                let generated = if let Ok(cached) = std::fs::read_to_string(&cache_file) {
                    eprintln!("  Cached: {}", framework);
                    cached
                } else {
                    // Cache miss: run bindgen
                    let mut builder = Builder::with_builtin_config(framework, &platform)
                        .expect("sdk lookup failed");
                    if let Ok(target) = std::env::var("TARGET") {
                        builder = builder.target(target);
                    }
                    let code = builder.generate().expect("binding generation failed");

                    // Save to cache
                    let _ = std::fs::create_dir_all(&bindgen_cache_dir);
                    let _ = std::fs::write(&cache_file, &code);

                    eprintln!("  Generated: {}", framework);
                    code
                };

                // Build dependency graphs only when symbol cache missed (Phase 2 needs it)
                let graphs = if symbol_cache_hit {
                    DependencyGraphs {
                        definition_deps: HashMap::new(),
                        all_deps: HashMap::new(),
                    }
                } else {
                    build_dependency_graphs(&generated)
                };

                (framework.to_string(), (generated, graphs))
            })
            .collect();
        eprintln!("Phase 1 completed in {:?}", t1.elapsed());

        // Phase 2: Global ownership + reachability + dependency closure
        let t2 = Instant::now();
        let (framework_defined, framework_deps): (
            HashMap<String, HashSet<String>>,
            HashMap<String, Vec<String>>,
        ) = if let Some(cached) = symbol_cache {
            eprintln!("Phase 2: Using cached symbols");
            let mut defined = HashMap::new();
            let mut deps_map = HashMap::new();
            for (fw, (syms, dep_list)) in cached {
                defined.insert(fw.clone(), syms);
                deps_map.insert(fw, dep_list);
            }
            (defined, deps_map)
        } else {
            eprintln!("Phase 2: Computing unique symbols per framework...");
            compute_ownership(
                &sorted_frameworks,
                &phase1,
                &sdk_path,
                &symbol_cache_dir,
                &cache_key,
            )
        };
        eprintln!("Phase 2 completed in {:?}", t2.elapsed());

        // Filter cached dep lists to only include frameworks currently being built.
        // The symbol cache may reference frameworks not in this build's sorted_frameworks
        // (e.g. UIKit in a macOS-only build) when generated from a larger framework set.
        let built_set: HashSet<&str> = sorted_frameworks.iter().map(|s| s.as_str()).collect();
        let framework_deps: HashMap<String, Vec<String>> = framework_deps
            .into_iter()
            .map(|(fw, deps)| {
                let filtered: Vec<String> = deps
                    .into_iter()
                    .filter(|d| d == "objc" || built_set.contains(d.as_str()))
                    .collect();
                (fw, filtered)
            })
            .collect();

        // Scan system headers for POSIX types (pid_t, uid_t, etc.) — these are provided by libc
        let system_types = scan_system_types(sdk_path.path());

        // Phase 3: Filter and write
        let t3 = Instant::now();
        eprintln!(
            "Phase 3: Filtering and writing {} files...",
            sorted_frameworks.len()
        );

        // Generate the "objc" virtual module from the first framework's raw output.
        // This extracts ObjC runtime symbols (BOOL, Class, Protocol, NSObject, etc.)
        // into a dedicated module so other frameworks import them via `use crate::objc::*`.
        if let Some(objc_unique) = framework_defined.get("objc") {
            let first_fw = &sorted_frameworks[0];
            let (generated, _) = phase1.get(first_fw).unwrap();
            let objc_filtered = filter_to_reachable(generated, objc_unique, &[], Some(objc_unique));
            let objc_path = out_dir.join("objc.rs");
            let mut file = std::fs::File::create(&objc_path).expect("could not create objc.rs");
            file.write_all(objc_filtered.as_bytes())
                .expect("could not write objc.rs");
            eprintln!("  Wrote objc.rs ({} bytes)", objc_filtered.len());
        } else {
            // Empty fallback (e.g. __allow_empty with no frameworks)
            std::fs::write(out_dir.join("objc.rs"), "").unwrap();
        }

        for framework in &sorted_frameworks {
            let (generated, _) = phase1.get(framework).unwrap();
            let unique = framework_defined.get(framework).unwrap();
            let dep_frameworks = framework_deps.get(framework).cloned().unwrap_or_default();

            // Build available set: own unique symbols + all dep framework symbols.
            // This is passed to filter_to_reachable so it can filter out impl blocks
            // that reference types not available in this framework's dep chain.
            let mut available_for_filter: HashSet<String> = unique.clone();
            for dep_fw in &dep_frameworks {
                if let Some(dep_syms) = framework_defined.get(dep_fw) {
                    available_for_filter.extend(dep_syms.iter().cloned());
                }
            }
            let mut filtered = filter_to_reachable(
                generated,
                unique,
                &dep_frameworks,
                Some(&available_for_filter),
            );

            // Replace C integer alias references with Rust primitives.
            // Build list from system_types + well-known __uint*_t / __int*_t
            // types that bindgen may emit from arch-specific headers.
            let mut integer_aliases: Vec<_> = system_types
                .iter()
                .filter_map(|s| c_integer_primitive(s).map(|p| (s.clone(), p)))
                .collect();
            // bindgen may emit __uint8_t, __int32_t, etc. from <arch/_types.h>.
            // These don't appear in sys/_types/ so add them explicitly.
            for base in &[
                "__uint8_t",
                "__uint16_t",
                "__uint32_t",
                "__uint64_t",
                "__int8_t",
                "__int16_t",
                "__int32_t",
                "__int64_t",
                "uint8_t",
                "uint16_t",
                "uint32_t",
                "uint64_t",
            ] {
                if let Some(p) = c_integer_primitive(base) {
                    integer_aliases.push((base.to_string(), p));
                }
            }
            integer_aliases.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
            integer_aliases.dedup_by(|a, b| a.0 == b.0);
            for (sys_type, prim) in &integer_aliases {
                if filtered.contains(sys_type.as_str()) {
                    filtered = replace_whole_word(&filtered, sys_type, prim);
                }
            }

            // Replace __darwin_* intermediate typedefs with Rust primitives.
            for sys_type in system_types.iter() {
                if let Some(prim) = darwin_type_primitive(sys_type) {
                    if filtered.contains(sys_type.as_str()) {
                        filtered = replace_whole_word(&filtered, sys_type, prim);
                    }
                }
            }

            // Prepend libc imports for system types referenced in filtered output.
            let needed: Vec<&str> = system_types
                .iter()
                .filter(|ty| {
                    !ty.starts_with("__darwin_")
                        && !is_builtin(ty)
                        && !is_libc_missing(ty)
                        && filtered.contains(ty.as_str())
                })
                .map(|s| s.as_str())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect();
            let mut final_output = String::new();

            // Emit local typedefs for system types not available in libc.
            for (ty, def) in LIBC_MISSING_TYPES {
                if filtered.contains(*ty) {
                    final_output.push_str(&format!("pub type {ty} = {def};\n"));
                }
            }

            if !needed.is_empty() {
                final_output.push_str("#[allow(unused_imports)]\nuse libc::{");
                final_output.push_str(&needed.join(", "));
                final_output.push_str("};\n");
            }
            final_output.push_str(&filtered);

            let mut file = std::fs::File::create(out_dir.join(format!("{framework}.rs")))
                .expect("could not open bindings file");
            file.write_all(final_output.as_bytes())
                .expect("could not write bindings");
        }
        eprintln!("Phase 3 completed in {:?}", t3.elapsed());

        // Phase 4: Run rustfmt on all generated files in parallel
        let t4 = Instant::now();
        let mut rs_files: Vec<_> = sorted_frameworks
            .iter()
            .map(|fw| out_dir.join(format!("{fw}.rs")))
            .collect();
        // Include objc.rs for rustfmt + objc2 migration
        rs_files.push(out_dir.join("objc.rs"));

        if !rs_files.is_empty() {
            eprintln!(
                "Phase 4: Running rustfmt on {} files ({} parallel jobs)...",
                rs_files.len(),
                num_threads
            );

            let file_list: Vec<_> = rs_files
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            let status = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "echo '{}' | tr ' ' '\\n' | xargs -P {} -I {{}} rustfmt {{}}",
                    file_list.join(" "),
                    num_threads
                ))
                .status();

            match status {
                Ok(s) if s.success() => eprintln!("rustfmt completed"),
                Ok(s) => eprintln!("rustfmt exited with: {}", s),
                Err(e) => eprintln!("Failed to run rustfmt: {}", e),
            }
        }
        eprintln!("Phase 4 completed in {:?}", t4.elapsed());

        // Phase 5: Convert objc 0.2 → objc2 0.6 syntax.
        // Runs after rustfmt so the code has predictable formatting for
        // pattern matching (e.g. `use objc::{...};` on a single line).
        for path in &rs_files {
            let source = std::fs::read_to_string(path).unwrap();
            let migrated = apple_bindgen::objc2::migrate(&source);
            if migrated != source {
                std::fs::write(path, migrated).unwrap();
            }
        }

        // Phase 6: Post-migration fixups. Now that msg_send uses the objc2 format
        // (msg_send!(&*receiver, sel : arg, sel : arg)), we can reliably detect
        // and fix parameter/type name collisions.
        //
        // Build a global set of all pub struct/const names across all framework files
        // so that imported types (e.g. CGContext from CoreGraphics used in AddressBook)
        // are also detected as shadow names.
        let global_shadow_names: HashSet<String> = rs_files
            .iter()
            .flat_map(|path| {
                let source = std::fs::read_to_string(path).unwrap_or_default();
                super::collect_shadow_names(&source)
            })
            .collect();

        for path in &rs_files {
            let source = std::fs::read_to_string(path).unwrap();
            let fixed = super::strip_incompatible_msg_send(&source);
            let fixed = super::fix_msg_send_type_collisions(&fixed, &global_shadow_names);
            let fixed = super::fix_duplicate_params(&fixed);
            if fixed.len() != source.len() {
                let fw = path.file_stem().unwrap().to_string_lossy();
                eprintln!(
                    "  {fw}: post-migration fixups applied ({} byte delta)",
                    source.len() as i64 - fixed.len() as i64
                );
                std::fs::write(path, fixed).unwrap();
            }
        }

        eprintln!("Build complete!");
    }

    /// Compute ownership-based unique symbols per framework and save to cache.
    fn compute_ownership(
        sorted_frameworks: &[String],
        phase1: &HashMap<String, (String, DependencyGraphs)>,
        sdk_path: &SdkPath,
        cache_dir: &PathBuf,
        cache_key: &CacheKey,
    ) -> (
        HashMap<String, HashSet<String>>,
        HashMap<String, Vec<String>>,
    ) {
        // Step 1: Scan all framework headers + TBDs.
        //
        // We maintain two separate maps:
        // - `bfs_seeds`: headers + own TBD per framework — used as BFS starting points.
        //   Excludes sub-framework TBDs which contain many internal/re-exported symbols
        //   that would over-expand BFS reach in umbrella frameworks (e.g., Carbon).
        // - `all_owned`: headers + all TBDs per framework — used for global ownership.
        let scanned: HashMap<String, FrameworkSymbols> = sorted_frameworks
            .par_iter()
            .map(|fw| (fw.to_string(), scan_framework_headers(sdk_path.path(), fw)))
            .collect();

        let mut bfs_seeds: HashMap<String, HashSet<String>> = HashMap::new();
        let mut all_owned: HashMap<String, HashSet<String>> = HashMap::new();
        for (fw, fs) in &scanned {
            bfs_seeds.insert(fw.clone(), fs.bfs_seeds());
            all_owned.insert(fw.clone(), fs.all_symbols());
        }

        // Scan ObjC runtime headers (/usr/include/objc/) for the virtual "objc" module.
        // These symbols (BOOL, Class, Protocol, NSObject, etc.) are not part of any
        // framework but bindgen includes them from system headers. A dedicated module
        // prevents name collisions (e.g., PCSC also defines BOOL as i16).
        let objc_owned = scan_objc_headers(sdk_path.path());
        eprintln!(
            "  ObjC runtime types: {} symbols discovered",
            objc_owned.len()
        );
        bfs_seeds.insert("objc".to_string(), objc_owned.clone());
        all_owned.insert("objc".to_string(), objc_owned);

        // Build global ownership: symbol → owning framework.
        // "objc" is registered FIRST so its symbols take priority over any framework
        // that happens to declare identically-named types (e.g., PCSC's BOOL).
        let mut global_ownership: HashMap<String, String> = HashMap::new();
        if let Some(owned) = all_owned.get("objc") {
            for sym in owned {
                global_ownership.insert(sym.clone(), "objc".to_string());
            }
        }
        for framework in sorted_frameworks {
            if let Some(owned) = all_owned.get(framework) {
                for sym in owned {
                    global_ownership
                        .entry(sym.clone())
                        .or_insert_with(|| framework.to_string());
                }
            }
        }

        // Build set of all framework names being built (includes "objc" virtual module)
        let mut built_frameworks: HashSet<&str> =
            sorted_frameworks.iter().map(|s| s.as_str()).collect();
        built_frameworks.insert("objc");

        // Scan system headers for POSIX types
        let system_types = scan_system_types(sdk_path.path());
        eprintln!(
            "  System types (from libc): {} types discovered",
            system_types.len()
        );

        // Step 2: For each framework, compute unique symbols with dependency closure.
        // Uses `all_deps` for BFS reachability (to discover all needed symbols),
        // but `definition_deps` for the closure (to avoid impl block deps
        // removing base types like NSString due to cross-framework ObjC categories).
        let mut framework_defined: HashMap<String, HashSet<String>> = HashMap::new();
        let mut already_assigned: HashSet<String> = HashSet::new();

        // Process the virtual "objc" module first.
        // Uses the first framework's (CoreFoundation) dep graph for BFS, since
        // bindgen generates ObjC runtime types in every ObjC-capable framework.
        if !sorted_frameworks.is_empty() {
            let first_fw = &sorted_frameworks[0];
            let (_, graphs) = phase1.get(first_fw).unwrap();
            let objc_symbols = bfs_seeds.get("objc").unwrap();

            let reachable = compute_reachable(&graphs.all_deps, objc_symbols);

            let mut objc_unique: HashSet<String> = reachable
                .iter()
                .filter(|sym| {
                    if system_types.contains(*sym) && !is_builtin(sym) {
                        return false;
                    }
                    match global_ownership.get(*sym) {
                        Some(owner) if owner == "objc" => true,
                        _ => false,
                    }
                })
                .cloned()
                .collect();

            // Dependency closure: remove symbols whose definition deps reference
            // types not available within the objc module (e.g., functions that
            // take NSString* or other Foundation types as parameters).
            loop {
                let to_remove: HashSet<String> = objc_unique
                    .iter()
                    .filter(|sym| {
                        if let Some(sym_deps) = graphs.definition_deps.get(*sym) {
                            sym_deps.iter().any(|dep| {
                                !is_builtin(dep)
                                    && !system_types.contains(dep)
                                    && !objc_unique.contains(dep)
                            })
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();
                if to_remove.is_empty() {
                    break;
                }
                for sym in &to_remove {
                    objc_unique.remove(sym);
                }
            }

            eprintln!(
                "  objc: {} owned, {} reachable, {} unique",
                objc_symbols.len(),
                reachable.len(),
                objc_unique.len(),
            );

            already_assigned.extend(objc_unique.iter().cloned());
            framework_defined.insert("objc".to_string(), objc_unique);
        }

        for (idx, framework) in sorted_frameworks.iter().enumerate() {
            let (_, graphs) = phase1.get(framework).unwrap();
            let owned_symbols = all_owned.get(framework).unwrap();
            let seed_symbols = bfs_seeds.get(framework).unwrap();

            // BFS reachability uses bfs_seeds (headers + own TBD, excluding
            // sub-framework TBDs which can over-expand reach in umbrella frameworks).
            let mut reachable = compute_reachable(&graphs.all_deps, seed_symbols);

            // Include sub-framework TBD functions directly in reachable (without BFS
            // expansion). These are legitimate exports but excluded from BFS seeds to
            // prevent orphan type claiming. Their type deps are resolved via the
            // dependency closure against `available` (which includes prior frameworks).
            if let Some(fs) = scanned.get(framework) {
                for sym in &fs.sub_tbd_symbols {
                    reachable.insert(sym.clone());
                }
            }

            let mut unique: HashSet<String> = reachable
                .iter()
                .filter(|sym| {
                    if system_types.contains(*sym) && !is_builtin(sym) {
                        return false;
                    }
                    match global_ownership.get(*sym) {
                        Some(owner) if owner == framework => true,
                        Some(owner) if built_frameworks.contains(owner.as_str()) => {
                            // If the owning framework was already processed (in
                            // framework_defined) but dropped this symbol during
                            // dependency closure, it won't be in already_assigned,
                            // so allow this framework to pick it up.
                            if framework_defined.contains_key(owner) {
                                !already_assigned.contains(*sym)
                            } else {
                                // Owner hasn't processed yet — respect ownership
                                false
                            }
                        }
                        Some(_) => !already_assigned.contains(*sym),
                        None => !already_assigned.contains(*sym),
                    }
                })
                .cloned()
                .collect();

            let mut available: HashSet<String> = unique.clone();
            // Include "objc" virtual module symbols in available set
            if let Some(objc_syms) = framework_defined.get("objc") {
                available.extend(objc_syms.iter().cloned());
            }
            for prev_fw in &sorted_frameworks[..idx] {
                if let Some(prev_syms) = framework_defined.get(prev_fw) {
                    available.extend(prev_syms.iter().cloned());
                }
            }

            // Dependency closure uses definition_deps only.
            // This way, impl block deps (e.g., NSView from AppKit category on NSString)
            // don't cause NSString to be removed from Foundation.
            //
            // A dep is "truly unsatisfied" only if it has no known provider among
            // built frameworks. If the provider exists but hasn't been processed yet
            // (later in topological order), the dep will be resolved via `use crate::*`
            // imports at the filter_to_reachable stage.
            loop {
                let to_remove: HashSet<String> = unique
                    .iter()
                    .filter(|sym| {
                        if let Some(sym_deps) = graphs.definition_deps.get(*sym) {
                            sym_deps.iter().any(|dep| {
                                !is_builtin(dep)
                                    && !system_types.contains(dep)
                                    && !available.contains(dep)
                                    && !unique.contains(dep)
                                    // Keep the symbol if the dep is owned by a known
                                    // framework that will be built (even if not yet
                                    // processed). It will be importable at link time.
                                    && !global_ownership
                                        .get(dep)
                                        .map_or(false, |owner| built_frameworks.contains(owner.as_str()))
                            })
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();
                if to_remove.is_empty() {
                    break;
                }
                for sym in &to_remove {
                    unique.remove(sym);
                    available.remove(sym);
                }
            }

            eprintln!(
                "  {}: {} owned, {} reachable, {} unique",
                framework,
                owned_symbols.len(),
                reachable.len(),
                unique.len(),
            );

            already_assigned.extend(unique.iter().cloned());
            framework_defined.insert(framework.to_string(), unique);
        }

        // Build reverse map: symbol → defining framework
        let mut symbol_to_framework: HashMap<&str, &str> = HashMap::new();
        for (fw, syms) in &framework_defined {
            for sym in syms {
                symbol_to_framework.insert(sym.as_str(), fw.as_str());
            }
        }

        // Compute dependency frameworks for each framework and save to cache
        let mut framework_deps: HashMap<String, Vec<String>> = HashMap::new();

        for framework in sorted_frameworks {
            let (_, graphs) = phase1.get(framework).unwrap();
            let unique = framework_defined.get(framework).unwrap();

            // Discover needed frameworks from all_deps (definition + impl block deps)
            // of unique symbols. Uses direct deps (not BFS) to avoid false positives.
            // all_deps includes impl block deps (e.g., trait conformance references)
            // which are needed so filter_to_reachable knows that foreign traits like
            // PNSSecureCoding are available via imports and should not be re-defined.
            let mut needed_frameworks: HashSet<&str> = HashSet::new();
            for sym in unique {
                if let Some(sym_deps) = graphs.all_deps.get(sym) {
                    for dep in sym_deps {
                        if unique.contains(dep) {
                            continue;
                        }
                        if let Some(&provider) = symbol_to_framework.get(dep.as_str()) {
                            if provider != framework.as_str() {
                                needed_frameworks.insert(provider);
                            }
                        }
                    }
                }
            }
            let mut dep_list: Vec<String> =
                needed_frameworks.iter().map(|s| s.to_string()).collect();
            dep_list.sort();

            save_cached_symbols(cache_dir, framework, cache_key, unique, &dep_list);

            framework_deps.insert(framework.to_string(), dep_list);
        }

        // Save "objc" virtual module to cache
        if let Some(objc_unique) = framework_defined.get("objc") {
            let empty_deps: Vec<String> = Vec::new();
            save_cached_symbols(cache_dir, "objc", cache_key, objc_unique, &empty_deps);
        }

        eprintln!(
            "  Saved symbol cache to {}",
            cache_dir.join(cache_key.cache_subdir()).display()
        );

        (framework_defined, framework_deps)
    }
}
