import os
import re
import subprocess
import sys
import itertools
import tempfile
from glob import glob
from textwrap import dedent


def _strip_ifdef_blocks(content, guard_pattern):
    """Strip #if blocks whose condition matches guard_pattern, handling nesting."""
    lines = content.splitlines(keepends=True)
    result = []
    skip_depth = 0  # > 0 means we're inside a block to strip

    for line in lines:
        stripped = line.strip()
        if skip_depth > 0:
            if stripped.startswith('#if'):
                skip_depth += 1
            elif stripped.startswith('#endif'):
                skip_depth -= 1
            continue
        if stripped.startswith('#if') and re.search(guard_pattern, stripped):
            skip_depth = 1
            continue
        result.append(line)

    return ''.join(result)


def test_framework_has_declarations(sdk_path, framework_name):
    """Check if a framework's headers contain actual C/ObjC declarations.

    Some frameworks ship empty stub headers (only comments and include guards)
    on platforms where the API is unavailable (e.g. PushToTalk on macOS).
    Metal shader-only frameworks (#if __METAL_VERSION__) also produce no bindings.
    These should be excluded.
    """
    fw_dir = f"{framework_path(sdk_path)}/{framework_name}.framework"
    if not os.path.isdir(fw_dir):
        return False

    decl_pattern = re.compile(
        r'@interface\s|@protocol\s+\w+\s*[<({]|'
        r'typedef\s|extern\s|'
        r'\b(?:struct|union|enum)\s+\w+\s*\{|'
        r'CF_EXPORT\b|NS_INLINE\b|'
        r'API_AVAILABLE\b'
    )

    for header_file in glob(f"{fw_dir}/**/*.h", recursive=True):
        try:
            content = open(header_file, encoding="utf-8", errors="replace").read()
        except OSError:
            continue

        # Strip content inside __METAL_VERSION__ guards (Metal shader-only code)
        content = _strip_ifdef_blocks(content, r'__METAL_VERSION__')

        for line in content.splitlines():
            stripped = line.strip()
            # Skip preprocessor directives, comments, empty lines
            if not stripped or stripped.startswith(('//', '/*', '*', '#')):
                continue
            if decl_pattern.search(stripped):
                return True

    return False


def test_framework_is_umbrella(sdk_path, framework_name):
    """Check if a framework is an umbrella that imports other non-trivial frameworks.

    Umbrella frameworks (e.g. Cocoa, AudioUnit) have no own declarations but
    serve as convenience features that activate their dependencies.
    Importing only Foundation/CoreFoundation doesn't count — nearly every
    framework does that.
    """
    trivial_imports = frozenset(["Foundation", "CoreFoundation", "os", "simd"])

    fw_dir = f"{framework_path(sdk_path)}/{framework_name}.framework"
    if not os.path.isdir(fw_dir):
        return False

    import_pattern = re.compile(r'#(?:import|include)\s+<(\w+)/')
    for header_file in glob(f"{fw_dir}/**/*.h", recursive=True):
        try:
            content = open(header_file, encoding="utf-8", errors="replace").read()
        except OSError:
            continue
        for match in import_pattern.finditer(content):
            dep = match.group(1)
            if dep != framework_name and dep not in trivial_imports:
                return True

    return False


def test_framework_buildable(sdk_path, framework_name):
    """Test if a framework can be built by trying to compile @import."""
    with tempfile.NamedTemporaryFile(mode='w', suffix='.m', delete=False) as f:
        f.write(f'@import {framework_name};\n')
        f.write('int main() { return 0; }\n')
        temp_file = f.name

    try:
        result = subprocess.run(
            [
                'clang', '-fsyntax-only', '-fmodules',
                '-isysroot', sdk_path,
                temp_file
            ],
            capture_output=True,
            text=True,
            timeout=30
        )
        return result.returncode == 0
    except subprocess.TimeoutExpired:
        return False
    finally:
        os.unlink(temp_file)


SDK_MAP = {
    "MacOSX": "macosx",
    "iPhoneOS": "iphoneos",
    "iPhoneSimulator": "iphonesimulator",
}


def _xcrun_sdk(sdk_name, flag):
    xcrun_name = SDK_MAP.get(sdk_name, sdk_name.lower())
    result = subprocess.run(
        ["xcrun", "--sdk", xcrun_name, flag],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        return None
    return result.stdout.rstrip()


def get_sdk_path(sdk_name):
    """Get SDK path using xcrun."""
    return _xcrun_sdk(sdk_name, "--show-sdk-path")


def get_sdk_version(sdk_name):
    """Get SDK version using xcrun."""
    return _xcrun_sdk(sdk_name, "--show-sdk-version")


def update_prebuilt_versions(sdk_versions):
    """Update prebuilt crate versions based on SDK versions.

    Sets version to 0.{sdk_version} (e.g. SDK 26.2 -> version 0.26.2).
    Updates both the prebuilt Cargo.toml files and the apple-sys dependency versions.
    """
    prebuilt_map = {
        "MacOSX": ("apple-sys-prebuilt-macosx", "../apple-sys-prebuilt-macosx/Cargo.toml"),
        "iPhoneOS": ("apple-sys-prebuilt-iphoneos", "../apple-sys-prebuilt-iphoneos/Cargo.toml"),
    }

    for sdk_name, sdk_version in sdk_versions.items():
        if sdk_name not in prebuilt_map:
            continue
        dep_name, toml_path = prebuilt_map[sdk_name]
        crate_version = f"0.{sdk_version}"

        # Update prebuilt Cargo.toml
        if os.path.isfile(toml_path):
            content = open(toml_path).read()
            content = re.sub(
                r'^version\s*=\s*"[^"]*"',
                f'version = "{crate_version}"',
                content,
                count=1,
                flags=re.MULTILINE,
            )
            with open(toml_path, "w") as f:
                f.write(content)
            print(f"Updated {toml_path} version to {crate_version}")

        # Update apple-sys/Cargo.toml dependency version
        cargo_toml = open("Cargo.toml").read()
        cargo_toml = re.sub(
            rf'({dep_name}\s*=\s*\{{[^}}]*version\s*=\s*")[^"]*(")',
            rf'\g<1>{crate_version}\2',
            cargo_toml,
        )
        with open("Cargo.toml", "w") as f:
            f.write(cargo_toml)
        print(f"Updated {dep_name} dependency to {crate_version}")


def framework_path(sdk_path):
    return f"{sdk_path}/System/Library/Frameworks"


def analyze_deps_clang_m(sdk_path, framework_name, valid_frameworks):
    """Use clang -M preprocessor to detect all transitive framework dependencies.

    Returns a set of framework names, or None if clang -M failed.
    """
    with tempfile.NamedTemporaryFile(mode='w', suffix='.m', delete=False) as f:
        f.write(f'#import <{framework_name}/{framework_name}.h>\n')
        temp_file = f.name

    try:
        result = subprocess.run(
            ['clang', '-M', '-fsyntax-only', '-isysroot', sdk_path, temp_file],
            capture_output=True, text=True, timeout=60
        )
        if result.returncode != 0:
            return None

        deps = set()
        fw_pattern = re.compile(r'(\w+)\.framework/')
        for match in fw_pattern.finditer(result.stdout):
            fw = match.group(1)
            if fw != framework_name and fw in valid_frameworks:
                deps.add(fw)
        return deps
    except subprocess.TimeoutExpired:
        return None
    finally:
        os.unlink(temp_file)


def analyze_deps_headers(sdk_path, framework_name, valid_frameworks):
    """Fallback: Analyze headers to find dependencies via #import/#include/@import patterns."""
    fw_dir = f"{framework_path(sdk_path)}/{framework_name}.framework"
    if not os.path.isdir(fw_dir):
        return set()

    deps = set()
    import_pattern = re.compile(r'#(?:import|include)\s+<(\w+)/')
    module_pattern = re.compile(r'@import\s+(\w+)\s*;')

    for header_file in glob(f"{fw_dir}/**/*.h", recursive=True):
        try:
            content = open(header_file, encoding="utf-8", errors="replace").read()
        except OSError:
            continue

        for match in import_pattern.finditer(content):
            fw = match.group(1)
            if fw != framework_name and fw in valid_frameworks:
                deps.add(fw)

        for match in module_pattern.finditer(content):
            fw = match.group(1)
            if fw != framework_name and fw in valid_frameworks:
                deps.add(fw)

    return deps


def build_class_to_framework_map(sdk_path, framework_names):
    """Build a map of ObjC class/protocol names to their defining framework.

    Scans @interface and @protocol definitions (not forward declarations).
    """
    class_map = {}
    # Match class definitions only, not categories: @interface Foo : Bar, @interface Foo <Proto>, @interface Foo {
    # Excludes categories: @interface Foo (Category)
    interface_pattern = re.compile(r'@interface\s+(\w+)\s*[:<{]')
    protocol_pattern = re.compile(r'@protocol\s+(\w+)\s*[<\n({]')

    for name in framework_names:
        fw_dir = f"{framework_path(sdk_path)}/{name}.framework"
        if not os.path.isdir(fw_dir):
            continue

        for header_file in glob(f"{fw_dir}/**/*.h", recursive=True):
            try:
                content = open(header_file, encoding="utf-8", errors="replace").read()
            except OSError:
                continue

            for match in interface_pattern.finditer(content):
                cls = match.group(1)
                if cls not in class_map:
                    class_map[cls] = name

            for match in protocol_pattern.finditer(content):
                proto = match.group(1)
                if proto not in class_map:
                    class_map[proto] = name

    return class_map


def find_forward_decl_deps(sdk_path, framework_name, class_map):
    """Find additional dependencies from @class/@protocol forward declarations."""
    fw_dir = f"{framework_path(sdk_path)}/{framework_name}.framework"
    if not os.path.isdir(fw_dir):
        return set()

    deps = set()
    # @class Foo, Bar, Baz;  (must be single line, excludes doc comments)
    class_fwd_pattern = re.compile(r'@class\s+([^;\n]+);')
    # @protocol Foo;  (forward declaration ends with ; on same line)
    protocol_fwd_pattern = re.compile(r'@protocol\s+(\w+)\s*;')

    for header_file in glob(f"{fw_dir}/**/*.h", recursive=True):
        try:
            content = open(header_file, encoding="utf-8", errors="replace").read()
        except OSError:
            continue

        for match in class_fwd_pattern.finditer(content):
            classes_str = match.group(1)
            for cls in re.findall(r'\b([A-Z]\w+)\b', classes_str):
                if cls in class_map and class_map[cls] != framework_name:
                    deps.add(class_map[cls])

        for match in protocol_fwd_pattern.finditer(content):
            proto = match.group(1)
            if proto in class_map and class_map[proto] != framework_name:
                deps.add(class_map[proto])

    return deps


def compute_direct_deps(transitive_deps):
    """Transitive reduction: compute minimal direct deps from full transitive deps.

    For each framework A with transitive deps T(A), the direct deps are:
        direct(A) = T(A) - union(T(d) for d in T(A))
    """
    direct = {}
    for fw, trans in transitive_deps.items():
        if not trans:
            continue
        indirect = set()
        for dep in trans:
            if dep in transitive_deps:
                indirect |= transitive_deps[dep]
        result = trans - indirect
        if result:
            direct[fw] = sorted(result)
    return direct


def analyze_deps_from_prebuilt(prebuilt_dir, valid_frameworks):
    """Scan prebuilt .rs files for actual `use crate::XXX::*;` imports.

    Returns a dict of framework_name -> set of dependency framework names.
    This is authoritative because it reflects what the ownership system
    actually generated, which may differ from clang -M header analysis.
    """
    import_re = re.compile(r'^use crate::(\w+)::\*;', re.MULTILINE)
    deps = {}

    for fname in os.listdir(prebuilt_dir):
        if not fname.endswith('.rs'):
            continue
        fw = fname[:-3]
        if fw == 'objc' or fw not in valid_frameworks:
            continue

        filepath = os.path.join(prebuilt_dir, fname)
        try:
            content = open(filepath, encoding='utf-8').read()
        except OSError:
            continue

        imports = set()
        for m in import_re.finditer(content):
            dep = m.group(1)
            if dep != 'objc' and dep != fw and dep in valid_frameworks:
                imports.add(dep)

        if imports:
            deps[fw] = imports

    return deps


def find_framework_names(sdk_path, verify_buildable=True):
    f_path = framework_path(sdk_path)
    pattern = f_path + "/*.framework"

    # Known unbuildable frameworks (skip clang verification for speed)
    if 'MacOSX' in sdk_path:
        blocklist = frozenset(
            [
                # framework not found
                "CoreAudioTypes",
                "CoreMIDIServer",
                "DeviceActivity",
                "DriverKit",
                "Kernel",
                "QTKit",
                "RealityKit",
                "Ruby",
                "SwiftUICore",
                "Tk",
                "vecLib",
            ]
        )
    elif 'iPhone' in sdk_path:
        blocklist = frozenset(
            [
                # framework not found
                "CoreAudioTypes",
                "IOKit",
                "RealityKit",
                "SwiftUICore",
            ]
        )
    else:
        raise ValueError(f"blocklist is not set for: {sdk_path}")

    candidates = []
    for fw_path in glob(pattern):
        name = os.path.basename(fw_path).removesuffix(".framework")
        if name in blocklist:
            continue
        has_header = os.path.isdir(f"{fw_path}/Headers")
        if not has_header:
            continue
        if name.startswith("_"):
            continue
        candidates.append(name)

    if not verify_buildable:
        yield from candidates
        return

    # Verify each framework is actually buildable and has declarations
    print(f"Verifying {len(candidates)} frameworks for {os.path.basename(sdk_path)}...")
    unbuildable = []
    empty = []
    umbrella = []
    for i, name in enumerate(candidates):
        if (i + 1) % 20 == 0:
            print(f"  Progress: {i + 1}/{len(candidates)}")
        if not test_framework_buildable(sdk_path, name):
            unbuildable.append(name)
        elif not test_framework_has_declarations(sdk_path, name):
            if test_framework_is_umbrella(sdk_path, name):
                umbrella.append(name)
                yield name
            else:
                empty.append(name)
        else:
            yield name

    if unbuildable:
        print(f"  Unbuildable frameworks ({len(unbuildable)}): {', '.join(unbuildable)}")
    if empty:
        print(f"  Empty stub frameworks ({len(empty)}): {', '.join(empty)}")
    if umbrella:
        print(f"  Umbrella frameworks ({len(umbrella)}): {', '.join(umbrella)}")


def gen_lib(names: dict[str, list[str]]):
    source = dedent(
        f"""
    //! apple-sys main module
    //! auto-generated by "python {" ".join(map(repr, sys.argv))}"
    #![allow(dead_code)]
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(improper_ctypes)]
    #![allow(non_snake_case)]
    #![allow(ambiguous_glob_reexports)]
    #![allow(unused_imports)]
    #![allow(unsafe_op_in_unsafe_fn)]
    #![allow(unnecessary_transmutes)]
    
    """
    )

    # The "objc" virtual module is always generated by build.rs alongside frameworks.
    # It must be included unconditionally (gated only by target_os) so that
    # `use crate::objc::*;` in framework modules resolves correctly.
    all_os = sorted({target_os(p) for p in names.keys()})
    if len(all_os) == 1:
        source += f'#[cfg(target_os = "{all_os[0]}")] pub mod objc {{ include!(concat!(env!("OUT_DIR"), "/objc.rs")); }}\n'
    else:
        os_list = ", ".join(f'target_os = "{o}"' for o in all_os)
        source += f'#[cfg(any({os_list}))] pub mod objc {{ include!(concat!(env!("OUT_DIR"), "/objc.rs")); }}\n'

    def gen_module(os, name):
        return f"""#[cfg(all(target_os = "{os}", feature = "{name}"))] pub mod {name} {{ include!(concat!(env!("OUT_DIR"), "/{name}.rs"));  }}"""

    for platform, names in names.items():
        os = target_os(platform)
        source += "\n".join(gen_module(os, name) for name in names)
        source += "\n"

    return source


def gen_cargo(names, deps):
    """Generate Cargo.toml with feature dependencies.

    Args:
        names: dict of platform -> list of framework names
        deps: dict of framework_name -> list of dependency framework names
    """
    DELIMITER = "# AUTO-GENERATED: DO NOT ADD ANYTHING BELOW THIS LINE"
    source = open("Cargo.toml", "r").read()
    top, _ = source.split(DELIMITER)

    unique_names = sorted(frozenset(itertools.chain(*names.values())))

    lines = []
    for name in unique_names:
        framework_deps = deps.get(name, [])
        if framework_deps:
            deps_str = ", ".join(f'"{d}"' for d in framework_deps)
            lines.append(f"{name} = [{deps_str}]")
        else:
            lines.append(f"{name} = []")

    bottom = "\n".join(lines)
    return f"""{top}{DELIMITER}\n{bottom}\n"""


def gen_build(names):
    parts = [f'    #[cfg(feature = "{name}")] "{name}",' for name in names]
    body = "\n".join(parts)
    return f'''vec![
{body}
]
'''


def target_os(platform):
    MAP = {
        "MacOSX": "macos",
        "iPhoneOS": "ios",
        "iPhoneSimulator": "ios",
    }
    return MAP[platform]


def main(sdk_names):
    # Get SDK paths using xcrun
    sdk_paths = {}
    for sdk_name in sdk_names:
        path = get_sdk_path(sdk_name)
        if path:
            sdk_paths[sdk_name] = path
        else:
            print(f"Warning: SDK {sdk_name} not found, skipping")

    # Get SDK versions and update prebuilt crate versions
    sdk_versions = {}
    for sdk_name in sdk_paths:
        version = get_sdk_version(sdk_name)
        if version:
            sdk_versions[sdk_name] = version
            print(f"SDK {sdk_name} version: {version}")
    update_prebuilt_versions(sdk_versions)

    framework_names = {
        sdk_name: list(sorted(find_framework_names(sdk_path)))
        for sdk_name, sdk_path in sdk_paths.items()
    }

    # Collect all valid framework names across all SDKs
    all_frameworks = frozenset(itertools.chain(*framework_names.values()))

    # Phase 1: Detect transitive deps via clang -M
    print("Analyzing framework dependencies via clang -M...")
    transitive_deps = {}
    clang_m_failures = []

    for sdk_name, sdk_path in sdk_paths.items():
        names = framework_names.get(sdk_name, [])
        for i, name in enumerate(names):
            if name in transitive_deps:
                continue
            if (i + 1) % 20 == 0:
                print(f"  clang -M progress: {i + 1}/{len(names)} ({sdk_name})")

            deps = analyze_deps_clang_m(sdk_path, name, all_frameworks)
            if deps is not None:
                transitive_deps[name] = deps
            else:
                clang_m_failures.append(name)
                deps = analyze_deps_headers(sdk_path, name, all_frameworks)
                transitive_deps[name] = deps

    if clang_m_failures:
        print(f"  clang -M failed for {len(clang_m_failures)} frameworks (used header scan): {', '.join(clang_m_failures)}")

    # Phase 2: Detect @class/@protocol forward declaration deps
    # Only add if they don't create cycles (dep already depends on us)
    print("Building class-to-framework map for forward declaration analysis...")
    for sdk_name, sdk_path in sdk_paths.items():
        class_map = build_class_to_framework_map(
            sdk_path, framework_names.get(sdk_name, [])
        )
        print(f"  {len(class_map)} classes/protocols mapped ({sdk_name})")

        for name in framework_names.get(sdk_name, []):
            fwd_deps = find_forward_decl_deps(sdk_path, name, class_map)
            if fwd_deps:
                if name not in transitive_deps:
                    transitive_deps[name] = set()
                for dep in fwd_deps:
                    dep_trans = transitive_deps.get(dep, set())
                    if name not in dep_trans:
                        transitive_deps[name].add(dep)
                    else:
                        print(f"  Skipping cycle: {name} -> {dep}")

    # Phase 3: Compute direct deps via transitive reduction
    framework_deps = compute_direct_deps(transitive_deps)
    print(f"  Detected dependencies for {len(framework_deps)} frameworks")

    # Phase 4: If prebuilt .rs files exist, merge their actual imports
    # The ownership system in build.rs may create cross-framework imports
    # that don't appear in the header include chain (e.g. CATransform3D
    # owned by AddressBook but used by QuartzCore). Prebuilt imports are
    # authoritative for Cargo features because they reflect what `use crate::`
    # lines the build.rs actually generates.
    #
    # Scan ALL platform prebuilt dirs and take the union of imports,
    # so that Cargo feature deps work for both macOS and iOS.
    prebuilt_dirs = [
        os.path.join(os.path.dirname(__file__), "..", "apple-sys-prebuilt-macosx", "generated"),
        os.path.join(os.path.dirname(__file__), "..", "apple-sys-prebuilt-iphoneos", "generated"),
    ]
    existing_prebuilt = [d for d in prebuilt_dirs if os.path.isdir(d)]
    if existing_prebuilt:
        print(f"Scanning prebuilt .rs files for actual import deps ({len(existing_prebuilt)} platform(s))...")
        prebuilt_deps = {}
        for prebuilt_dir in existing_prebuilt:
            pb = analyze_deps_from_prebuilt(prebuilt_dir, all_frameworks)
            for fw, deps in pb.items():
                prebuilt_deps.setdefault(fw, set()).update(deps)

        # Merge: for each framework, ensure all prebuilt imports are
        # transitively reachable. Add missing ones as direct deps.
        def _transitive_closure(fw, deps_map, visited=None):
            if visited is None:
                visited = set()
            if fw in visited:
                return set()
            visited.add(fw)
            result = set()
            for dep in deps_map.get(fw, []):
                result.add(dep)
                result |= _transitive_closure(dep, deps_map, visited)
            return result

        added_count = 0
        for fw, pb_imports in sorted(prebuilt_deps.items()):
            reachable = _transitive_closure(fw, framework_deps)
            missing = pb_imports - reachable - {fw}
            if missing:
                existing = list(framework_deps.get(fw, []))
                new_deps = sorted(set(existing) | missing)
                framework_deps[fw] = new_deps
                added_count += len(missing)
                print(f"  {fw}: added prebuilt deps {sorted(missing)}")

        if added_count:
            print(f"  Added {added_count} deps from prebuilt imports")
        else:
            print("  Prebuilt imports consistent with header analysis")

    with open("src/lib.rs", "w") as f:
        content = gen_lib(framework_names)
        f.write(content)

    content = gen_cargo(framework_names, framework_deps)
    with open("Cargo.toml", "w") as f:
        f.write(content)

    for platform, names in framework_names.items():
        target = target_os(platform)
        with open(f"{target}.inc.rs", "w") as f:
            content = gen_build(names)
            f.write(content)

    for platform, names in framework_names.items():
        with open(f"test_script.{platform}.sh", "w") as f:
            target = ""
            command = "test"
            if platform == "iPhoneOS":
                target = "--target aarch64-apple-ios"
                command = "build"
            f.write(
                dedent(
                    f"""
                    names="{' '.join(names)}"
                    for name in $names; do
                        echo $name && cargo {command} {target} --features $name &> test.{platform}.$name.result && rm test.{platform}.$name.result
                    done
                    """
                )
            )

    # Generate deps.toml for build.rs to use (loaded by apple-bindgen via include_str!)
    deps_toml_path = os.path.join(os.path.dirname(__file__), "..", "apple-bindgen", "deps.toml")
    with open(deps_toml_path, "w") as f:
        f.write("# AUTO-GENERATED: Framework dependencies for build.rs\n")
        f.write("# Maps framework name to its direct dependencies\n\n")
        for name in sorted(framework_deps.keys()):
            deps_list = framework_deps[name]
            deps_str = ", ".join(f'"{d}"' for d in deps_list)
            f.write(f'{name} = [{deps_str}]\n')

    for platform, names in framework_names.items():
        print(f"generated {platform}:", ",".join(names))
    print(f"generated deps.toml: {len(framework_deps)} frameworks with dependencies")


if __name__ == "__main__":
    main(["MacOSX", "iPhoneOS"])
    subprocess.run(["cargo", "fmt"])


def test_xcode_select_path():
    assert xcode_select_path().endswith("/Developer")
