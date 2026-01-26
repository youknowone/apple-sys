#!/bin/bash
set -euo pipefail

# Verify that key public API symbols exist in the correct framework caches.
#
# Usage: bash scripts/check-key-symbols.sh [cache-dir]

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CACHE_DIR="${1:-$(ls -d "$REPO_ROOT/crates/apple-bindgen/symbol-cache"/MacOSX* 2>/dev/null | head -1)}"

if [ -z "$CACHE_DIR" ] || [ ! -d "$CACHE_DIR" ]; then
    echo "ERROR: No symbol cache directory found"
    echo "Usage: $0 [cache-dir]"
    exit 1
fi

echo "Checking key symbols in: $CACHE_DIR"
echo ""

failures=0
checks=0

check_symbol() {
    local fw="$1"
    local sym="$2"
    local toml="$CACHE_DIR/${fw}.toml"
    checks=$((checks + 1))

    if [ ! -f "$toml" ]; then
        echo "  FAIL: $fw.toml not found (expected symbol: $sym)"
        failures=$((failures + 1))
        return
    fi

    if grep -q "\"$sym\"" "$toml"; then
        echo "  OK: $fw :: $sym"
    else
        echo "  FAIL: $fw :: $sym (not found)"
        failures=$((failures + 1))
    fi
}

# Check that a symbol exists in ANY framework cache
check_symbol_exists() {
    local sym="$1"
    local expected_fw="$2"  # hint for display
    checks=$((checks + 1))

    local found_in
    found_in=$(grep -rl "\"$sym\"" "$CACHE_DIR"/*.toml 2>/dev/null | head -1)
    if [ -n "$found_in" ]; then
        local actual_fw
        actual_fw=$(basename "$found_in" .toml)
        if [ "$actual_fw" = "$expected_fw" ]; then
            echo "  OK: $actual_fw :: $sym"
        else
            echo "  OK: $actual_fw :: $sym (expected $expected_fw, acceptable)"
        fi
    else
        echo "  FAIL: $sym not found in any framework"
        failures=$((failures + 1))
    fi
}

echo "=== CoreFoundation ==="
check_symbol CoreFoundation CFStringRef
check_symbol CoreFoundation CFArrayRef
check_symbol CoreFoundation CFDictionaryRef
check_symbol CoreFoundation CFRetain
check_symbol CoreFoundation CFRelease
check_symbol CoreFoundation CFTypeRef
check_symbol_exists NSObject objc

echo ""
echo "=== Foundation ==="
check_symbol Foundation NSString
check_symbol Foundation NSArray
check_symbol Foundation NSDictionary
check_symbol Foundation NSUserName
check_symbol Foundation NSFullUserName
check_symbol Foundation NSHomeDirectory
check_symbol Foundation NSTemporaryDirectory
check_symbol Foundation NSPageSize

echo ""
echo "=== CoreGraphics ==="
# CGFloat/CGPoint/CGRect/CGSize are defined in CoreFoundation headers (NSGeometry/CGBase)
# and may be assigned there depending on topological ordering
check_symbol_exists CGFloat CoreGraphics
check_symbol_exists CGPoint CoreGraphics
check_symbol_exists CGRect CoreGraphics
check_symbol_exists CGSize CoreGraphics
check_symbol CoreGraphics CGPathRef

echo ""
echo "=== Metal ==="
check_symbol Metal MTLCreateSystemDefaultDevice
check_symbol Metal MTLCopyAllDevices
check_symbol Metal MTLTextureDescriptor
check_symbol Metal MTLRenderPipelineDescriptor
check_symbol Metal MTLSize
check_symbol Metal MTLClearColor
check_symbol Metal MTLSamplePosition

echo ""
echo "=== ColorSync ==="
check_symbol ColorSync ColorSyncProfileRef
check_symbol ColorSync ColorSyncTransformRef
check_symbol ColorSync ColorSyncMutableProfileRef

echo ""
echo "=== ImageIO ==="
check_symbol ImageIO CGImageSourceRef
check_symbol ImageIO CGImageSource
check_symbol ImageIO CGImageDestination
check_symbol ImageIO CGImageDestinationRef

echo ""
echo "=== CoreText ==="
check_symbol CoreText CTFontRef
check_symbol CoreText CTFontDescriptorRef
check_symbol CoreText CTLineRef

echo ""
echo "=== AppKit ==="
check_symbol AppKit NSWindow
check_symbol AppKit NSView
check_symbol AppKit NSApplication
check_symbol AppKit NSViewController

echo ""
echo "=== Security ==="
check_symbol Security SecKeyRef
# SecCertificateRef may be in AuthenticationServices due to header declarations
check_symbol_exists SecCertificateRef Security

echo ""
echo "=== CoreServices ==="
# AEDesc may be in Foundation due to header declarations
check_symbol_exists AEDesc CoreServices
check_symbol_exists FSEventStreamRef CoreServices

echo ""
echo "=== IOKit ==="
check_symbol IOKit io_object_t
check_symbol IOKit io_service_t
check_symbol IOKit IOReturn

echo ""
echo "=== QuartzCore ==="
check_symbol QuartzCore CALayer

echo ""
echo "========================="
echo "Total checks: $checks"
if [ "$failures" -eq 0 ]; then
    echo "OK: All key symbols present"
    exit 0
else
    echo "FAIL: $failures symbol(s) missing"
    exit 1
fi
