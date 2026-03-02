pub use crate::{
    config::{Config, ConfigMap},
    sdk::{SdkPath, SdkPathError},
};

#[derive(Debug)]
pub struct Builder {
    framework: String,
    sdk: SdkPath,
    target: Option<String>,
    config: Config,
}

impl Builder {
    pub fn new(
        framework: &str,
        sdk: impl TryInto<SdkPath, Error = SdkPathError>,
        config: Config,
    ) -> Result<Self, SdkPathError> {
        Ok(Self {
            framework: framework.to_owned(),
            sdk: sdk.try_into()?,
            target: None,
            config,
        })
    }

    pub fn with_builtin_config(
        framework: &str,
        sdk: impl TryInto<SdkPath, Error = SdkPathError>,
    ) -> Result<Self, SdkPathError> {
        Self::new(
            framework,
            sdk,
            ConfigMap::with_builtin_config().framework_config(framework),
        )
    }

    pub fn target(mut self, target: impl AsRef<str>) -> Self {
        assert!(self.target.is_none());
        self.target = Some(target.as_ref().to_owned());
        self
    }

    pub fn bindgen_builder(&self) -> bindgen::Builder {
        // Begin building the bindgen params.
        let mut builder = bindgen::Builder::default();

        let mut clang_args = vec!["-x", "objective-c", "-fblocks", "-fmodules"];
        let target_arg;
        if let Some(target) = self.target.as_ref() {
            target_arg = format!("--target={}", target);
            clang_args.push(&target_arg);
        }

        clang_args.extend(&[
            "-isysroot",
            self.sdk
                .path()
                .to_str()
                .expect("sdk path is not utf-8 representable"),
        ]);

        builder = builder
            .clang_args(&clang_args)
            .layout_tests(self.config.layout_tests)
            .formatter(bindgen::Formatter::Prettyplease);

        for opaque_type in &self.config.opaque_types {
            builder = builder.opaque_type(opaque_type);
        }
        for blocklist_item in &self.config.blocklist_items {
            builder = builder.blocklist_item(blocklist_item);
        }

        builder = builder.header_contents(
            &format!("{}.h", self.framework),
            &format!("@import {};", self.framework),
        );

        // Only generate bindings for items defined in framework headers, the ObjC runtime,
        // and MacTypes.h. This excludes irrelevant system types (arm_debug_state32_t, etc.)
        // from non-framework paths like <mach/>, <sys/>, <arm/>.
        // allowlist_recursively (default true) ensures types referenced by framework APIs
        // from system headers (e.g. simd types) are still included.
        builder = builder
            .allowlist_file(".*\\.framework/.*")
            .allowlist_file(".*/usr/include/objc/.*")
            .allowlist_file(".*/usr/include/os/.*")
            .allowlist_file(".*/usr/include/MacTypes\\.h");

        builder
    }

    pub fn generate(&self) -> Result<String, bindgen::BindgenError> {
        let bindgen_builder = self.bindgen_builder();

        // Generate the bindings.
        let bindings = bindgen_builder.generate()?;

        // TODO: find the best way to do this post-processing
        let mut out = bindings.to_string();

        // remove redundant and malformed definitions of `id`
        out = out.replace("pub type id = *mut objc::runtime::Object", "PUB-TYPE-ID");
        let re = regex::Regex::new("pub type id = .*;").unwrap();
        out = re.replace_all(&mut out, "").into_owned();
        out = out.replace("PUB-TYPE-ID", "pub type id = *mut objc::runtime::Object");

        // Bindgen.toml `replacements`
        for replacement in &self.config.replacements {
            let (old, new) = replacement
                .split_once(" #=># ")
                .expect("Bindgen.toml is malformed");
            out = out.replace(old, new);
        }

        // Fix msg_send! arguments that collide with struct type names.
        // e.g. `msg_send!(*self, setPDFView: PDFView)` where `PDFView`
        // resolves to the struct constructor instead of the parameter.
        out = fix_msg_send_type_collisions(&out);

        // Bindgen.toml `impl_debugs`
        for ty in &self.config.impl_debugs {
            if out.contains(ty) {
                out.push_str(&format!(
                    r#"
impl ::std::fmt::Debug for {ty} {{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
        f.debug_struct(stringify!(#ty))
            .finish()
    }}
}}
                "#
                ));
            }
        }

        Ok(out)
    }
}

/// Rename msg_send! arguments and fn parameters that collide with top-level item names.
///
/// Bindgen may generate methods where a parameter name matches a `pub struct` or
/// `pub const` defined in the same output. In `msg_send!` macro expansion, these
/// names resolve to the struct constructor or constant value instead of the local
/// parameter. This appends `_` to colliding names in both parameter declarations
/// and `msg_send!` calls.
fn fix_msg_send_type_collisions(source: &str) -> String {
    use std::collections::HashSet;

    let mut shadow_names: HashSet<&str> = HashSet::new();
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
                    shadow_names.insert(name);
                }
            }
        }
    }
    if shadow_names.is_empty() {
        return source.to_string();
    }

    let msg_arg_re = regex::Regex::new(r" : (\w+)").unwrap();
    let comma_param_re = regex::Regex::new(r", (\w+): ").unwrap();
    let paren_param_re = regex::Regex::new(r"\((\w+): ").unwrap();

    let mut in_trait = false;
    let mut trait_brace_depth: i32 = 0;
    let mut in_msg_send = false;
    let mut msg_send_depth: i32 = 0;

    let mut result = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed = line.trim();

        // Track ObjC trait blocks to restrict parameter renames
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

        let mut fixed = line.to_string();
        let mut did_fix = false;

        // msg_send! arguments: ` : name` where name shadows a top-level item
        // Also handle continuation lines of multi-line msg_send! blocks
        if fixed.contains("msg_send") || is_in_msg_send {
            let new_fixed = msg_arg_re.replace_all(&fixed, |caps: &regex::Captures| {
                let name = caps.get(1).unwrap().as_str();
                if shadow_names.contains(name) {
                    format!(" : {}_", name)
                } else {
                    caps[0].to_string()
                }
            });
            if new_fixed != fixed {
                fixed = new_fixed.into_owned();
                did_fix = true;
            }
        }

        // Parameter renames only inside trait blocks (extern/free fn params don't collide)
        if in_trait {
            // Indented parameter: `        name: Type` (skip fn-definition lines)
            if !trimmed.starts_with("unsafe fn ") && fixed.starts_with("        ") {
                let after_indent = &fixed[8..];
                if let Some(colon_pos) = after_indent.find(": ") {
                    let candidate = &after_indent[..colon_pos];
                    if !candidate.is_empty()
                        && candidate.chars().all(|c| c.is_alphanumeric() || c == '_')
                        && shadow_names.contains(candidate)
                    {
                        fixed = fixed.replacen(
                            &format!("        {}: ", candidate),
                            &format!("        {}_: ", candidate),
                            1,
                        );
                        did_fix = true;
                    }
                }
            }

            // Inline parameter: `, name: Type`
            {
                let orig = fixed.clone();
                let new_fixed = comma_param_re.replace_all(&orig, |caps: &regex::Captures| {
                    let name = caps.get(1).unwrap().as_str();
                    if shadow_names.contains(name) {
                        format!(", {}_: ", name)
                    } else {
                        caps[0].to_string()
                    }
                });
                if new_fixed != orig {
                    fixed = new_fixed.into_owned();
                    did_fix = true;
                }
            }

            // Opening-paren parameter: `(name: Type`
            {
                let orig = fixed.clone();
                let new_fixed = paren_param_re.replace_all(&orig, |caps: &regex::Captures| {
                    let name = caps.get(1).unwrap().as_str();
                    if shadow_names.contains(name) {
                        format!("({}_: ", name)
                    } else {
                        caps[0].to_string()
                    }
                });
                if new_fixed != orig {
                    fixed = new_fixed.into_owned();
                    did_fix = true;
                }
            }
        }

        if did_fix {
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}
