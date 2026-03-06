#!/bin/bash
set -euo pipefail

# Check for duplicate symbols across framework symbol caches.
# Excludes bindgen_ty_* anonymous types which are expected to overlap.
#
# Usage: bash scripts/check-symbol-duplicates.sh [cache-dir]

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CACHE_DIR="${1:-$(ls -d "$REPO_ROOT/crates/apple-bindgen/symbol-cache"/MacOSX* 2>/dev/null | head -1)}"

if [ -z "$CACHE_DIR" ] || [ ! -d "$CACHE_DIR" ]; then
    echo "ERROR: No symbol cache directory found"
    echo "Usage: $0 [cache-dir]"
    exit 1
fi

echo "Checking symbol duplicates in: $CACHE_DIR"

python3 -c "
import os, sys, re

cache_dir = '$CACHE_DIR'
sym_map = {}  # symbol -> list of frameworks
total_files = 0
total_symbols = 0

skip_prefixes = ('_bindgen_ty_', '__BindgenBitfieldUnit', '__BindgenComplex',
                 '__BindgenFloat16', '__IncompleteArrayField')

for filename in sorted(os.listdir(cache_dir)):
    if not filename.endswith('.toml'):
        continue
    fw = filename[:-5]
    total_files += 1
    filepath = os.path.join(cache_dir, filename)
    in_symbols = False
    with open(filepath) as f:
        for line in f:
            line = line.strip()
            if line == 'symbols = [':
                in_symbols = True
                continue
            if in_symbols and line == ']':
                in_symbols = False
                continue
            if in_symbols:
                m = re.match(r'\"(.+)\"', line.strip().rstrip(','))
                if m:
                    sym = m.group(1)
                    if any(sym.startswith(p) for p in skip_prefixes):
                        continue
                    total_symbols += 1
                    sym_map.setdefault(sym, []).append(fw)

print(f'Scanned {total_files} frameworks, {total_symbols} total symbol entries')
print()

dup_count = 0
for sym in sorted(sym_map):
    if len(sym_map[sym]) > 1:
        dup_count += 1
        fws = ', '.join(sym_map[sym])
        print(f'DUPLICATE: {sym}')
        print(f'  Frameworks: {fws}')

print()
if dup_count == 0:
    print(f'OK: No duplicate symbols found across {total_files} frameworks')
else:
    print(f'FAIL: {dup_count} duplicate symbols found')
    sys.exit(1)
"
