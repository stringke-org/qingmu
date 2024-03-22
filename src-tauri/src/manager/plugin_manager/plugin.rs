use std::path::PathBuf;

use deno_core::{JsRuntime, RuntimeOptions};

pub enum PluginStatus {
    Installed,
    Uninstalled,
    Updated,
    Outdated,
}

pub struct Plugin {
    pub info: PluginInfo,

    _entry: PathBuf,
    _runtime: JsRuntime,
}

pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub repository: String,
    pub homepage: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub permissions: Vec<String>,
}

impl Plugin {
    pub async fn new(package_name: &str, version: &str) -> Self {
        let runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![],
            ..Default::default()
        });

        Self {
            info,
            _runtime: runtime,
            _entry: PathBuf::from(""),
        }
    }

    pub async fn restart(&self) {
        self.stop().await;
        self.start().await;
    }

    pub async fn stop(&self) {
        // Stop the plugin
    }

    pub async fn start(&mut self) {
        // Start the plugin
        let entry = self._entry.clone();
        self._runtime.execute_script("plugin.js", &entry)
    }

    pub async fn uninstall(&self) {
        // Uninstall the plugin
    }

    pub async fn install(&self) {
        // Install the plugin
    }

    pub async fn update(&self) {
        // Update the plugin
    }

    pub async fn info(&self) {
        // Get the plugin info
    }
}
