pub use crate::{
    config::{Config, ConfigMap, FileConfig},
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
            .rustfmt_bindings(true);

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
