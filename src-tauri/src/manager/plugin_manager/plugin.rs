use std::path::PathBuf;

pub enum PluginStatus {
    Installed,
    Uninstalled,
    Updated,
    Outdated,
}

pub struct Plugin {
    pub info: PluginInfo,

    _entry: PathBuf,
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

impl Plugin {}
