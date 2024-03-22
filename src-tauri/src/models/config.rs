use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u8,
    pub api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            api_key: "".to_string(),
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
