use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u8,
    pub npm_registry: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            npm_registry: "https://registry.npmjs.org/".to_string(),
        }
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Self {
        confy::load("qingmu", "config").unwrap_or_else(|_| AppConfig::new())
    }

    pub fn path(&self) -> Option<String> {
        return match confy::get_configuration_file_path("qingmu", "config") {
            Ok(path) => Some(path.to_str().unwrap().to_string()),
            Err(_) => None,
        };
    }

    pub fn save(&self) {
        confy::store("qingmu", "config", self).unwrap();
    }
}
