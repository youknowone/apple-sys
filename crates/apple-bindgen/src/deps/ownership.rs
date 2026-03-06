//! Framework ownership determination via SDK TBD files and header scanning.
//!
//! Uses a hybrid approach:
//! - **TBD files** (text-based stubs) for functions and variables — these list
//!   every exported linker symbol and are the ground truth for what a framework exports.
//! - **Header scanning** for types only (@interface, @protocol, typedef, struct, enum) —
//!   these are not linker symbols and only exist in headers.
//!
//! The two sources serve different purposes:
//! - **Header symbols** are used as BFS seeds in reachability analysis. Since headers
//!   contain types declared by public API, they anchor each framework's scope.
//! - **TBD symbols** are registered in the global ownership map for attribution.
//!   They are NOT used as BFS seeds because TBD includes many internal/re-exported
//!   symbols that would cause BFS to reach unrelated orphan types.

use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

/// Result of scanning a framework's SDK artifacts.
#[derive(Debug, Clone)]
pub struct FrameworkSymbols {
    /// Symbols from headers (types: struct, enum, typedef, ObjC class/protocol).
    pub header_symbols: HashSet<String>,
    /// Symbols from the framework's own TBD file (exported functions/variables).
    pub own_tbd_symbols: HashSet<String>,
    /// Symbols from sub-framework TBD files (exported functions/variables).
    /// Separated because sub-framework TBDs include many internal symbols
    /// that should not be used as BFS seeds (to prevent orphan type claiming).
    pub sub_tbd_symbols: HashSet<String>,
}

impl FrameworkSymbols {
    /// Symbols suitable for BFS seeding: headers + own TBD.
    /// Excludes sub-framework TBDs which can over-expand BFS reach
    /// in umbrella frameworks (e.g., Carbon/HIToolbox).
    pub fn bfs_seeds(&self) -> HashSet<String> {
        self.header_symbols
            .union(&self.own_tbd_symbols)
            .cloned()
            .collect()
    }

    /// All symbols combined (headers + own TBD + sub-framework TBDs).
    pub fn all_symbols(&self) -> HashSet<String> {
        let mut all = self.bfs_seeds();
        all.extend(self.sub_tbd_symbols.iter().cloned());
        all
    }
}

/// Scan a framework's TBD file and headers to extract declared symbol names.
///
/// Returns a [`FrameworkSymbols`] with header symbols and TBD symbols separated.
///
/// For umbrella frameworks, also scans sub-framework TBD/headers IF the sub-framework
/// does NOT exist as a top-level framework.
pub fn scan_framework_headers(sdk_path: &Path, framework: &str) -> FrameworkSymbols {
    let top_level_fw_dir = sdk_path.join("System/Library/Frameworks");
    let framework_dir = top_level_fw_dir.join(format!("{}.framework", framework));

    let mut header_symbols = HashSet::new();
    let mut own_tbd_symbols = HashSet::new();
    let mut sub_tbd_symbols = HashSet::new();

    // 1. Extract functions/variables from the framework's own TBD file
    let tbd_path = framework_dir.join(format!("{}.tbd", framework));
    if tbd_path.exists() {
        scan_tbd_file(&tbd_path, &mut own_tbd_symbols);
    }

    // 2. Extract types from header files
    let headers_dir = framework_dir.join("Headers");
    if headers_dir.exists() {
        scan_headers_dir(&headers_dir, &mut header_symbols);
    } else {
        eprintln!(
            "Warning: Headers directory not found: {}",
            headers_dir.display()
        );
    }

    // 3. Scan sub-frameworks for those that are NOT top-level
    let sub_fw_dir = framework_dir.join("Frameworks");
    if sub_fw_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&sub_fw_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("framework") || !path.is_dir()
                {
                    continue;
                }
                if let Some(sub_name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Skip if this sub-framework also exists as a top-level framework
                    let top_level = top_level_fw_dir.join(format!("{}.framework", sub_name));
                    if top_level.is_dir() {
                        continue;
                    }
                    // Sub-framework TBD → separate set (not used as BFS seeds)
                    let sub_tbd = path.join(format!("{}.tbd", sub_name));
                    if sub_tbd.exists() {
                        scan_tbd_file(&sub_tbd, &mut sub_tbd_symbols);
                    }
                    // Sub-framework headers
                    let sub_headers = path.join("Headers");
                    if sub_headers.is_dir() {
                        scan_headers_dir(&sub_headers, &mut header_symbols);
                    }
                }
            }
        }
    }

    FrameworkSymbols {
        header_symbols,
        own_tbd_symbols,
        sub_tbd_symbols,
    }
}

/// Parse a TBD (text-based stub) file to extract exported symbol names.
///
/// TBD files are YAML-based and contain `symbols: [ _Foo, _Bar, ... ]` entries
/// listing all exported linker symbols. Symbol names are prefixed with `_` in the
/// file and we strip that prefix.
///
/// We skip linker directive symbols (`$ld$hide$`, `$ld$add$`) and ObjC metadata
/// symbols (`_OBJC_CLASS_$_`, `_OBJC_METACLASS_$_`).
fn scan_tbd_file(tbd_path: &Path, symbols: &mut HashSet<String>) {
    let content = match std::fs::read_to_string(tbd_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Warning: Could not read TBD file {}: {}",
                tbd_path.display(),
                e
            );
            return;
        }
    };
    scan_tbd_file_from_content(&content, symbols);
}

/// Parse TBD content string to extract exported symbol names.
fn scan_tbd_file_from_content(content: &str, symbols: &mut HashSet<String>) {
    // Only parse the first YAML document (the framework itself).
    // Multi-document TBD files (umbrella frameworks) embed sub-framework
    // documents after the first `---`, but we scan those separately.
    let first_doc = if content.len() > 4 {
        if let Some(second_start) = content[4..].find("\n--- ") {
            &content[..second_start + 4]
        } else {
            content
        }
    } else {
        content
    };

    // Extract symbols from `symbols: [ ... ]` arrays
    extract_tbd_array(first_doc, "symbols:", symbols, |entry| {
        // Skip linker directives and ObjC metadata
        if entry.starts_with("$ld$")
            || entry.starts_with("_OBJC_CLASS_$_")
            || entry.starts_with("_OBJC_METACLASS_$_")
            || entry.starts_with(".objc_class_name_")
        {
            return None;
        }
        // Strip leading underscore (C symbol mangling convention)
        let name = entry.strip_prefix('_').unwrap_or(entry);
        if name.is_empty() {
            return None;
        }
        Some(name.to_string())
    });
}

/// Extract entries from a TBD YAML array field.
///
/// Finds lines matching `field_name [ ... ]` and parses the comma-separated
/// entries, which may span multiple lines until the closing `]`.
fn extract_tbd_array(
    content: &str,
    field_name: &str,
    symbols: &mut HashSet<String>,
    transform: impl Fn(&str) -> Option<String>,
) {
    let mut pos = 0;

    // Find all occurrences of the field name
    while let Some(field_start) = content[pos..].find(field_name) {
        let abs_start = pos + field_start + field_name.len();
        pos = abs_start;

        // Find the opening bracket
        let bracket_start = match content[abs_start..].find('[') {
            Some(i) => abs_start + i + 1,
            None => continue,
        };

        // Find the closing bracket
        let bracket_end = match content[bracket_start..].find(']') {
            Some(i) => bracket_start + i,
            None => continue,
        };

        pos = bracket_end + 1;

        // Parse comma-separated entries between [ and ]
        let array_content = &content[bracket_start..bracket_end];
        for entry in array_content.split(',') {
            let entry = entry.trim().trim_matches('\'').trim_matches('"');
            if entry.is_empty() {
                continue;
            }
            if let Some(name) = transform(entry) {
                symbols.insert(name);
            }
        }
    }
}

/// Scan ObjC runtime headers (`{sdk_path}/usr/include/objc/`) to extract symbol names.
///
/// Returns symbols declared in the ObjC runtime system headers (BOOL, Class,
/// Protocol, NSObject, etc.). These are not part of any framework but are
/// included by bindgen via its objc allowlist. A dedicated "objc" virtual module
/// owns them so they don't collide with identically-named symbols in other
/// frameworks (e.g., PCSC defines `BOOL` as `i16` for its Smart Card API).
pub fn scan_objc_headers(sdk_path: &Path) -> HashSet<String> {
    let objc_dir = sdk_path.join("usr/include/objc");
    let mut symbols = HashSet::new();
    if objc_dir.is_dir() {
        scan_headers_dir(&objc_dir, &mut symbols);
    }
    symbols
}

/// Discover sub-frameworks inside an umbrella framework.
///
/// Returns the names of sub-frameworks found in
/// `{sdk_path}/System/Library/Frameworks/{framework}.framework/Frameworks/*.framework`.
///
/// For example, `scan_sub_frameworks(sdk, "ApplicationServices")` returns
/// `["ATS", "ATSUI", "ColorSync", "CoreGraphics", "CoreText", "HIServices", ...]`.
pub fn scan_sub_frameworks(sdk_path: &Path, framework: &str) -> Vec<String> {
    let sub_fw_dir = sdk_path
        .join("System/Library/Frameworks")
        .join(format!("{}.framework", framework))
        .join("Frameworks");

    let mut subs = Vec::new();
    if !sub_fw_dir.is_dir() {
        return subs;
    }

    if let Ok(entries) = std::fs::read_dir(&sub_fw_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("framework") && path.is_dir() {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    subs.push(name.to_string());
                }
            }
        }
    }

    subs.sort();
    subs
}

/// Scan all `.h` files in a directory and extract symbol names.
fn scan_headers_dir(headers_dir: &Path, symbols: &mut HashSet<String>) {
    let entries = match std::fs::read_dir(headers_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "Warning: Could not read headers directory {}: {}",
                headers_dir.display(),
                e
            );
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("h") {
            // Use read() + lossy conversion: some Apple headers are ISO-8859-1
            if let Ok(bytes) = std::fs::read(&path) {
                let content = String::from_utf8_lossy(&bytes);
                extract_symbols_from_header(&content, symbols);
            }
        }
    }
}

/// Extract type names from a single header file content.
///
/// Only extracts types: @interface, @protocol, typedef, struct, enum, union.
/// Function and variable symbols are obtained from TBD files instead.
fn extract_symbols_from_header(content: &str, symbols: &mut HashSet<String>) {
    let patterns = HeaderPatterns::new();

    // State for multi-line typedef struct/enum/union tracking
    let mut in_typedef_block = false;
    let mut typedef_brace_depth = 0i32;

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle multi-line typedef block continuation
        if in_typedef_block {
            typedef_brace_depth += trimmed.chars().filter(|&c| c == '{').count() as i32;
            typedef_brace_depth -= trimmed.chars().filter(|&c| c == '}').count() as i32;

            if typedef_brace_depth <= 0 {
                in_typedef_block = false;
                // Extract name from "} Name;" or "} Name API_AVAILABLE(...);"
                if let Some(caps) = patterns.typedef_block_end.captures(trimmed) {
                    if let Some(name) = caps.get(1) {
                        let n = name.as_str();
                        if !is_noise_symbol(n) {
                            symbols.insert(n.to_string());
                        }
                    }
                }
            }
            continue;
        }

        // Skip preprocessor directives and comments
        if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') {
            continue;
        }

        // Check for multi-line typedef struct/enum/union start
        if trimmed.starts_with("typedef ")
            && (trimmed.contains("struct") || trimmed.contains("enum") || trimmed.contains("union"))
            && !trimmed.contains(';')
        {
            let open = trimmed.chars().filter(|&c| c == '{').count() as i32;
            let close = trimmed.chars().filter(|&c| c == '}').count() as i32;

            if open > close || (open == 0 && close == 0) {
                in_typedef_block = true;
                typedef_brace_depth = open - close;

                if let Some(caps) = patterns.typedef_struct_name.captures(trimmed) {
                    if let Some(name) = caps.get(1) {
                        let n = name.as_str();
                        if !is_noise_symbol(n) {
                            symbols.insert(n.to_string());
                        }
                    }
                }
                continue;
            }
        }

        // @interface ClassName (but NOT category extensions like @interface ClassName (Category))
        if let Some(caps) = patterns.interface.captures(trimmed) {
            if let Some(name) = caps.get(1) {
                let after_name = trimmed[name.end()..].trim_start();
                if !after_name.starts_with('(') {
                    let n = name.as_str();
                    symbols.insert(n.to_string());
                    symbols.insert(format!("I{n}"));
                }
            }
        }

        // @protocol ProtocolName
        if let Some(caps) = patterns.protocol.captures(trimmed) {
            if let Some(name) = caps.get(1) {
                let n = name.as_str();
                if n != "NSObject" || !trimmed.contains(';') {
                    symbols.insert(n.to_string());
                    symbols.insert(format!("P{n}"));
                }
            }
        }

        // typedef MACRO(arg1, arg2) where MACRO contains ENUM/OPTIONS/CLOSED
        // Both arguments are captured; noise symbols (primitive types) are filtered.
        if let Some(caps) = patterns.ns_enum.captures(trimmed) {
            for i in 1..=2 {
                if let Some(m) = caps.get(i) {
                    let n = m.as_str();
                    if !is_noise_symbol(n) {
                        symbols.insert(n.to_string());
                    }
                }
            }
        }

        // typedef ... Name; (simple typedefs)
        if let Some(caps) = patterns.typedef_simple.captures(trimmed) {
            if let Some(name) = caps.get(1) {
                let n = name.as_str();
                if !is_noise_symbol(n) {
                    symbols.insert(n.to_string());
                }
            }
        }

        // Extract struct/enum/union name from single-line typedefs
        if let Some(caps) = patterns.typedef_struct_name.captures(trimmed) {
            if let Some(name) = caps.get(1) {
                let n = name.as_str();
                if !is_noise_symbol(n) {
                    symbols.insert(n.to_string());
                }
            }
        }
    }
}

/// Check if a symbol name is a common false positive from regex matching
fn is_noise_symbol(name: &str) -> bool {
    // Single character or very short names
    if name.len() <= 1 {
        return true;
    }
    // C primitive types and common SDK base types that should not be registered
    // as framework-owned symbols (they're either builtin or defined elsewhere).
    matches!(
        name,
        "void"
            | "int"
            | "char"
            | "long"
            | "short"
            | "float"
            | "double"
            | "unsigned"
            | "signed"
            | "bool"
            | "size_t"
            | "ssize_t"
            | "uint8_t"
            | "uint16_t"
            | "uint32_t"
            | "uint64_t"
            | "int8_t"
            | "int16_t"
            | "int32_t"
            | "int64_t"
            | "UInt8"
            | "UInt16"
            | "UInt32"
            | "UInt64"
            | "SInt8"
            | "SInt16"
            | "SInt32"
            | "SInt64"
            | "NSInteger"
            | "NSUInteger"
    )
}

/// Regex patterns for extracting type names from headers.
///
/// Only type-related patterns are needed — function/variable symbols
/// come from TBD files.
struct HeaderPatterns {
    interface: Regex,
    protocol: Regex,
    ns_enum: Regex,
    typedef_simple: Regex,
    typedef_struct_name: Regex,
    typedef_block_end: Regex,
}

impl HeaderPatterns {
    fn new() -> Self {
        Self {
            // @interface FOO or @interface FOO<T> or @interface FOO : BAR
            interface: Regex::new(r"^@interface\s+(\w+)").unwrap(),
            // @protocol FOO
            protocol: Regex::new(r"^@protocol\s+(\w+)").unwrap(),
            // typedef MACRO(arg1, arg2) where MACRO contains ENUM/OPTIONS/CLOSED.
            // Captures both arguments — one is the type, the other is the name.
            // Handles NS_ENUM(type, Name), CF_OPTIONS(type, Name),
            // VIMAGE_OPTIONS_ENUM(Name, type), vDSP_ENUM(type, Name), etc.
            ns_enum: Regex::new(
                r"typedef\s+\w*(?:ENUM|OPTIONS|CLOSED)\w*\s*\(\s*(\w+)\s*,\s*(\w+)\s*\)",
            )
            .unwrap(),
            // typedef ... Name; (captures last word before semicolon)
            typedef_simple: Regex::new(r"^typedef\s+.*?(\w+)\s*;").unwrap(),
            // Extract struct/enum/union name from typedef declarations
            typedef_struct_name: Regex::new(
                r"typedef\s+(?:struct|enum|union)\s+(?:\w+\s*\([^)]*\)\s+)*(\w+)",
            )
            .unwrap(),
            // End of multi-line typedef block: "} Name;" or "} Name API_AVAILABLE(...);"
            typedef_block_end: Regex::new(r"^\}\s*(\w+)\s*(?:;|API_|__attribute__)").unwrap(),
        }
    }
}

/// Scan SDK system headers to discover POSIX/Darwin types.
///
/// Reads filenames from `{sdk_path}/usr/include/sys/_types/` to extract
/// POSIX type names (e.g., `pid_t` from `_pid_t.h`) and their corresponding
/// Darwin intermediate typedefs (e.g., `__darwin_pid_t`).
pub fn scan_system_types(sdk_path: &Path) -> HashSet<String> {
    let types_dir = sdk_path.join("usr/include/sys/_types");
    let mut types = HashSet::new();

    let entries = match std::fs::read_dir(&types_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "Warning: Could not read system types directory {}: {}",
                types_dir.display(),
                e
            );
            return types;
        }
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        // _pid_t.h → pid_t + __darwin_pid_t
        if let Some(type_name) = name.strip_prefix('_').and_then(|n| n.strip_suffix(".h")) {
            if type_name.ends_with("_t") {
                types.insert(type_name.to_string());
                types.insert(format!("__darwin_{type_name}"));
            }
        }
    }

    types
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header("@interface NSString : NSObject", &mut symbols);
        assert!(symbols.contains("NSString"));
        assert!(
            symbols.contains("INSString"),
            "should also register I-prefix trait name"
        );
    }

    #[test]
    fn test_interface_with_generics() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "@interface NSArray<__covariant ObjectType> : NSObject",
            &mut symbols,
        );
        assert!(symbols.contains("NSArray"));
        assert!(
            symbols.contains("INSArray"),
            "should also register I-prefix trait name"
        );
    }

    #[test]
    fn test_protocol() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header("@protocol NSCoding", &mut symbols);
        assert!(symbols.contains("NSCoding"));
        assert!(
            symbols.contains("PNSCoding"),
            "should also register P-prefix trait name"
        );
    }

    #[test]
    fn test_ns_enum() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef NS_ENUM(NSUInteger, NSWindowStyleMask) {",
            &mut symbols,
        );
        assert!(symbols.contains("NSWindowStyleMask"));
    }

    #[test]
    fn test_cf_enum() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef CF_ENUM(CFStringEncoding, CFStringBuiltInEncodings) {",
            &mut symbols,
        );
        assert!(symbols.contains("CFStringBuiltInEncodings"));
    }

    #[test]
    fn test_tbd_symbols_extraction() {
        let mut symbols = HashSet::new();
        let tbd_content = r#"--- !tapi-tbd
tbd-version:     4
targets:         [ arm64-macos ]
install-name:    '/System/Library/Frameworks/Foo.framework/Versions/A/Foo'
exports:
  - targets:         [ arm64-macos ]
    symbols:         [ _FooCreate, _FooDestroy, _kFooDefault,
                       '$ld$hide$os10.7$_SCDynamicStoreCreate',
                       _OBJC_CLASS_$_FooObj, _OBJC_METACLASS_$_FooObj ]
"#;
        extract_tbd_array(tbd_content, "symbols:", &mut symbols, |entry| {
            if entry.starts_with("$ld$")
                || entry.starts_with("_OBJC_CLASS_$_")
                || entry.starts_with("_OBJC_METACLASS_$_")
            {
                return None;
            }
            let name = entry.strip_prefix('_').unwrap_or(entry);
            if name.is_empty() {
                return None;
            }
            Some(name.to_string())
        });
        assert!(symbols.contains("FooCreate"));
        assert!(symbols.contains("FooDestroy"));
        assert!(symbols.contains("kFooDefault"));
        assert!(
            !symbols.contains("SCDynamicStoreCreate"),
            "$ld$ should be filtered"
        );
        assert!(!symbols.contains("FooObj"), "OBJC_CLASS should be filtered");
    }

    #[test]
    fn test_tbd_multi_document_first_only() {
        let mut symbols = HashSet::new();
        let tbd_content = r#"--- !tapi-tbd
tbd-version:     4
targets:         [ arm64-macos ]
exports:
  - targets:         [ arm64-macos ]
    symbols:         [ _UmbrellaFunc ]
--- !tapi-tbd
tbd-version:     4
targets:         [ arm64-macos ]
exports:
  - targets:         [ arm64-macos ]
    symbols:         [ _SubFrameworkFunc ]
"#;
        scan_tbd_file_from_content(tbd_content, &mut symbols);
        assert!(symbols.contains("UmbrellaFunc"));
        assert!(
            !symbols.contains("SubFrameworkFunc"),
            "second document should not be parsed"
        );
    }

    #[test]
    fn test_interface_category_excluded() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "@interface NSURL(NSPasteboardSupport) <NSPasteboardWriting, NSPasteboardReading>",
            &mut symbols,
        );
        assert!(
            !symbols.contains("NSURL"),
            "category extension should not count as owned"
        );
        assert!(
            !symbols.contains("INSURL"),
            "category extension should not register I-prefix"
        );
    }

    #[test]
    fn test_interface_category_with_space_excluded() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header("@interface NSImage (Deprecated)", &mut symbols);
        assert!(
            !symbols.contains("NSImage"),
            "category extension with space should not count as owned"
        );
        assert!(
            !symbols.contains("INSImage"),
            "category extension should not register I-prefix"
        );
    }

    #[test]
    fn test_typedef_simple() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef const struct __CFString * CFStringRef;",
            &mut symbols,
        );
        assert!(symbols.contains("CFStringRef"));
    }

    #[test]
    fn test_multiline_typedef_struct() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef struct {\n    float x, y;\n} MTLSamplePosition;",
            &mut symbols,
        );
        assert!(
            symbols.contains("MTLSamplePosition"),
            "should capture name from multi-line typedef struct"
        );
    }

    #[test]
    fn test_multiline_typedef_struct_with_name() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef struct MTLRegion {\n    MTLOrigin origin;\n    MTLSize size;\n} MTLRegion;",
            &mut symbols,
        );
        assert!(
            symbols.contains("MTLRegion"),
            "should capture named struct from multi-line typedef"
        );
    }

    #[test]
    fn test_multiline_typedef_enum() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef enum {\n    kFoo = 0,\n    kBar = 1,\n} MyEnumName;",
            &mut symbols,
        );
        assert!(
            symbols.contains("MyEnumName"),
            "should capture name from multi-line typedef enum"
        );
    }

    #[test]
    fn test_typedef_struct_name_with_bridged_type() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef struct IIO_BRIDGED_TYPE(id) CGImageSource * CGImageSourceRef;",
            &mut symbols,
        );
        assert!(
            symbols.contains("CGImageSourceRef"),
            "should capture typedef name"
        );
        assert!(
            symbols.contains("CGImageSource"),
            "should also capture struct name from typedef"
        );
    }

    #[test]
    fn test_typedef_struct_name_simple() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header("typedef struct __CFString * CFStringRef;", &mut symbols);
        assert!(symbols.contains("CFStringRef"));
        assert!(
            symbols.contains("__CFString"),
            "should capture struct name from typedef"
        );
    }

    #[test]
    fn test_typedef_struct_two_bridged_macros() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef struct CF_BRIDGED_TYPE(id) CF_BRIDGED_MUTABLE_TYPE(IOSurface) __IOSurface *IOSurfaceRef;",
            &mut symbols,
        );
        assert!(symbols.contains("IOSurfaceRef"));
        assert!(
            symbols.contains("__IOSurface"),
            "should capture struct name with multiple bridged macros"
        );
    }

    #[test]
    fn test_multiline_typedef_struct_nsuinteger() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef struct {\n    NSUInteger width, height, depth;\n} MTLSize;",
            &mut symbols,
        );
        assert!(
            symbols.contains("MTLSize"),
            "should capture name from multi-line typedef struct with NSUInteger fields"
        );
    }

    #[test]
    fn test_scan_metal_has_struct_types() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let fs = scan_framework_headers(&sdk_path, "Metal");

        // Struct types come from headers
        assert!(
            fs.header_symbols.contains("MTLSamplePosition"),
            "should find MTLSamplePosition from Metal (multi-line typedef struct)"
        );
        assert!(
            fs.header_symbols.contains("MTLSize"),
            "should find MTLSize from Metal (multi-line typedef struct)"
        );
        assert!(
            fs.header_symbols.contains("MTLClearColor"),
            "should find MTLClearColor from Metal"
        );
    }

    #[test]
    fn test_scan_imageio_has_struct_names() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let fs = scan_framework_headers(&sdk_path, "ImageIO");

        // Struct types come from headers
        assert!(
            fs.header_symbols.contains("CGImageSourceRef"),
            "should find CGImageSourceRef from ImageIO"
        );
        assert!(
            fs.header_symbols.contains("CGImageSource"),
            "should find CGImageSource struct name from ImageIO (typedef struct ... CGImageSource *)"
        );
        assert!(
            fs.header_symbols.contains("CGImageDestination"),
            "should find CGImageDestination struct name from ImageIO"
        );
    }

    #[test]
    fn test_scan_foundation_functions() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let fs = scan_framework_headers(&sdk_path, "Foundation");

        // Functions come from TBD
        assert!(
            fs.own_tbd_symbols.contains("NSUserName"),
            "should find NSUserName from Foundation TBD"
        );
        assert!(
            fs.own_tbd_symbols.contains("NSFullUserName"),
            "should find NSFullUserName from Foundation TBD"
        );
        assert!(
            fs.own_tbd_symbols.contains("NSHomeDirectory"),
            "should find NSHomeDirectory from Foundation TBD"
        );
        assert!(
            fs.own_tbd_symbols.contains("NSTemporaryDirectory"),
            "should find NSTemporaryDirectory from Foundation TBD"
        );
        assert!(
            fs.own_tbd_symbols.contains("NSPageSize"),
            "should find NSPageSize from Foundation TBD"
        );
    }

    #[test]
    fn test_scan_metal_functions_and_structs() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let fs = scan_framework_headers(&sdk_path, "Metal");

        // Functions come from TBD
        assert!(
            fs.own_tbd_symbols.contains("MTLCreateSystemDefaultDevice"),
            "should find MTLCreateSystemDefaultDevice from Metal TBD"
        );
        assert!(
            fs.own_tbd_symbols.contains("MTLCopyAllDevices"),
            "should find MTLCopyAllDevices from Metal TBD"
        );
    }

    #[test]
    fn test_scan_application_services_sub_framework_ownership() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let fs = scan_framework_headers(&sdk_path, "ApplicationServices");
        let all = fs.all_symbols();

        // Sub-frameworks that are NOT top-level should be included
        assert!(
            all.contains("AXIsProcessTrusted"),
            "should find AXIsProcessTrusted (HIServices is sub-fw only, no top-level)"
        );
        assert!(
            all.contains("PMCreateSession"),
            "should find PMCreateSession (PrintCore is sub-fw only, no top-level)"
        );

        // Sub-frameworks that ARE also top-level should NOT be included
        assert!(
            !all.contains("ColorSyncProfileRef"),
            "should NOT find ColorSyncProfileRef (ColorSync is a top-level framework)"
        );
    }

    #[test]
    fn test_scan_sub_frameworks() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let subs = scan_sub_frameworks(&sdk_path, "ApplicationServices");
        assert!(
            subs.contains(&"ColorSync".to_string()),
            "ApplicationServices should have ColorSync sub-framework"
        );
        assert!(
            subs.contains(&"HIServices".to_string()),
            "ApplicationServices should have HIServices sub-framework"
        );
        assert!(
            subs.contains(&"PrintCore".to_string()),
            "ApplicationServices should have PrintCore sub-framework"
        );

        // Non-umbrella framework should have no sub-frameworks
        let no_subs = scan_sub_frameworks(&sdk_path, "Foundation");
        assert!(
            no_subs.is_empty(),
            "Foundation should have no sub-frameworks"
        );
    }

    #[test]
    fn test_scan_accelerate_vimage_types() {
        use std::path::PathBuf;
        use std::process::Command;

        let sdk_path = Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .expect("could not get SDK path");

        let fs = scan_framework_headers(&sdk_path, "Accelerate");

        // vImage types using custom macros (VIMAGE_OPTIONS_ENUM, VIMAGE_CHOICE_ENUM)
        assert!(
            fs.header_symbols.contains("vImage_Flags"),
            "should find vImage_Flags (VIMAGE_OPTIONS_ENUM)"
        );
        assert!(
            fs.header_symbols.contains("vImage_Error"),
            "should find vImage_Error (VIMAGE_CHOICE_ENUM)"
        );

        // Note: vDSP types use a single-char macro `n` (#define n CF_ENUM)
        // which can't be matched by raw header scanning. They're still available
        // through the TBD/dependency graph since clang preprocesses them.

        // Primitive types should NOT be registered as symbols
        assert!(
            !fs.header_symbols.contains("uint32_t"),
            "uint32_t should be filtered as noise"
        );
        assert!(
            !fs.header_symbols.contains("ssize_t"),
            "ssize_t should be filtered as noise"
        );
    }

    #[test]
    fn test_custom_enum_macro_extraction() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header(
            "typedef VIMAGE_OPTIONS_ENUM(vImage_Flags, uint32_t)",
            &mut symbols,
        );
        assert!(
            symbols.contains("vImage_Flags"),
            "should capture name from VIMAGE_OPTIONS_ENUM"
        );
        assert!(
            !symbols.contains("uint32_t"),
            "should NOT capture primitive type"
        );
    }

    #[test]
    fn test_vdsp_enum_extraction() {
        let mut symbols = HashSet::new();
        extract_symbols_from_header("typedef vDSP_ENUM(int, vDSP_DFT_Direction)", &mut symbols);
        assert!(
            symbols.contains("vDSP_DFT_Direction"),
            "should capture name from vDSP_ENUM"
        );
        assert!(
            !symbols.contains("int"),
            "should NOT capture primitive type"
        );
    }
}
