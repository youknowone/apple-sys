use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Deserialize)]
pub struct ConfigMap {
    #[serde(flatten)]
    map: HashMap<String, FileConfig>,
}

impl ConfigMap {
    pub fn from_str(content: &str) -> Result<Self, ConfigError> {
        toml::from_str(&content).map_err(|e| ConfigError::TomlError(e))
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(|e| ConfigError::OpenError(e))?;
        Self::from_str(&content)
    }

    pub fn with_builtin_config() -> Self {
        let content = include_str!("../Bindgen.toml");
        Self::from_str(content).expect("Builtin config file is corrupted. This is bug.")
    }

    pub fn framework_config(&self, name: &str) -> Config {
        let mut deps = Vec::new();
        if self.map.contains_key(name) {
            deps.push(name);
        }

        let mut i = 0;
        while i < deps.len() {
            let dep = deps[i];
            let config = self
                .map
                .get(dep)
                .unwrap_or_else(|| panic!("wrong dependency name {dep}"));
            for dep in &config.deps {
                if deps.contains(&dep.as_str()) {
                    continue;
                }
                deps.push(dep.as_str());
            }
            i += 1;
        }
        deps.push("default");
        {
            let mut cloned = deps.clone();
            cloned.sort_unstable();
            cloned.dedup();
            assert_eq!(deps.len(), cloned.len());
        }

        deps.into_iter()
            .map(|dep| &self.map.get(dep).unwrap().config)
            .collect()
    }
}

impl<'a> FromIterator<&'a Config> for Config {
    fn from_iter<T: IntoIterator<Item = &'a Config>>(iter: T) -> Self {
        let mut opaque_types = Vec::new();
        let mut blocklist_items = Vec::new();
        let mut replacements = Vec::new();
        let mut layout_tests = false;
        for config in iter {
            opaque_types.extend(config.opaque_types.iter().cloned());
            blocklist_items.extend(config.blocklist_items.iter().cloned());
            replacements.extend(config.replacements.iter().cloned());
            layout_tests |= config.layout_tests;
        }
        Self {
            opaque_types,
            blocklist_items,
            replacements,
            layout_tests,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct FileConfig {
    #[serde(default)]
    deps: Vec<String>,
    #[serde(flatten)]
    config: Config,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub opaque_types: Vec<String>,
    #[serde(default)]
    pub blocklist_items: Vec<String>,
    #[serde(default)]
    pub replacements: Vec<String>,
    #[serde(default)]
    pub layout_tests: bool,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("toml parsing error {0}")]
    OpenError(std::io::Error),
    #[error("toml parsing error {0}")]
    TomlError(toml::de::Error),
}
