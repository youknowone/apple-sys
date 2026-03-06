//! Analyze framework dependencies from umbrella headers.

use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

/// Analyzes framework dependencies by parsing umbrella headers.
pub struct DependencyAnalyzer {
    sdk_path: std::path::PathBuf,
}

impl DependencyAnalyzer {
    /// Create a new analyzer for the given SDK path.
    pub fn new(sdk_path: impl AsRef<Path>) -> Self {
        Self {
            sdk_path: sdk_path.as_ref().to_path_buf(),
        }
    }

    /// Get the path to a framework's umbrella header.
    pub fn umbrella_header_path(&self, framework: &str) -> std::path::PathBuf {
        self.sdk_path
            .join("System/Library/Frameworks")
            .join(format!("{}.framework", framework))
            .join("Headers")
            .join(format!("{}.h", framework))
    }

    /// Analyze dependencies of a framework by parsing its umbrella header.
    ///
    /// Returns a list of framework names that this framework imports.
    pub fn analyze(&self, framework: &str) -> std::io::Result<Vec<String>> {
        let header_path = self.umbrella_header_path(framework);
        let content = std::fs::read_to_string(&header_path)?;
        Ok(extract_framework_imports(&content, framework))
    }
}

/// Extract framework names from import statements in header content.
///
/// Parses:
/// - `#import <Foundation/Foundation.h>` → "Foundation"
/// - `#include <CoreFoundation/CoreFoundation.h>` → "CoreFoundation"
/// - `@import Foundation;` → "Foundation"
fn extract_framework_imports(content: &str, self_framework: &str) -> Vec<String> {
    let mut frameworks = HashSet::new();

    // #import <Framework/...> or #include <Framework/...>
    let import_re = Regex::new(r#"#(?:import|include)\s+<(\w+)/"#).unwrap();
    for cap in import_re.captures_iter(content) {
        let fw = &cap[1];
        if fw != self_framework {
            frameworks.insert(fw.to_string());
        }
    }

    // @import Framework;
    let module_re = Regex::new(r"@import\s+(\w+)\s*;").unwrap();
    for cap in module_re.captures_iter(content) {
        let fw = &cap[1];
        if fw != self_framework {
            frameworks.insert(fw.to_string());
        }
    }

    let mut result: Vec<_> = frameworks.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_import() {
        let content = r#"
            #import <Foundation/Foundation.h>
            #import <CoreGraphics/CoreGraphics.h>
            #import <AppKit/NSView.h>
        "#;
        let deps = extract_framework_imports(content, "AppKit");
        assert!(deps.contains(&"Foundation".to_string()));
        assert!(deps.contains(&"CoreGraphics".to_string()));
        // Should not include self
        assert!(!deps.contains(&"AppKit".to_string()));
    }

    #[test]
    fn test_extract_include() {
        let content = r#"
            #include <CoreFoundation/CoreFoundation.h>
        "#;
        let deps = extract_framework_imports(content, "Foundation");
        assert!(deps.contains(&"CoreFoundation".to_string()));
    }

    #[test]
    fn test_extract_module_import() {
        let content = r#"
            @import Foundation;
            @import CoreGraphics;
        "#;
        let deps = extract_framework_imports(content, "AppKit");
        assert!(deps.contains(&"Foundation".to_string()));
        assert!(deps.contains(&"CoreGraphics".to_string()));
    }
}
