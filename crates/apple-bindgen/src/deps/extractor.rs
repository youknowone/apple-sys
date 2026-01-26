//! Extract type names from generated Rust binding files.

use regex::Regex;
use std::collections::HashSet;

/// Get the type prefixes for a given framework.
///
/// Returns a list of prefixes that identify types belonging to the framework.
/// Types without these prefixes (system types like `qos_class_t`) will be excluded.
pub fn framework_prefixes(framework: &str) -> Option<Vec<&'static str>> {
    match framework {
        "CoreFoundation" => Some(vec!["CF", "__CF"]),
        "Foundation" => Some(vec!["NS", "__NS"]),
        "AppKit" => Some(vec!["NS", "__NS"]),
        "CoreGraphics" => Some(vec!["CG", "__CG"]),
        "CoreText" => Some(vec!["CT", "__CT"]),
        "QuartzCore" => Some(vec!["CA", "__CA"]),
        "CoreData" => Some(vec!["NS", "__NS"]),
        "CoreServices" => Some(vec!["LS", "UT", "MDItem", "FSEvent", "AE", "Carbon"]),
        "CoreImage" => Some(vec!["CI", "__CI"]),
        "CoreMedia" => Some(vec!["CM", "__CM"]),
        "CoreVideo" => Some(vec!["CV", "__CV"]),
        "CoreAudio" => Some(vec!["Audio", "kAudio"]),
        "AVFoundation" => Some(vec!["AV", "__AV"]),
        "Metal" => Some(vec!["MTL", "__MTL"]),
        "IOKit" => Some(vec!["IO", "io_", "kIO"]),
        "Security" => Some(vec!["Sec", "CSSM", "kSec"]),
        "SystemConfiguration" => Some(vec!["SC", "kSC"]),
        "ImageIO" => Some(vec!["CGImage", "kCGImage"]),
        "ColorSync" => Some(vec!["ColorSync"]),
        _ => None, // No filtering for unknown frameworks - include all types
    }
}

/// Extract all public type names from a generated `.rs` binding file.
///
/// This parses the Rust source to find:
/// - `pub struct TypeName`
/// - `pub enum TypeName`
/// - `pub type TypeName = ...`
/// - `pub trait TypeName`
///
/// # Example
/// ```ignore
/// let rs_content = std::fs::read_to_string("Foundation.rs")?;
/// let types = extract_types_from_rs(&rs_content);
/// // types: ["NSString", "NSArray", "NSObject", ...]
/// ```
pub fn extract_types_from_rs(content: &str) -> Vec<String> {
    let mut types = HashSet::new();

    // pub struct NSString
    let struct_re = Regex::new(r"pub struct (\w+)").unwrap();
    for cap in struct_re.captures_iter(content) {
        types.insert(cap[1].to_string());
    }

    // pub enum NSComparisonResult
    let enum_re = Regex::new(r"pub enum (\w+)").unwrap();
    for cap in enum_re.captures_iter(content) {
        types.insert(cap[1].to_string());
    }

    // pub type NSInteger = ...
    let type_re = Regex::new(r"pub type (\w+)\s*=").unwrap();
    for cap in type_re.captures_iter(content) {
        types.insert(cap[1].to_string());
    }

    // pub trait NSCopying_ (ObjC protocols become traits)
    let trait_re = Regex::new(r"pub trait (\w+)").unwrap();
    for cap in trait_re.captures_iter(content) {
        types.insert(cap[1].to_string());
    }

    // pub union (rare but possible)
    let union_re = Regex::new(r"pub union (\w+)").unwrap();
    for cap in union_re.captures_iter(content) {
        types.insert(cap[1].to_string());
    }

    let mut result: Vec<_> = types.into_iter().collect();
    result.sort();
    result
}

/// Extract public type names filtered by framework-specific prefixes.
///
/// This extracts only types that belong to the specified framework,
/// filtering out system types like `qos_class_t`, `fenv_t`, etc.
///
/// # Example
/// ```ignore
/// let rs_content = std::fs::read_to_string("CoreFoundation.rs")?;
/// let types = extract_types_for_framework(&rs_content, "CoreFoundation");
/// // types: ["CFString", "CFArray", "CFDictionary", ...] (no system types)
/// ```
pub fn extract_types_for_framework(content: &str, framework: &str) -> Vec<String> {
    let all_types = extract_types_from_rs(content);

    match framework_prefixes(framework) {
        Some(prefixes) => all_types
            .into_iter()
            .filter(|t| prefixes.iter().any(|p| t.starts_with(p)))
            .collect(),
        None => all_types, // No filtering for unknown frameworks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_struct() {
        let content = r#"
            pub struct NSString {
                _data: [u8; 0],
            }
            pub struct NSArray {
                _data: [u8; 0],
            }
        "#;
        let types = extract_types_from_rs(content);
        assert!(types.contains(&"NSString".to_string()));
        assert!(types.contains(&"NSArray".to_string()));
    }

    #[test]
    fn test_extract_enum() {
        let content = r#"
            pub enum NSComparisonResult {
                NSOrderedAscending = -1,
                NSOrderedSame = 0,
                NSOrderedDescending = 1,
            }
        "#;
        let types = extract_types_from_rs(content);
        assert!(types.contains(&"NSComparisonResult".to_string()));
    }

    #[test]
    fn test_extract_type_alias() {
        let content = r#"
            pub type NSInteger = ::std::os::raw::c_long;
            pub type CGFloat = f64;
        "#;
        let types = extract_types_from_rs(content);
        assert!(types.contains(&"NSInteger".to_string()));
        assert!(types.contains(&"CGFloat".to_string()));
    }

    #[test]
    fn test_extract_trait() {
        let content = r#"
            pub trait NSCopying_ {
                fn copy(&self) -> id;
            }
            pub trait NSCoding_ {
                fn encodeWithCoder_(&self, coder: id);
            }
        "#;
        let types = extract_types_from_rs(content);
        assert!(types.contains(&"NSCopying_".to_string()));
        assert!(types.contains(&"NSCoding_".to_string()));
    }

    #[test]
    fn test_no_duplicates() {
        let content = r#"
            pub struct NSString {}
            pub struct NSString {}
        "#;
        let types = extract_types_from_rs(content);
        assert_eq!(types.iter().filter(|t| *t == "NSString").count(), 1);
    }
}
